import json
from collections.abc import AsyncIterator
from pathlib import Path

import httpx
import pytest

from timetable_chat.assignments import AssignmentWorkbookClient
from timetable_chat.models import UiMessage
from timetable_chat.openrouter import (
    MessageFinished,
    OpenRouterAgent,
    TextDelta,
    ToolArgumentsDelta,
    ToolArgumentsFinished,
    ToolCallStarted,
    ToolResult,
)
from timetable_chat.preferences import PreferenceStore
from timetable_chat.semester import SemesterRepository
from timetable_chat.sessions import SessionLog
from timetable_chat.tools import ToolService


class FragmentedStream(httpx.AsyncByteStream):
    def __init__(self, fragments: list[bytes]) -> None:
        self.fragments = fragments

    async def __aiter__(self) -> AsyncIterator[bytes]:
        for fragment in self.fragments:
            yield fragment


def sse_fragments(events: list[dict[str, object]]) -> list[bytes]:
    wire = (
        b"".join(
            f"data: {json.dumps(event, separators=(',', ':'))}\n\n".encode() for event in events
        )
        + b"data: [DONE]\n\n"
    )
    cut_points = [1, 17, 43, 89, 144, len(wire)]
    return [
        wire[start:end]
        for start, end in zip([0, *cut_points], cut_points, strict=False)
        if start < end
    ]


@pytest.mark.asyncio
async def test_agent_streams_fragmented_text_and_tool_rounds(
    spring_repository: SemesterRepository,
    assignment_client: AssignmentWorkbookClient,
    runtime_directory: Path,
) -> None:
    request_count = 0

    def provider(request: httpx.Request) -> httpx.Response:
        nonlocal request_count
        request_count += 1
        payload = json.loads(request.content)
        assert payload["stream"] is True
        assert payload["stream_options"] == {"include_usage": True}
        if request_count == 1:
            assert "parallel_tool_calls" not in payload
            assert payload["provider"] == {"sort": "throughput"}
            events: list[dict[str, object]] = [
                {
                    "id": "generation-1",
                    "object": "chat.completion.chunk",
                    "choices": [
                        {"index": 0, "delta": {"role": "assistant"}, "finish_reason": None}
                    ],
                },
                {
                    "id": "generation-1",
                    "choices": [
                        {
                            "index": 0,
                            "delta": {
                                "reasoning": "I should retrieve the faculty history.",
                                "reasoning_details": [
                                    {
                                        "type": "reasoning.text",
                                        "text": "I should retrieve the faculty history.",
                                        "format": "unknown",
                                        "id": "reasoning-1",
                                    }
                                ],
                            },
                            "finish_reason": None,
                        }
                    ],
                },
                {
                    "id": "generation-1",
                    "choices": [
                        {
                            "index": 0,
                            "delta": {
                                "tool_calls": [
                                    {
                                        "index": 0,
                                        "id": "call-1",
                                        "type": "function",
                                        "function": {
                                            "name": "load_faculty_workspace",
                                            "arguments": "",
                                        },
                                    }
                                ]
                            },
                            "finish_reason": None,
                        }
                    ],
                },
                {
                    "id": "generation-1",
                    "choices": [
                        {
                            "index": 0,
                            "delta": {
                                "tool_calls": [
                                    {
                                        "index": 0,
                                        "function": {"arguments": '{"faculty_name":"Bart '},
                                    }
                                ]
                            },
                            "finish_reason": None,
                        }
                    ],
                },
                {
                    "id": "generation-1",
                    "choices": [
                        {
                            "index": 0,
                            "delta": {
                                "tool_calls": [
                                    {
                                        "index": 0,
                                        "function": {"arguments": 'Stander"}'},
                                    }
                                ]
                            },
                            "finish_reason": "tool_calls",
                        }
                    ],
                },
            ]
            return httpx.Response(200, stream=FragmentedStream(sse_fragments(events)))

        tool_message = payload["messages"][-1]
        assistant_message = payload["messages"][-2]
        assert "reasoning" not in assistant_message
        assert assistant_message["reasoning_details"] == [
            {
                "type": "reasoning.text",
                "text": "I should retrieve the faculty history.",
                "format": "unknown",
                "id": "reasoning-1",
            }
        ]
        assert tool_message["role"] == "tool"
        assert '"history"' in tool_message["content"]
        final_events: list[dict[str, object]] = [
            {
                "id": "generation-2",
                "choices": [
                    {
                        "index": 0,
                        "delta": {"content": "I found your "},
                        "finish_reason": None,
                    }
                ],
            },
            {
                "id": "generation-2",
                "choices": [
                    {
                        "index": 0,
                        "delta": {"content": "Spring 2026 starting point."},
                        "finish_reason": "stop",
                    }
                ],
            },
            {
                "id": "generation-2",
                "choices": [],
                "usage": {
                    "prompt_tokens": 100,
                    "completion_tokens": 10,
                    "total_tokens": 110,
                },
            },
        ]
        return httpx.Response(200, stream=FragmentedStream(sse_fragments(final_events)))

    client = httpx.AsyncClient(transport=httpx.MockTransport(provider))
    tools = ToolService(
        spring_repository,
        PreferenceStore(runtime_directory / "preferences", spring_repository),
        assignment_client,
    )
    agent = OpenRouterAgent(
        api_key="test-key",
        model="test/model",
        base_url="https://openrouter.test/api/v1",
        max_tool_rounds=3,
        tools=tools,
        sessions=SessionLog(runtime_directory / "sessions"),
        client=client,
    )

    streamed = [
        event
        async for event in agent.stream(
            system_prompt="Test system prompt",
            ui_messages=[UiMessage(role="user", content="I'm Bart Stander")],
            session_id="session-1",
        )
    ]
    await client.aclose()

    assert streamed[0] == ToolCallStarted("call-1", "load_faculty_workspace")
    assert streamed[1:3] == [
        ToolArgumentsDelta("call-1", '{"faculty_name":"Bart '),
        ToolArgumentsDelta("call-1", 'Stander"}'),
    ]
    assert streamed[3] == ToolArgumentsFinished("call-1")
    assert isinstance(streamed[4], ToolResult)
    assert streamed[4].is_error is False
    assert streamed[5:] == [
        TextDelta("I found your "),
        TextDelta("Spring 2026 starting point."),
        MessageFinished(input_tokens=100, output_tokens=10),
    ]
    assert request_count == 2
    events = [
        json.loads(line)
        for line in (runtime_directory / "sessions" / "session-1.jsonl")
        .read_text(encoding="utf-8")
        .splitlines()
    ]
    assert [event["event"] for event in events] == ["request", "tool", "assistant"]

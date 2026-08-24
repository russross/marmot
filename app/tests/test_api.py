import httpx
import pytest
from pytest import MonkeyPatch

from timetable_chat.config import get_settings
from timetable_chat.main import agent_event_frame, app, data_stream_frame
from timetable_chat.openrouter import (
    MessageFinished,
    TextDelta,
    ToolArgumentsDelta,
    ToolArgumentsFinished,
    ToolCallStarted,
    ToolResult,
)


@pytest.mark.asyncio
async def test_app_loads_installed_snapshot_and_reports_missing_provider_key(
    monkeypatch: MonkeyPatch,
) -> None:
    monkeypatch.setenv("OPENROUTER_API_KEY", "")
    get_settings.cache_clear()
    transport = httpx.ASGITransport(app=app)
    try:
        async with (
            app.router.lifespan_context(app),
            httpx.AsyncClient(transport=transport, base_url="http://test") as client,
        ):
            health = await client.get("/api/health")
            chat = await client.post(
                "/api/chat",
                json={
                    "messages": [
                        {
                            "role": "user",
                            "content": [{"type": "text", "text": "Hello"}],
                        }
                    ],
                    "system": None,
                    "tools": {},
                },
            )
    finally:
        get_settings.cache_clear()

    assert health.json() == {"status": "ok", "term": "Fall 2026", "faculty_count": 18}
    assert chat.status_code == 503
    assert chat.json()["detail"] == "OPENROUTER_API_KEY is not configured"


def test_data_stream_frames_preserve_unicode_and_protocol_shape() -> None:
    assert data_stream_frame("0", "MWF 3\u00d750") == b'0:"MWF 3\xc3\x9750"\n'


def test_agent_events_encode_as_assistant_ui_data_stream() -> None:
    assert agent_event_frame(TextDelta("Hello ")) == b'0:"Hello "\n'
    assert agent_event_frame(ToolCallStarted("call-1", "get_faculty_context")) == (
        b'b:{"toolCallId":"call-1","toolName":"get_faculty_context"}\n'
    )
    assert agent_event_frame(ToolArgumentsDelta("call-1", '{"faculty_name":')) == (
        b'c:{"toolCallId":"call-1","argsTextDelta":"{\\"faculty_name\\":"}\n'
    )
    assert agent_event_frame(ToolArgumentsFinished("call-1")) == (
        b'c:{"toolCallId":"call-1","argsTextDelta":"","isFinal":true}\n'
    )
    assert agent_event_frame(ToolResult("call-1", {"ok": True}, False)) == (
        b'a:{"toolCallId":"call-1","result":{"ok":true}}\n'
    )
    assert agent_event_frame(MessageFinished(12, 3)) == (
        b'd:{"finishReason":"stop","usage":{"inputTokens":12,"outputTokens":3}}\n'
    )

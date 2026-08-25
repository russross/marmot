from __future__ import annotations

import json
from collections.abc import AsyncIterator, Sequence
from dataclasses import dataclass

import httpx
from pydantic import JsonValue, TypeAdapter, ValidationError

from timetable_chat.models import (
    OpenRouterAssistantMessage,
    OpenRouterStreamChunk,
    OpenRouterToolCall,
    OpenRouterToolFunction,
    UiMessage,
)
from timetable_chat.sessions import SessionLog
from timetable_chat.tools import ToolService

OPENROUTER_STREAM_CHUNK_ADAPTER = TypeAdapter(OpenRouterStreamChunk)
JSON_VALUE_ADAPTER = TypeAdapter(JsonValue)


class OpenRouterError(RuntimeError):
    pass


@dataclass(frozen=True)
class TextDelta:
    text: str


@dataclass(frozen=True)
class ToolCallStarted:
    tool_call_id: str
    tool_name: str


@dataclass(frozen=True)
class ToolArgumentsDelta:
    tool_call_id: str
    text: str


@dataclass(frozen=True)
class ToolArgumentsFinished:
    tool_call_id: str


@dataclass(frozen=True)
class ToolResult:
    tool_call_id: str
    result: JsonValue
    is_error: bool


@dataclass(frozen=True)
class MessageFinished:
    input_tokens: int
    output_tokens: int


AgentEvent = (
    TextDelta
    | ToolCallStarted
    | ToolArgumentsDelta
    | ToolArgumentsFinished
    | ToolResult
    | MessageFinished
)


@dataclass
class _ToolCallBuffer:
    index: int
    tool_call_id: str = ""
    tool_name: str = ""
    arguments: str = ""
    announced: bool = False

    def complete(self) -> OpenRouterToolCall:
        if not self.tool_call_id or not self.tool_name:
            raise OpenRouterError("OpenRouter returned an incomplete tool call")
        return OpenRouterToolCall(
            id=self.tool_call_id,
            index=self.index,
            type="function",
            function=OpenRouterToolFunction(
                name=self.tool_name,
                arguments=self.arguments or "{}",
            ),
        )


class OpenRouterAgent:
    def __init__(
        self,
        *,
        api_key: str,
        model: str,
        base_url: str,
        max_tool_rounds: int,
        tools: ToolService,
        sessions: SessionLog,
        client: httpx.AsyncClient | None = None,
    ) -> None:
        self.api_key = api_key
        self.model = model
        self.base_url = base_url.rstrip("/")
        self.max_tool_rounds = max_tool_rounds
        self.tools = tools
        self.sessions = sessions
        self.client = client or httpx.AsyncClient(timeout=httpx.Timeout(120.0, connect=15.0))
        self._owns_client = client is None

    async def close(self) -> None:
        if self._owns_client:
            await self.client.aclose()

    async def stream(
        self,
        *,
        system_prompt: str,
        ui_messages: Sequence[UiMessage],
        session_id: str,
    ) -> AsyncIterator[AgentEvent]:
        messages: list[dict[str, JsonValue]] = [{"role": "system", "content": system_prompt}]
        messages.extend(
            {"role": message.role, "content": text}
            for message in ui_messages
            if message.role != "tool" and (text := message.text())
        )
        self.sessions.append(
            session_id,
            "request",
            {"model": self.model, "messages": messages[1:]},
        )

        total_input_tokens = 0
        total_output_tokens = 0
        for round_number in range(self.max_tool_rounds + 1):
            text_parts: list[str] = []
            reasoning_parts: list[str] = []
            reasoning_details: list[JsonValue] = []
            tool_buffers: dict[int, _ToolCallBuffer] = {}
            finish_reason: str | None = None

            async for chunk in self._completion_chunks(messages):
                if chunk.usage is not None:
                    total_input_tokens += chunk.usage.prompt_tokens
                    total_output_tokens += chunk.usage.completion_tokens
                if not chunk.choices:
                    continue
                if len(chunk.choices) != 1:
                    raise OpenRouterError(
                        f"OpenRouter returned {len(chunk.choices)} choices; expected exactly one"
                    )
                choice = chunk.choices[0]
                finish_reason = choice.finish_reason or finish_reason
                delta = choice.delta
                if delta.reasoning:
                    reasoning_parts.append(delta.reasoning)
                if delta.reasoning_details:
                    reasoning_details.extend(delta.reasoning_details)
                if delta.content:
                    text_parts.append(delta.content)
                    yield TextDelta(delta.content)
                for partial_call in delta.tool_calls or []:
                    buffer = tool_buffers.setdefault(
                        partial_call.index,
                        _ToolCallBuffer(index=partial_call.index),
                    )
                    if partial_call.id:
                        buffer.tool_call_id = partial_call.id
                    if partial_call.function is not None:
                        if partial_call.function.name:
                            buffer.tool_name += partial_call.function.name
                        if partial_call.function.arguments:
                            buffer.arguments += partial_call.function.arguments
                    if buffer.tool_call_id and buffer.tool_name and not buffer.announced:
                        buffer.announced = True
                        yield ToolCallStarted(buffer.tool_call_id, buffer.tool_name)
                    if partial_call.function is not None and partial_call.function.arguments:
                        if not buffer.announced:
                            raise OpenRouterError(
                                "OpenRouter streamed tool arguments before identifying the tool"
                            )
                        yield ToolArgumentsDelta(
                            buffer.tool_call_id,
                            partial_call.function.arguments,
                        )

            content = "".join(text_parts) or None
            tool_calls = [tool_buffers[index].complete() for index in sorted(tool_buffers)]
            assistant = OpenRouterAssistantMessage(
                role="assistant",
                content=content,
                tool_calls=tool_calls or None,
                reasoning=None if reasoning_details else "".join(reasoning_parts) or None,
                reasoning_details=reasoning_details or None,
            )
            messages.append(assistant.model_dump(mode="json", exclude_none=True))
            if not tool_calls:
                if content is None:
                    raise OpenRouterError("OpenRouter returned neither text nor tool calls")
                self.sessions.append(session_id, "assistant", content)
                yield MessageFinished(total_input_tokens, total_output_tokens)
                return
            if finish_reason not in {None, "tool_calls"}:
                raise OpenRouterError(
                    f"OpenRouter returned tool calls with finish reason {finish_reason!r}"
                )
            if round_number == self.max_tool_rounds:
                raise OpenRouterError("model exceeded the configured tool-call round limit")

            for tool_call in tool_calls:
                yield ToolArgumentsFinished(tool_call.id)
                result_text = await self.tools.execute(
                    tool_call.function.name,
                    tool_call.function.arguments,
                )
                result = self._parse_tool_result(result_text)
                is_error = isinstance(result, dict) and result.get("ok") is False
                self.sessions.append(
                    session_id,
                    "tool",
                    {
                        "id": tool_call.id,
                        "name": tool_call.function.name,
                        "arguments": tool_call.function.arguments,
                        "result": result,
                    },
                )
                yield ToolResult(tool_call.id, result, is_error)
                messages.append(
                    {
                        "role": "tool",
                        "tool_call_id": tool_call.id,
                        "name": tool_call.function.name,
                        "content": result_text,
                    }
                )
        raise OpenRouterError("unreachable tool loop state")

    async def _completion_chunks(
        self,
        messages: list[dict[str, JsonValue]],
    ) -> AsyncIterator[OpenRouterStreamChunk]:
        if not self.api_key:
            raise OpenRouterError("OPENROUTER_API_KEY is not configured")
        try:
            async with self.client.stream(
                "POST",
                f"{self.base_url}/chat/completions",
                headers={
                    "Authorization": f"Bearer {self.api_key}",
                    "Content-Type": "application/json",
                    "HTTP-Referer": "http://marmot.local",
                    "X-Title": "Marmot Faculty Timetabling",
                },
                json={
                    "model": self.model,
                    "messages": messages,
                    "tools": self.tools.definitions(),
                    "tool_choice": "auto",
                    "provider": {"sort": "throughput"},
                    "stream": True,
                    "stream_options": {"include_usage": True},
                },
            ) as response:
                if response.is_error:
                    detail = (await response.aread()).decode(errors="replace")[:1000]
                    raise OpenRouterError(
                        f"OpenRouter returned HTTP {response.status_code}: {detail}"
                    )
                async for line in response.aiter_lines():
                    if not line.startswith("data:"):
                        continue
                    payload = line[5:].strip()
                    if not payload or payload == "[DONE]":
                        continue
                    yield self._parse_stream_chunk(payload)
        except httpx.HTTPError as error:
            raise OpenRouterError(f"OpenRouter request failed: {error}") from error

    @staticmethod
    def _parse_stream_chunk(payload: str) -> OpenRouterStreamChunk:
        try:
            raw = JSON_VALUE_ADAPTER.validate_json(payload, strict=True)
        except ValidationError as error:
            raise OpenRouterError("OpenRouter streamed invalid JSON") from error
        if isinstance(raw, dict) and "error" in raw:
            raise OpenRouterError(f"OpenRouter stream error: {json.dumps(raw['error'])}")
        try:
            return OPENROUTER_STREAM_CHUNK_ADAPTER.validate_python(raw, strict=True)
        except ValidationError as error:
            raise OpenRouterError("OpenRouter streamed an invalid completion chunk") from error

    @staticmethod
    def _parse_tool_result(result: str) -> JsonValue:
        try:
            return JSON_VALUE_ADAPTER.validate_json(result, strict=True)
        except ValidationError as error:
            raise OpenRouterError("backend tool returned invalid JSON") from error

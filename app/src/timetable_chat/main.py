from __future__ import annotations

import json
from collections.abc import AsyncIterator
from contextlib import asynccontextmanager
from typing import Annotated
from uuid import uuid4

from fastapi import Depends, FastAPI, Request
from fastapi.responses import FileResponse, JSONResponse, Response, StreamingResponse
from fastapi.staticfiles import StaticFiles

from timetable_chat.config import APP_ROOT, Settings, get_settings
from timetable_chat.models import ChatRequest
from timetable_chat.openrouter import (
    AgentEvent,
    MessageFinished,
    OpenRouterAgent,
    OpenRouterError,
    TextDelta,
    ToolArgumentsDelta,
    ToolArgumentsFinished,
    ToolCallStarted,
    ToolResult,
)
from timetable_chat.preferences import PreferenceStore
from timetable_chat.prompt import build_system_prompt
from timetable_chat.semester import SemesterRepository
from timetable_chat.sessions import SessionLog
from timetable_chat.tools import ToolService


class Services:
    def __init__(self, settings: Settings) -> None:
        self.repository = SemesterRepository.load(settings.marmot_data_file)
        preference_store = PreferenceStore(
            settings.marmot_runtime_dir / "preferences", self.repository
        )
        self.tools = ToolService(self.repository, preference_store)
        self.sessions = SessionLog(settings.marmot_runtime_dir / "sessions")
        self.agent = OpenRouterAgent(
            api_key=settings.openrouter_api_key.get_secret_value(),
            model=settings.openrouter_model,
            base_url=settings.openrouter_base_url,
            max_tool_rounds=settings.max_tool_rounds,
            tools=self.tools,
            sessions=self.sessions,
        )
        self.system_prompt = build_system_prompt(self.repository)


@asynccontextmanager
async def lifespan(application: FastAPI) -> AsyncIterator[None]:
    services = Services(get_settings())
    application.state.services = services
    try:
        yield
    finally:
        await services.agent.close()


app = FastAPI(title="Marmot Faculty Timetabling", lifespan=lifespan)


def get_services(request: Request) -> Services:
    services: Services = request.app.state.services
    return services


def data_stream_frame(kind: str, value: object) -> bytes:
    return f"{kind}:{json.dumps(value, ensure_ascii=False, separators=(',', ':'))}\n".encode()


def agent_event_frame(event: AgentEvent) -> bytes:
    match event:
        case TextDelta(text):
            return data_stream_frame("0", text)
        case ToolCallStarted(tool_call_id, tool_name):
            return data_stream_frame(
                "b",
                {"toolCallId": tool_call_id, "toolName": tool_name},
            )
        case ToolArgumentsDelta(tool_call_id, text):
            return data_stream_frame(
                "c",
                {"toolCallId": tool_call_id, "argsTextDelta": text},
            )
        case ToolArgumentsFinished(tool_call_id):
            return data_stream_frame(
                "c",
                {"toolCallId": tool_call_id, "argsTextDelta": "", "isFinal": True},
            )
        case ToolResult(tool_call_id, result, is_error):
            value: dict[str, object] = {
                "toolCallId": tool_call_id,
                "result": result,
            }
            if is_error:
                value["isError"] = True
            return data_stream_frame("a", value)
        case MessageFinished(input_tokens, output_tokens):
            return data_stream_frame(
                "d",
                {
                    "finishReason": "stop",
                    "usage": {
                        "inputTokens": input_tokens,
                        "outputTokens": output_tokens,
                    },
                },
            )


@app.get("/api/health")
async def health(services: Annotated[Services, Depends(get_services)]) -> dict[str, object]:
    return {
        "status": "ok",
        "term": services.repository.semester.term,
        "faculty_count": len(services.repository.semester.faculty),
    }


@app.post("/api/chat")
async def chat(
    chat_request: ChatRequest,
    services: Annotated[Services, Depends(get_services)],
) -> Response:
    if not services.agent.api_key:
        return JSONResponse(
            status_code=503,
            content={"detail": "OPENROUTER_API_KEY is not configured"},
        )
    session_id = chat_request.session_id or chat_request.thread_id or str(uuid4())

    async def stream() -> AsyncIterator[bytes]:
        try:
            async for event in services.agent.stream(
                system_prompt=services.system_prompt,
                ui_messages=chat_request.messages,
                session_id=session_id,
            ):
                yield agent_event_frame(event)
        except OpenRouterError as error:
            services.sessions.append(session_id, "error", str(error))
            yield data_stream_frame("3", str(error))

    return StreamingResponse(
        stream(),
        media_type="text/plain; charset=utf-8",
        headers={
            "x-vercel-ai-data-stream": "v1",
            "Cache-Control": "no-cache",
            "X-Accel-Buffering": "no",
        },
    )


FRONTEND_OUT = APP_ROOT / "frontend" / "out"
if FRONTEND_OUT.exists():
    app.mount("/_next", StaticFiles(directory=FRONTEND_OUT / "_next"), name="next-static")

    @app.get("/{path:path}")
    async def frontend(path: str) -> FileResponse:
        requested = FRONTEND_OUT / path
        if requested.is_file():
            return FileResponse(requested)
        return FileResponse(FRONTEND_OUT / "index.html")

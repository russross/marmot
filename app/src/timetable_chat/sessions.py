import json
import re
from datetime import UTC, datetime
from pathlib import Path

from pydantic import JsonValue

SAFE_SESSION_ID = re.compile(r"[^a-zA-Z0-9_-]+")


class SessionLog:
    def __init__(self, directory: Path) -> None:
        self.directory = directory

    def append(self, session_id: str, event: str, payload: JsonValue) -> None:
        safe_id = SAFE_SESSION_ID.sub("-", session_id).strip("-")[:100] or "unknown"
        self.directory.mkdir(parents=True, exist_ok=True)
        record = {
            "timestamp": datetime.now(UTC).isoformat(),
            "event": event,
            "payload": payload,
        }
        with (self.directory / f"{safe_id}.jsonl").open("a", encoding="utf-8") as output:
            output.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n")

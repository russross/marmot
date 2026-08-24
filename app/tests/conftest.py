from collections.abc import Iterator
from pathlib import Path

import pytest

from timetable_chat.semester import SemesterRepository

APP_ROOT = Path(__file__).resolve().parents[1]


@pytest.fixture(scope="session")
def repository() -> SemesterRepository:
    return SemesterRepository.load(APP_ROOT / "data" / "fall-2026.json")


@pytest.fixture
def runtime_directory(tmp_path: Path) -> Iterator[Path]:
    runtime = tmp_path / "runtime"
    yield runtime

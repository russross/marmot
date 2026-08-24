from pathlib import Path

from pydantic import TypeAdapter

from timetable_chat.models import Faculty, Semester

SEMESTER_ADAPTER = TypeAdapter(Semester)


class SemesterRepository:
    def __init__(self, semester: Semester) -> None:
        self.semester = semester
        self._faculty_by_normalized_name = {
            faculty.name.casefold(): faculty for faculty in semester.faculty
        }

    @classmethod
    def load(cls, path: Path) -> "SemesterRepository":
        payload = path.read_bytes()
        return cls(SEMESTER_ADAPTER.validate_json(payload, strict=True))

    def faculty_names(self) -> list[str]:
        return [faculty.name for faculty in self.semester.faculty]

    def faculty(self, name: str) -> Faculty:
        try:
            return self._faculty_by_normalized_name[name.strip().casefold()]
        except KeyError as error:
            available = ", ".join(self.faculty_names())
            message = f"unknown faculty member {name!r}; available names: {available}"
            raise ValueError(message) from error

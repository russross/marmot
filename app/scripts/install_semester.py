#!/usr/bin/env python3

import argparse
import ast
import json
import sqlite3
from collections.abc import Sequence
from contextlib import suppress
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Never


@dataclass(frozen=True)
class Interval:
    days: str
    start: str
    end: str


@dataclass(frozen=True)
class FacultySource:
    name: str
    department: str
    availability: list[Interval]
    section_setup: list["SectionSetup"]
    approved_unavailable_time_slots: list[str]
    current_preferences: str | None


@dataclass(frozen=True)
class SectionSetup:
    method: str
    name: str
    tags: list[str]


@dataclass(frozen=True)
class HistoricalInput:
    term: str
    faculty_source_path: Path


LEGACY_PREFERENCE_NAMES = {
    "DaysOff",
    "EvenlySpread",
    "NoRoomSwitch",
    "TooManyRooms",
    "GapTooShort",
    "GapTooLong",
    "ClusterTooShort",
    "ClusterTooLong",
}


def fail(message: str) -> Never:
    raise ValueError(message)


def call_name(node: ast.Call) -> str | None:
    match node.func:
        case ast.Attribute(attr=name):
            return name
        case ast.Name(id=name):
            return name
        case _:
            return None


def string_constant(node: ast.expr) -> str:
    match node:
        case ast.Constant(value=str(value)):
            return value
        case _:
            return fail(f"expected a string at line {node.lineno}")


def integer_constant(node: ast.expr) -> int:
    match node:
        case ast.Constant(value=int(value)):
            return value
        case _:
            return fail(f"expected an integer at line {node.lineno}")


def parse_interval(node: ast.expr) -> Interval:
    match node:
        case ast.Call(
            func=ast.Name(id="TimeInterval" | "Available"),
            args=[days, start, end, *_],
        ):
            return Interval(
                days=string_constant(days),
                start=string_constant(start),
                end=string_constant(end),
            )
        case _:
            return fail(f"expected TimeInterval at line {node.lineno}")


def parse_intervals(node: ast.expr, variables: dict[str, list[Interval]]) -> list[Interval]:
    match node:
        case ast.Name(id=name):
            try:
                return variables[name]
            except KeyError:
                return fail(f"unknown availability variable {name!r} at line {node.lineno}")
        case ast.List(elts=items):
            return [parse_interval(item) for item in items]
        case ast.BinOp(left=left, op=ast.Add(), right=right):
            return [
                *parse_intervals(left, variables),
                *parse_intervals(right, variables),
            ]
        case _:
            return fail(f"expected an availability list at line {node.lineno}")


def expand_preferences(
    nodes: list[ast.expr],
    variables: dict[str, list[ast.expr]],
) -> list[ast.expr]:
    expanded: list[ast.expr] = []
    for node in nodes:
        if not isinstance(node, ast.Starred) or not isinstance(node.value, ast.Name):
            expanded.append(node)
            continue
        try:
            expanded.extend(variables[node.value.id])
        except KeyError:
            fail(f"unknown preference variable {node.value.id!r} at line {node.lineno}")
    return expanded


def convert_legacy_preference(node: ast.expr) -> str:
    if not isinstance(node, ast.Call):
        return fail(f"expected a legacy preference call at line {node.lineno}")
    name = call_name(node)
    if name == "DaysOff" and len(node.args) == 2:
        days_off = integer_constant(node.args[0])
        priority = integer_constant(node.args[1])
        match days_off:
            case 0:
                return f"DoNotWantADayOff(priority={priority})"
            case 1:
                return f"WantADayOff(priority={priority})"
            case _:
                return fail(f"cannot convert DaysOff({days_off}) at line {node.lineno}")

    simple_names = {
        "EvenlySpread": "WantClassesEvenlySpreadAcrossDays",
        "NoRoomSwitch": "WantBackToBackClassesInTheSameRoom",
        "TooManyRooms": "WantClassesPackedIntoAsFewRoomsAsPossible",
    }
    if name in simple_names and len(node.args) == 1:
        priority = integer_constant(node.args[0])
        return f"{simple_names[name]}(priority={priority})"

    duration_names = {
        "GapTooShort": "AvoidGapBetweenClassClustersShorterThan",
        "GapTooLong": "AvoidGapBetweenClassClustersLongerThan",
        "ClusterTooShort": "AvoidClassClusterShorterThan",
        "ClusterTooLong": "AvoidClassClusterLongerThan",
    }
    if name in duration_names and len(node.args) == 2:
        minutes = integer_constant(node.args[0])
        priority = integer_constant(node.args[1])
        return f"{duration_names[name]}({minutes}, priority={priority})"
    return fail(f"cannot convert legacy preference {name!r} at line {node.lineno}")


def preference_snippet(
    *,
    source: str,
    statement: ast.Expr,
    call: ast.Call,
    faculty_name: str,
    preference_variables: dict[str, list[ast.expr]],
) -> str:
    preference_nodes = expand_preferences(call.args[2:], preference_variables)
    names = {
        name
        for node in preference_nodes
        if isinstance(node, ast.Call) and (name := call_name(node)) is not None
    }
    has_legacy_preferences = bool(names & LEGACY_PREFERENCE_NAMES)
    has_expansion = any(isinstance(node, ast.Starred) for node in call.args[2:])
    if not has_legacy_preferences and not has_expansion:
        snippet = ast.get_source_segment(source, statement)
        if snippet is None:
            fail(f"could not extract preferences for {faculty_name}")
        return snippet

    days_to_check = string_constant(call.args[1])
    if has_legacy_preferences:
        rendered = [convert_legacy_preference(node) for node in preference_nodes]
    else:
        rendered = [ast.unparse(node) for node in preference_nodes]
    lines = [f"db.faculty_preferences({faculty_name!r}, {days_to_check!r},"]
    lines.extend(f"    {preference}," for preference in rendered)
    lines.append(")")
    return "\n".join(lines)


def extract_faculty_source(path: Path) -> dict[str, FacultySource]:
    source = path.read_text(encoding="utf-8")
    module = ast.parse(source, filename=str(path))
    build_function = next(
        (
            node
            for node in module.body
            if isinstance(node, ast.FunctionDef) and node.name in {"build_faculty", "build"}
        ),
        None,
    )
    if build_function is None:
        fail(f"{path} has no build_faculty or build function")

    availability_variables: dict[str, list[Interval]] = {}
    preference_variables: dict[str, list[ast.expr]] = {}
    faculty: dict[str, FacultySource] = {}
    for statement in build_function.body:
        if isinstance(statement, ast.Assign) and len(statement.targets) == 1:
            target = statement.targets[0]
            if isinstance(target, ast.Name) and isinstance(statement.value, ast.List):
                preference_variables[target.id] = list(statement.value.elts)
                with suppress(ValueError):
                    availability_variables[target.id] = parse_intervals(
                        statement.value, availability_variables
                    )
            continue

        if not isinstance(statement, ast.Expr) or not isinstance(statement.value, ast.Call):
            continue
        call = statement.value
        name = call_name(call)
        if name == "make_faculty" and len(call.args) >= 3:
            faculty_name = string_constant(call.args[0])
            faculty[faculty_name] = FacultySource(
                name=faculty_name,
                department=string_constant(call.args[1]),
                availability=parse_intervals(call.args[2], availability_variables),
                section_setup=[],
                approved_unavailable_time_slots=[],
                current_preferences=None,
            )
            continue
        if name in {"make_faculty_section", "assign_faculty_to_existing_section"}:
            if len(call.args) < 2:
                continue
            faculty_name = string_constant(call.args[0])
            existing = faculty.get(faculty_name)
            if existing is None:
                continue
            tags = [string_constant(argument) for argument in call.args[2:]]
            existing.section_setup.append(
                SectionSetup(method=name, name=string_constant(call.args[1]), tags=tags)
            )
            continue
        if name != "faculty_preferences" or len(call.args) < 2:
            continue

        faculty_name = string_constant(call.args[0])
        existing = faculty.get(faculty_name)
        if existing is None:
            continue
        snippet = preference_snippet(
            source=source,
            statement=statement,
            call=call,
            faculty_name=faculty_name,
            preference_variables=preference_variables,
        )
        approved_unavailable_time_slots = [
            string_constant(preference.args[0])
            for preference in call.args[2:]
            if isinstance(preference, ast.Call)
            and call_name(preference) == "UnavailableTimeSlot"
            and len(preference.args) == 1
        ]
        faculty[faculty_name] = FacultySource(
            name=existing.name,
            department=existing.department,
            availability=existing.availability,
            section_setup=existing.section_setup,
            approved_unavailable_time_slots=approved_unavailable_time_slots,
            current_preferences=snippet,
        )
    return faculty


def rows(connection: sqlite3.Connection, query: str) -> list[sqlite3.Row]:
    return list(connection.execute(query))


def grouped_values(
    connection: sqlite3.Connection,
    query: str,
    *,
    key_column: str,
    value_column: str,
) -> dict[str, list[str]]:
    grouped: dict[str, list[str]] = {}
    for row in rows(connection, query):
        grouped.setdefault(str(row[key_column]), []).append(str(row[value_column]))
    return grouped


def build_snapshot(
    *,
    database_path: Path,
    current_source_path: Path,
    historical_inputs: Sequence[HistoricalInput],
    term: str,
) -> dict[str, object]:
    if len(historical_inputs) != 2:
        fail("exactly two historical faculty inputs are required")
    historical_terms = [historical.term for historical in historical_inputs]
    if len(set(historical_terms)) != 2:
        fail("historical faculty input terms must be distinct")

    current_source = extract_faculty_source(current_source_path)
    historical_sources = [
        (historical, extract_faculty_source(historical.faculty_source_path))
        for historical in historical_inputs
    ]

    connection = sqlite3.connect(f"file:{database_path}?mode=ro", uri=True)
    connection.row_factory = sqlite3.Row
    try:
        faculty_sections = grouped_values(
            connection,
            "SELECT faculty, section FROM faculty_sections ORDER BY faculty, section",
            key_column="faculty",
            value_column="section",
        )
        section_room_tags = grouped_values(
            connection,
            "SELECT section, room_tag FROM section_room_tags ORDER BY section, room_tag",
            key_column="section",
            value_column="room_tag",
        )
        section_time_tags = grouped_values(
            connection,
            "SELECT section, time_slot_tag FROM section_time_slot_tags "
            "ORDER BY section, time_slot_tag",
            key_column="section",
            value_column="time_slot_tag",
        )
        room_tags = grouped_values(
            connection,
            "SELECT room_tag, room FROM rooms_room_tags ORDER BY room_tag, room",
            key_column="room_tag",
            value_column="room",
        )
        time_slot_tags = grouped_values(
            connection,
            "SELECT time_slot_tag, time_slot FROM time_slots_time_slot_tags "
            "ORDER BY time_slot_tag, time_slot",
            key_column="time_slot_tag",
            value_column="time_slot",
        )

        faculty_payload: list[dict[str, object]] = []
        for name, current in sorted(current_source.items()):
            sections = []
            for section in faculty_sections.get(name, []):
                sections.append(
                    {
                        "name": section,
                        "room_tags": section_room_tags.get(section, []),
                        "time_slot_tags": section_time_tags.get(section, []),
                    }
                )
            faculty_payload.append(
                {
                    "name": name,
                    "department": current.department,
                    "availability": [asdict(interval) for interval in current.availability],
                    "sections": sections,
                    "section_setup": [asdict(section) for section in current.section_setup],
                    "approved_unavailable_time_slots": (current.approved_unavailable_time_slots),
                    "current_preferences": current.current_preferences,
                    "preference_history": [
                        {
                            "term": historical.term,
                            "faculty_present": name in source,
                            "preferences": (
                                source[name].current_preferences if name in source else None
                            ),
                        }
                        for historical, source in historical_sources
                    ],
                }
            )

        programs: list[dict[str, object]] = []
        for program_row in rows(
            connection, "SELECT program, department FROM programs ORDER BY program"
        ):
            program_name = str(program_row["program"])
            conflict_payload: list[dict[str, object]] = []
            for conflict_row in connection.execute(
                "SELECT conflict_name, conflict_priority, boost_priority "
                "FROM conflicts WHERE program = ? ORDER BY conflict_name",
                (program_name,),
            ):
                conflict_name = str(conflict_row["conflict_name"])
                members = [
                    str(row["course"])
                    for row in connection.execute(
                        "SELECT course FROM conflict_courses "
                        "WHERE program = ? AND conflict_name = ? ORDER BY course",
                        (program_name, conflict_name),
                    )
                ]
                members.extend(
                    str(row["section"])
                    for row in connection.execute(
                        "SELECT section FROM conflict_sections "
                        "WHERE program = ? AND conflict_name = ? ORDER BY section",
                        (program_name, conflict_name),
                    )
                )
                conflict_payload.append(
                    {
                        "name": conflict_name,
                        "priority": conflict_row["conflict_priority"],
                        "mode": "boost" if conflict_row["boost_priority"] else "reduce",
                        "members": members,
                    }
                )
            programs.append(
                {
                    "name": program_name,
                    "department": str(program_row["department"]),
                    "conflicts": conflict_payload,
                }
            )

        return {
            "term": term,
            "historical_terms": historical_terms,
            "provenance": {
                "database": str(database_path),
                "current_faculty_source": str(current_source_path),
                "historical_faculty_sources": [
                    {
                        "term": historical.term,
                        "faculty_source": str(historical.faculty_source_path),
                    }
                    for historical in historical_inputs
                ],
                "note": (
                    "The fall2026 source database reports Spring 2026 internally; "
                    "the directory and requested test scenario define this snapshot as Fall 2026."
                ),
            },
            "faculty": faculty_payload,
            "rooms": [
                {"name": str(row["room"]), "capacity": int(row["capacity"])}
                for row in rows(connection, "SELECT room, capacity FROM rooms ORDER BY room")
            ],
            "room_tags": room_tags,
            "time_slots": [
                {
                    "name": str(row["time_slot"]),
                    "days": str(row["days"]),
                    "start_minutes": int(row["start_time"]),
                    "duration_minutes": int(row["duration"]),
                }
                for row in rows(
                    connection,
                    "SELECT time_slot, days, start_time, duration FROM time_slots "
                    "ORDER BY first_day, start_time, duration",
                )
            ],
            "time_slot_tags": time_slot_tags,
            "courses": [
                {
                    "code": str(row["course"]),
                    "department": str(row["department"]),
                    "name": str(row["course_name"]),
                }
                for row in rows(
                    connection,
                    "SELECT course, department, course_name FROM courses ORDER BY course",
                )
            ],
            "programs": programs,
        }
    finally:
        connection.close()


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description="Install a Marmot semester snapshot")
    result.add_argument("--database", required=True, type=Path)
    result.add_argument("--current-faculty", required=True, type=Path)
    result.add_argument("--previous-faculty", required=True, type=Path)
    result.add_argument("--older-faculty", required=True, type=Path)
    result.add_argument("--term", required=True)
    result.add_argument("--previous-term", required=True)
    result.add_argument("--older-term", required=True)
    result.add_argument("--output", required=True, type=Path)
    return result


def main(argv: Sequence[str] | None = None) -> None:
    args = parser().parse_args(argv)
    snapshot = build_snapshot(
        database_path=args.database,
        current_source_path=args.current_faculty,
        historical_inputs=(
            HistoricalInput(
                term=args.previous_term,
                faculty_source_path=args.previous_faculty,
            ),
            HistoricalInput(
                term=args.older_term,
                faculty_source_path=args.older_faculty,
            ),
        ),
        term=args.term,
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(snapshot, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()

from dataclasses import dataclass
import sqlite3
from typing import Any, Callable, TypeVar, ParamSpec, Protocol, Self

class Available:
    def __init__(self, days: str, start_time: str, end_time: str, priority: int = 20):
        assert(len(days) > 0)
        assert(len(start_time) == 4)
        assert(len(end_time) == 4)
        assert(start_time.isdigit())
        assert(end_time.isdigit())
        start = int(start_time[:2]) * 60 + int(start_time[2:])
        end = int(end_time[:2]) * 60 + int(end_time[2:])
        assert(f'{start//60:02}{start%60:02}') == start_time
        assert(f'{end//60:02}{end%60:02}') == end_time
        assert(start < end)
        assert(end <= 24*60 and end%5 == 0)

        intervals = []
        days = days.lower()
        prev = -1
        for day in days.upper():
            i = 'MTWRFSU'.index(day, prev+1)
            intervals.append( (day, start, end, priority) )
        self.intervals = intervals

@dataclass
class FacultyPreferences:
    pass

@dataclass
class DaysOff(FacultyPreferences):
    days_off: int
    priority: int

    def __post_init__(self) -> None:
        assert(self.days_off >= 0 and self.days_off < 7)
        assert(self.priority >= 10 and self.priority < 20)

@dataclass
class EvenlySpread(FacultyPreferences):
    priority: int

    def __post_init__(self) -> None:
        assert(self.priority >= 10 and self.priority < 20)

@dataclass
class NoRoomSwitch(FacultyPreferences):
    priority: int

    def __post_init__(self) -> None:
        assert(self.priority >= 10 and self.priority < 20)

@dataclass
class TooManyRooms(FacultyPreferences):
    priority: int

    def __post_init__(self) -> None:
        assert(self.priority >= 10 and self.priority < 20)

@dataclass
class GapTooShort(FacultyPreferences):
    minutes: int
    priority: int

    def __post_init__(self) -> None:
        assert(self.minutes > 50 and self.minutes < 720)
        assert(self.priority >= 10 and self.priority < 20)

@dataclass
class GapTooLong(FacultyPreferences):
    minutes: int
    priority: int

    def __post_init__(self) -> None:
        assert(self.minutes > 50 and self.minutes < 720)
        assert(self.priority >= 10 and self.priority < 20)

@dataclass
class ClusterTooShort(FacultyPreferences):
    minutes: int
    priority: int

    def __post_init__(self) -> None:
        assert(self.minutes > 50 and self.minutes < 720)
        assert(self.priority >= 10 and self.priority < 20)

@dataclass
class ClusterTooLong(FacultyPreferences):
    minutes: int
    priority: int

    def __post_init__(self) -> None:
        assert(self.minutes > 50 and self.minutes < 720)
        assert(self.priority >= 10 and self.priority < 20)

T = TypeVar('T')
P = ParamSpec('P')

# Define a protocol for objects with a rollback method
class RollbackSupport(Protocol):
    def rollback(self) -> None: ...

def rollback_on_exception(method: Callable[P, T]) -> Callable[P, T]:
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
        if hasattr(args[0], 'rollback'):
            self: RollbackSupport = args[0]
            try:
                return method(*args, **kwargs)
            except Exception:
                self.rollback()
                raise
        else:
            return method(*args, **kwargs)

    wrapper.__name__ = method.__name__
    wrapper.__doc__ = method.__doc__
    wrapper.__annotations__ = method.__annotations__

    return wrapper

class DB:
    def __init__(self, filename: str) -> None:
        self.db = sqlite3.connect(f'file:{filename}?mode=rw', uri=True)
        self.db.execute('PRAGMA busy_timeout = 10000')
        self.db.execute('PRAGMA foreign_keys = ON')
        self.db.execute('PRAGMA journal_mode = MEMORY')
        self.db.execute('PRAGMA locking_mode = NORMAL')
        #self.db.execute('PRAGMA synchronous = OFF')
        self.db.execute('PRAGMA temp_store = MEMORY')
        self.db.execute('PRAGMA mmap_size = 100000000')
        self.db.execute('BEGIN')

    def rollback(self) -> None:
        self.db.rollback()

    @rollback_on_exception
    def make_term(self, term: str, start_date: str, end_date: str) -> None:
        self.db.execute('INSERT INTO terms VALUES (?, ?, ?)', (term, start_date, end_date))

    @rollback_on_exception
    def make_holiday(self, holiday: str) -> None:
        self.db.execute('INSERT INTO holidays VALUES (?)', (holiday,))

    @rollback_on_exception
    def make_building(self, building: str) -> None:
        self.db.execute('INSERT INTO buildings VALUES (?)', (building,))

    @rollback_on_exception
    def make_room(self, room: str, capacity: int, room_tags: list[str]) -> None:
        self.db.execute('INSERT INTO rooms VALUES (?, ?)', (room, capacity))
        for room_tag in list(room_tags) + [room]:
            (n,) = self.db.execute('SELECT COUNT(1) FROM room_tags WHERE room_tag = ?', (room_tag,)).fetchone()
            if n == 0:
                self.db.execute('INSERT INTO room_tags VALUES (?)', (room_tag,))
            self.db.execute('INSERT INTO rooms_room_tags VALUES (?, ?)', (room_tag, room))

    @rollback_on_exception
    def make_time_slot(self, time_slot: str, time_slot_tags: list[str]) -> None:
        self.db.execute('INSERT INTO time_slots VALUES (?)', (time_slot,))
        for time_slot_tag in list(time_slot_tags) + [time_slot]:
            (n,) = self.db.execute('SELECT COUNT(1) FROM time_slot_tags WHERE time_slot_tag = ?', (time_slot_tag,)).fetchone()
            if n == 0:
                self.db.execute('INSERT INTO time_slot_tags VALUES (?)', (time_slot_tag,))
            self.db.execute('INSERT INTO time_slots_time_slot_tags VALUES (?, ?)', (time_slot_tag, time_slot))

    @rollback_on_exception
    def make_department(self, department: str) -> None:
        self.db.execute('INSERT INTO departments VALUES (?)', (department,))

    @rollback_on_exception
    def make_faculty(self, faculty: str, department: str, available: list[Available]) -> None:
        week = 'MTWRFSU'

        all_intervals = [ [ -1 for interval in range(24*60//5) ] for day in range(7) ]
        for avail in available:
            for (day_letter, start_time, end_time, priority) in avail.intervals:
                day_n = week.index(day_letter)
                for interval in range(start_time//5, end_time//5):
                    if all_intervals[day_n][interval] >= 0:
                        all_intervals[day_n][interval] = min(all_intervals[day_n][interval], priority)
                    else:
                        all_intervals[day_n][interval] = priority

        self.db.execute('INSERT INTO faculty VALUES (?, ?)', (faculty, department))

        # now reformat it
        entries = []
        for (letter, intervals) in zip(week, all_intervals):
            start_minute = 0
            prev = -1
            for (minute, priority) in zip(range(0, 24*60, 5), intervals):
                if priority == prev:
                    continue
                if prev >= 0:
                    # end of a range
                    entries.append((letter, start_minute, minute, prev))
                if priority >= 0:
                    start_minute = minute
                prev = priority
            if prev >= 0:
                entries.append((letter, start_minute, minute, prev))

        for (letter, start_minute, end_minute, priority) in entries:
            duration = end_minute - start_minute
            self.db.execute('INSERT INTO faculty_availability VALUES (?, ?, ?, ?, ?)', (faculty, letter, start_minute, duration, None if priority == 20 else priority))

    @rollback_on_exception
    def faculty_preferences(self, faculty: str, days_to_check: str, *prefs: FacultyPreferences) -> None:
        days_off, days_off_priority = (None, None)
        evenly_spread_priority = None
        no_room_switch_priority = None
        too_many_rooms_priority = None
        max_gap_within_cluster = 50
        cluster = []
        for elt in prefs:
            match elt:
                case DaysOff(days, n):
                    days_off = days
                    days_off_priority = n
                case EvenlySpread(n):
                    evenly_spread_priority = n
                case NoRoomSwitch(n):
                    no_room_switch_priority = n
                case TooManyRooms(n):
                    too_many_rooms_priority = n
                case GapTooShort(minutes, n):
                    cluster.append( (faculty, False, True, minutes, n) )
                case GapTooLong(minutes, n):
                    cluster.append( (faculty, False, False, minutes, n) )
                case ClusterTooShort(minutes, n):
                    cluster.append( (faculty, True, True, minutes, n) )
                case ClusterTooLong(minutes, n):
                    cluster.append( (faculty, True, False, minutes, n) )

        self.db.execute('INSERT INTO faculty_preferences VALUES (?, ?, ?, ?, ?, ?, ?, ?)',
            (faculty,
            days_to_check,
            days_off,
            days_off_priority,
            evenly_spread_priority,
            no_room_switch_priority,
            too_many_rooms_priority,
            max_gap_within_cluster))
        for payload in cluster:
            self.db.execute('INSERT INTO faculty_preference_intervals VALUES (?, ?, ?, ?, ?)',
                payload)

    @rollback_on_exception
    def make_course(self, department: str, course: str, course_name: str) -> None:
        self.db.execute('INSERT INTO courses VALUES (?, ?, ?)', (course, department, course_name))

    @rollback_on_exception
    def add_prereqs(self, course: str, prereqs: list[str]) -> None:
        for elt in prereqs:
            self.db.execute('INSERT INTO prereqs VALUES (?, ?)', (course, elt))

    @rollback_on_exception
    def add_coreqs(self, course: str, coreqs: list[str]) -> None:
        for elt in coreqs:
            self.db.execute('INSERT INTO coreqs VALUES (?, ?)', (course, elt))

    @rollback_on_exception
    def make_section(self, section: str, *tags: str) -> None:
        self.db.execute('INSERT INTO sections VALUES (?)', (section, ))
        for tag in tags:
            colon = tag.find(':')
            if colon >= 0:
                priority = int(tag[colon+1:])
                tag = tag[:colon]
            else:
                priority = 0
            (room_tags,) = self.db.execute('SELECT COUNT(1) FROM room_tags WHERE room_tag = ?', (tag,)).fetchone()
            (time_slot_tags,) = self.db.execute('SELECT COUNT(1) FROM time_slot_tags WHERE time_slot_tag = ?', (tag,)).fetchone()
            if room_tags == 0 and time_slot_tags == 0:
                raise RuntimeError(f'section {section} tag "{tag}" not found as room_tag or time_slot_tag')
            elif room_tags > 0 and time_slot_tags > 0:
                raise RuntimeError(f'section {section} tag "{tag}" found as both room_tag and time_slot_tag, unable to proceed')
            elif room_tags > 0:
                self.db.execute('INSERT INTO section_room_tags VALUES (?, ?, ?)', (section, tag, None if priority == 0 else priority))
            elif time_slot_tags > 0:
                self.db.execute('INSERT INTO section_time_slot_tags VALUES (?, ?, ?)', (section, tag, None if priority == 0 else priority))

    @rollback_on_exception
    def assign_faculty_sections(self, faculty: str, *sections: str) -> None:
        for section in sections:
            self.db.execute('INSERT INTO faculty_sections VALUES (?, ?)', (faculty, section))

    @rollback_on_exception
    def add_cross_listing(self, primary: str, sections: list[str]) -> None:
        if len(sections) < 1:
            raise RuntimeError(f'add_cross_listing needs at least two sections to cross list')
        self.db.execute('INSERT INTO cross_listings VALUES (?)', (primary,))
        for section in sections:
            self.db.execute('INSERT INTO cross_listing_sections VALUES (?, ?)', (section, primary))

    @rollback_on_exception
    def add_anti_conflict(self, priority: int, single: str, group: list[str]) -> None:
        if len(group) < 1:
            raise RuntimeError(f'add_anti_conflict needs at least one section in the group')
        self.db.execute('INSERT INTO anti_conflicts VALUES (?, ?)', (single, int(priority)))
        for elt in group:
            if '-' not in elt:
                self.db.execute('INSERT INTO anti_conflict_courses VALUES (?, ?)', (single, elt))
            else:
                self.db.execute('INSERT INTO anti_conflict_sections VALUES (?, ?)', (single, elt))

    @rollback_on_exception
    def make_program(self, program: str, department: str) -> None:
        self.db.execute('INSERT INTO programs VALUES (?, ?)', (program, department))

    @rollback_on_exception
    def make_conflict(self, program: str, conflict_name: str, conflict_priority: int, boost_s: str, courses: list[str]) -> None:
        if boost_s == 'boost':
            boost = True
        elif boost_s == 'reduce':
            boost = False
        else:
            raise RuntimeError(f'make_conflict: {program} {conflict_name}: boost option must be "boost" or "reduce"')
        conflict_priority = int(conflict_priority)
        if conflict_priority < 0 or conflict_priority >= 10:
            raise RuntimeError(f'make_conflict: {program} {conflict_name}: conflict priority option must be between 0 and 10')

        self.db.execute('INSERT INTO conflicts VALUES (?, ?, ?, ?)', (program, conflict_name, None if conflict_priority == 0 else conflict_priority, boost))

        for elt in courses:
            if '-' not in elt:
                (n,) = self.db.execute('SELECT COUNT(1) FROM courses WHERE course = ?', (elt,)).fetchone()
                if n == 0:
                    print(f'make_conflict error: "{program}" "{conflict_name}": no course "{elt}" found, skipping')
                else:
                    self.db.execute('INSERT INTO conflict_courses VALUES (?, ?, ?)', (program, conflict_name, elt))
            else:
                self.db.execute('INSERT INTO conflict_sections VALUES (?, ?, ?)', (program, conflict_name, elt))

    @rollback_on_exception
    def add_multiple_section_override(self, course: str, section_count: int) -> None:
        self.db.execute('INSERT INTO multiple_section_overrides VALUES (?, ?)', (course, section_count))

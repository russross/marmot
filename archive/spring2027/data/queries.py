from dataclasses import dataclass
from functools import wraps
import math
import sqlite3
from typing import Callable, Concatenate, TypeVar, Optional, ParamSpec, Protocol

MIN_PREF_PRIORITY = 10
MAX_STATED_PRIORITY = 99
PRIORITY_LEVELS = MAX_STATED_PRIORITY

@dataclass
class FacultyPreferences:
    pass

@dataclass
class WantSameDayOffAs(FacultyPreferences):
    other_faculty: str
    priority: int | None = None

    def __post_init__(self) -> None:
        if not self.other_faculty.strip():
            raise ValueError('a shared day off requires another faculty member')
        if self.priority is not None and not MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS:
            raise ValueError('faculty preference priority must be between 10 and 99')

@dataclass
class WantADayOff(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class DoNotWantADayOff(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class WantClassesEvenlySpreadAcrossDays(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class WantBackToBackClassesInTheSameRoom(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class WantClassesPackedIntoAsFewRoomsAsPossible(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class AvoidGapBetweenClassClustersShorterThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class AvoidGapBetweenClassClustersLongerThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class AvoidClassClusterShorterThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class AvoidClassClusterLongerThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class AvoidSectionInRooms(FacultyPreferences):
    section: str
    room_tags: list[str]
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class AvoidSectionInTimeSlots(FacultyPreferences):
    section: str
    time_slot_tags: list[str]
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class AvoidTimeSlot(FacultyPreferences):
    time_slot: str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)

@dataclass
class UnavailableTimeSlot(FacultyPreferences):
    time_slot: str

@dataclass
class UseSameTimePattern(FacultyPreferences):
    sections: list[str]
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or MIN_PREF_PRIORITY <= self.priority <= PRIORITY_LEVELS)


class TimeInterval:
    def __init__(self, days: str, start_time: str|int, end_time: str|int):
        assert(len(days) > 0)
        if isinstance(start_time, str):
            assert(len(start_time) == 4)
            assert(start_time.isdigit())
            start = int(start_time[:2]) * 60 + int(start_time[2:])
            assert(f'{start//60:02}{start%60:02}') == start_time
        else:
            start = start_time
        if isinstance(end_time, str):
            assert(len(end_time) == 4)
            assert(end_time.isdigit())
            end = int(end_time[:2]) * 60 + int(end_time[2:])
            assert(f'{end//60:02}{end%60:02}') == end_time
        else:
            end = end_time
        assert(start >= 0 and start%5 == 0)
        assert(start < end)
        assert(end <= 24*60 and end%5 == 0)

        intervals: list[tuple[str, int, int]] = []
        previous_index = -1
        for day in days.upper():
            previous_index = 'MTWRFSU'.index(day, previous_index + 1)
            intervals.append( (day, start, end) )
        self.intervals = intervals

def parse_minutes(duration: str|int) -> int:
    if isinstance(duration, int):
        return duration
    n = 0
    digits = ''
    for ch in duration:
        match ch:
            case digit if digit.isdigit():
                digits += digit
            case 'h' if len(digits) > 0:
                n += int(digits) * 60
                digits = ''
            case 'm' if len(digits) > 0:
                n += int(digits)
                digits = ''
            case _:
                raise RuntimeError(f'malformed duration: {duration} should be of form 2h45m')
    assert(len(digits) == 0)
    return n

T = TypeVar('T')
P = ParamSpec('P')

# Define a protocol for objects with a rollback method
class RollbackSupport(Protocol):
    def rollback(self) -> None: ...

S = TypeVar('S', bound=RollbackSupport)

def rollback_on_exception(
    method: Callable[Concatenate[S, P], T],
) -> Callable[Concatenate[S, P], T]:
    @wraps(method)
    def wrapper(self: S, *args: P.args, **kwargs: P.kwargs) -> T:
        try:
            return method(self, *args, **kwargs)
        except Exception:
            self.rollback()
            raise
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

    def _update_availability(self, faculty: str, available: list[TimeInterval]) -> None:
        if len(available) == 0:
            return

        week = 'MTWRFSU'
        all_intervals: list[list[bool]] = [
            [False for interval in range(24*60//5)] for day in range(7)
        ]

        def merge_interval(day_letter: str, start_time: int, end_time: int) -> None:
            nonlocal week, all_intervals
            day_n = week.index(day_letter)
            for interval in range(start_time//5, end_time//5):
                all_intervals[day_n][interval] = True

        # get existing intervals from the db
        rows = self.db.execute('SELECT day_of_week, start_time, duration FROM faculty_availability WHERE faculty = ?', (faculty,)).fetchall()
        for (day_letter, start_time, duration) in rows:
            merge_interval(day_letter, start_time, start_time+duration)

        # merge the new intervals we were given
        for avail in available:
            for (day_letter, start_time, end_time) in avail.intervals:
                merge_interval(day_letter, start_time, end_time)

        # consolidate everything into database format
        entries: list[tuple[str, int, int]] = []
        for (letter, intervals) in zip(week, all_intervals):
            start_minute = 0
            prev = False
            for (minute, is_available) in zip(range(0, 24*60, 5), intervals):
                if is_available == prev:
                    continue
                if prev:
                    # end of a range
                    entries.append((letter, start_minute, minute))
                if is_available:
                    start_minute = minute
                prev = is_available
            if prev:
                entries.append((letter, start_minute, minute))

        # blow away old intervals
        self.db.execute('DELETE FROM faculty_availability WHERE faculty = ?', (faculty,))

        # insert new
        for (letter, start_minute, end_minute) in entries:
            duration = end_minute - start_minute
            self.db.execute('INSERT INTO faculty_availability VALUES (?, ?, ?, ?)', (faculty, letter, start_minute, duration))

    @rollback_on_exception
    def make_faculty(self, faculty: str, department: str, available: list[TimeInterval]) -> None:
        self.db.execute('INSERT INTO faculty VALUES (?, ?)', (faculty, department))
        self._update_availability(faculty, available)

    @rollback_on_exception
    def faculty_preferences(self, faculty: str, days_to_check: str, *prefs: FacultyPreferences) -> None:
        shared_day_off_count = sum(isinstance(preference, WantSameDayOffAs) for preference in prefs)
        if shared_day_off_count > 1:
            raise ValueError(f'{faculty}: a shared day off may be requested with only one other faculty member')
        days_off, days_off_priority = (None, None)
        evenly_spread_priority = None
        no_room_switch_priority = None
        too_many_rooms_priority = None
        max_gap_within_cluster = 50
        cluster = []
        priority = MIN_PREF_PRIORITY-1
        def next_priority(p: Optional[int]) -> int:
            nonlocal priority
            if p is None:
                priority += 1
            else:
                priority = p
            if not MIN_PREF_PRIORITY <= priority <= MAX_STATED_PRIORITY:
                raise ValueError(f'{faculty}: automatic faculty preference priority must be between 10 and 99')
            return priority

        for elt in prefs:
            match elt:
                case WantSameDayOffAs(other_faculty, p):
                    if faculty == other_faculty:
                        raise ValueError(f'{faculty}: a shared day off requires a different faculty member')
                    self.db.execute(
                        'INSERT INTO faculty_shared_day_off_preferences VALUES (?, ?, ?)',
                        (faculty, other_faculty, next_priority(p)))
                case WantADayOff(p):
                    days_off = 1
                    days_off_priority = next_priority(p)
                case DoNotWantADayOff(p):
                    days_off = 0
                    days_off_priority = next_priority(p)
                case WantClassesEvenlySpreadAcrossDays(p):
                    evenly_spread_priority = next_priority(p)
                case WantBackToBackClassesInTheSameRoom(p):
                    no_room_switch_priority = next_priority(p)
                case WantClassesPackedIntoAsFewRoomsAsPossible(p):
                    too_many_rooms_priority = next_priority(p)
                case AvoidGapBetweenClassClustersShorterThan(minutes, p):
                    assert(type(minutes) == int)
                    cluster.append( (faculty, False, True, minutes, next_priority(p)) )
                case AvoidGapBetweenClassClustersLongerThan(minutes, p):
                    assert(type(minutes) == int)
                    cluster.append( (faculty, False, False, minutes, next_priority(p)) )
                case AvoidClassClusterShorterThan(minutes, p):
                    assert(type(minutes) == int)
                    cluster.append( (faculty, True, True, minutes, next_priority(p)) )
                case AvoidClassClusterLongerThan(minutes, p):
                    assert(type(minutes) == int)
                    cluster.append( (faculty, True, False, minutes, next_priority(p)) )
                case AvoidTimeSlot(time_slot, p):
                    # it has to be an existing time slot, so check it and let the db do the parsing
                    rows = self.db.execute('SELECT days, start_time, duration FROM time_slots WHERE time_slot = ?', (time_slot,)).fetchall()
                    if len(rows) != 1:
                        raise RuntimeError(f'preference {elt} is invalid because {time_slot} is not a valid time slot')
                    once = next_priority(p)
                    self.db.execute('INSERT OR IGNORE INTO faculty_time_slot_preferences VALUES (?, ?, ?)', (faculty, time_slot, once))
                    self.db.execute(
                        'UPDATE faculty_time_slot_preferences SET time_slot_priority = MIN(time_slot_priority, ?) WHERE faculty = ? AND time_slot = ?',
                        (once, faculty, time_slot))
                case AvoidSectionInTimeSlots(section, time_slot_tags, p):
                    once = next_priority(p)
                    for tag in time_slot_tags:
                        self.db.execute('INSERT OR IGNORE INTO faculty_section_time_slot_preferences VALUES (?, ?, ?, ?)', (faculty, section, tag, once))
                        self.db.execute(
                            'UPDATE faculty_section_time_slot_preferences SET time_slot_priority = MIN(time_slot_priority, ?) WHERE faculty = ? AND section = ? AND time_slot_tag = ?',
                            (once, faculty, section, tag))
                case AvoidSectionInRooms(section, room_tags, p):
                    once = next_priority(p)
                    for tag in room_tags:
                        self.db.execute('INSERT OR IGNORE INTO faculty_section_room_preferences VALUES (?, ?, ?, ?)', (faculty, section, tag, once))
                        self.db.execute(
                            'UPDATE faculty_section_room_preferences SET room_priority = MIN(room_priority, ?) WHERE faculty = ? AND section = ? AND room_tag = ?',
                            (once, faculty, section, tag))
                case UnavailableTimeSlot(time_slot):
                    # it has to be an existing time slot, so check it and let the db do the parsing
                    rows = self.db.execute('SELECT days, start_time, duration FROM time_slots WHERE time_slot = ?', (time_slot,)).fetchall()
                    if len(rows) != 1:
                        raise RuntimeError(f'preference {elt} is invalid because {time_slot} is not a valid time slot')
                    self.db.execute('INSERT OR IGNORE INTO faculty_unavailable_time_slots VALUES (?, ?)', (faculty, time_slot))
                case UseSameTimePattern(sections, p):
                    sections.sort()
                    self.db.execute('INSERT INTO faculty_time_pattern_matches VALUES (?, ?, ?)', (faculty, sections[0], next_priority(p)))
                    for section in sections:
                        self.db.execute('INSERT INTO faculty_time_pattern_match_sections VALUES (?, ?, ?)', (faculty, sections[0], section))
                case _:
                    raise RuntimeError(f'unimplemented faculty preference: {elt}')

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

    def validate_shared_day_off_preferences(self) -> None:
        rows: list[tuple[str, str, str | None, str | None, int, int, int]] = self.db.execute('''
            SELECT request.faculty, request.other_faculty, owner.days_to_check, partner.days_to_check,
                EXISTS (SELECT 1 FROM faculty_shared_day_off_preferences AS reciprocal
                    WHERE reciprocal.faculty = request.other_faculty
                      AND reciprocal.other_faculty = request.faculty),
                EXISTS (SELECT 1 FROM faculty_sections_to_be_scheduled AS sections
                    WHERE sections.faculty = request.faculty),
                EXISTS (SELECT 1 FROM faculty_sections_to_be_scheduled AS sections
                    WHERE sections.faculty = request.other_faculty)
            FROM faculty_shared_day_off_preferences AS request
            LEFT JOIN faculty_preferences AS owner ON owner.faculty = request.faculty
            LEFT JOIN faculty_preferences AS partner ON partner.faculty = request.other_faculty
        ''').fetchall()
        for faculty, partner, days, partner_days, reciprocal, owner_sections, partner_sections in rows:
            label = f'{faculty} and {partner}: shared day off'
            if not reciprocal:
                raise ValueError(f'{label} requires reciprocal preferences')
            if days is None or partner_days is None or days != partner_days or len(days) < 2:
                raise ValueError(f'{label} requires the same set of at least two representative days')
            if not owner_sections or not partner_sections:
                raise ValueError(f'{label} requires schedulable sections for both faculty')

    @rollback_on_exception
    def make_course(
        self,
        department: str,
        course: str,
        course_name: str,
        minimum_credit_hours: float,
        maximum_credit_hours: float,
    ) -> None:
        self.db.execute(
            'INSERT INTO courses VALUES (?, ?, ?, ?, ?)',
            (course, department, course_name, minimum_credit_hours, maximum_credit_hours),
        )

    @rollback_on_exception
    def add_course_rotation(self, course: str, rotation: str) -> None:
        self.db.execute('INSERT INTO course_rotations VALUES (?, ?)', (course, rotation))

    @rollback_on_exception
    def add_prereqs(self, course: str, prereqs: list[str]) -> None:
        for elt in prereqs:
            self.db.execute('INSERT INTO prereqs VALUES (?, ?)', (course, elt))

    @rollback_on_exception
    def add_coreqs(self, course: str, coreqs: list[str]) -> None:
        for elt in coreqs:
            self.db.execute('INSERT INTO coreqs VALUES (?, ?)', (course, elt))

    @rollback_on_exception
    def make_section_with_no_faculty(
        self,
        section: str,
        *tags: str,
        credit_hours: float | None = None,
    ) -> None:
        course, separator, section_number = section.rpartition('-')
        if not separator or not section_number:
            raise RuntimeError(f'section {section!r} must contain a course and section number')
        row = self.db.execute(
            'SELECT minimum_credit_hours, maximum_credit_hours FROM courses WHERE course = ?',
            (course,),
        ).fetchone()
        if row is None:
            raise RuntimeError(f'section {section!r} uses unknown course {course!r}')
        minimum_credit_hours, maximum_credit_hours = row
        if credit_hours is None:
            if minimum_credit_hours != maximum_credit_hours:
                raise RuntimeError(
                    f'section {section!r} must specify credit_hours between '
                    f'{minimum_credit_hours:g} and {maximum_credit_hours:g}'
                )
            credit_hours = minimum_credit_hours
        if isinstance(credit_hours, bool) or not isinstance(credit_hours, (int, float)):
            raise RuntimeError(f'section {section!r} credit_hours must be a number')
        credit_hours = float(credit_hours)
        if not math.isfinite(credit_hours):
            raise RuntimeError(f'section {section!r} credit_hours must be finite')
        if not minimum_credit_hours <= credit_hours <= maximum_credit_hours:
            raise RuntimeError(
                f'section {section!r} credit_hours {credit_hours:g} is outside the catalog '
                f'range {minimum_credit_hours:g}-{maximum_credit_hours:g}'
            )
        self.db.execute('INSERT INTO sections VALUES (?, ?)', (section, credit_hours))
        for tag in tags:
            (room_tags,) = self.db.execute('SELECT COUNT(1) FROM room_tags WHERE room_tag = ?', (tag,)).fetchone()
            (time_slot_tags,) = self.db.execute('SELECT COUNT(1) FROM time_slot_tags WHERE time_slot_tag = ?', (tag,)).fetchone()
            if room_tags == 0 and time_slot_tags == 0:
                # try creating a new time slot to match (this will validate the name)
                self.make_time_slot(tag, [])
                time_slot_tags += 1
                #print(f'created new time slot: {tag}')
            elif room_tags > 0 and time_slot_tags > 0:
                raise RuntimeError(f'section {section} tag "{tag}" found as both room_tag and time_slot_tag, unable to proceed')

            if room_tags > 0:
                self.db.execute('INSERT INTO section_room_tags VALUES (?, ?)', (section, tag))
            elif time_slot_tags > 0:
                self.db.execute('INSERT INTO section_time_slot_tags VALUES (?, ?)', (section, tag))

    @rollback_on_exception
    def assign_faculty_to_existing_section(self, faculty: str, section: str) -> None:
        self.db.execute('INSERT INTO faculty_sections VALUES (?, ?)', (faculty, section))

    @rollback_on_exception
    def make_faculty_section(
        self,
        faculty: str,
        section: str,
        *tags: str,
        credit_hours: float | None = None,
    ) -> None:
        self.make_section_with_no_faculty(section, *tags, credit_hours=credit_hours)
        self.assign_faculty_to_existing_section(faculty, section)

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
    def make_conflict(self, program: str, conflict_name: str, conflict_priority: Optional[int], boost: str, courses: list[str]) -> None:
        if boost not in ('boost', 'reduce'):
            raise RuntimeError(f'make_conflict: {program} {conflict_name}: boost option must be "boost" or "reduce"')
        if boost == 'boost' and conflict_priority is None:
            raise RuntimeError(f'make_conflict: {program} {conflict_name}: cannot "boost" with priority None')
        if conflict_priority is not None and (conflict_priority < 0 or conflict_priority >= MIN_PREF_PRIORITY):
            raise RuntimeError(f'make_conflict: {program} {conflict_name}: conflict priority option must be None or between 0 and {MIN_PREF_PRIORITY-1}')

        self.db.execute('INSERT INTO conflicts VALUES (?, ?, ?, ?)', (program, conflict_name, conflict_priority, boost == 'boost'))

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
    def add_multiple_section_override(self, course: str, override_section_count: int) -> None:
        self.db.execute('INSERT INTO multiple_section_overrides VALUES (?, ?)', (course, override_section_count))

from dataclasses import dataclass
import sqlite3
from typing import Any, Callable, TypeVar, Optional, ParamSpec, Protocol, Self

MIN_PREF_PRIORITY = 10
PRIORITY_LEVELS = 25

@dataclass
class FacultyPreferences:
    pass

@dataclass
class WantADayOff(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class DoNotWantADayOff(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class WantClassesEvenlySpreadAcrossDays(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class WantBackToBackClassesInTheSameRoom(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class WantClassesPackedIntoAsFewRoomsAsPossible(FacultyPreferences):
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class AvoidGapBetweenClassClustersShorterThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class AvoidGapBetweenClassClustersLongerThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class AvoidClassClusterShorterThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class AvoidClassClusterLongerThan(FacultyPreferences):
    minutes: int|str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        self.minutes = parse_minutes(self.minutes)
        assert(type(self.minutes) == int and self.minutes > 50 and self.minutes < 720)
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class AvoidSectionInRooms(FacultyPreferences):
    section: str
    room_tags: list[str]
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class AvoidSectionInTimeSlots(FacultyPreferences):
    section: str
    time_slot_tags: list[str]
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class AvoidTimeSlot(FacultyPreferences):
    time_slot: str
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)

@dataclass
class UnavailableTimeSlot(FacultyPreferences):
    time_slot: str

@dataclass
class UseSameTimePattern(FacultyPreferences):
    sections: list[str]
    priority: Optional[int] = None

    def __post_init__(self) -> None:
        assert(self.priority is None or self.priority >= MIN_PREF_PRIORITY and self.priority < PRIORITY_LEVELS)


class TimeInterval:
    def __init__(self, days: str, start_time: str|int, end_time: str|int, priority: int = PRIORITY_LEVELS):
        assert(len(days) > 0)
        if type(start_time) == str:
            assert(len(start_time) == 4)
            assert(start_time.isdigit())
            start = int(start_time[:2]) * 60 + int(start_time[2:])
            assert(f'{start//60:02}{start%60:02}') == start_time
        else:
            assert(type(start_time) == int)
            start = start_time
        if type(end_time) == str:
            assert(len(end_time) == 4)
            assert(end_time.isdigit())
            end = int(end_time[:2]) * 60 + int(end_time[2:])
            assert(f'{end//60:02}{end%60:02}') == end_time
        else:
            assert(type(end_time) == int)
            end = end_time
        assert(start >= 0 and start%5 == 0)
        assert(start < end)
        assert(end <= 24*60 and end%5 == 0)

        intervals = []
        prev = -1
        for day in days.upper():
            i = 'MTWRFSU'.index(day, prev+1)
            intervals.append( (day, start, end, priority) )
        self.intervals = intervals

def parse_minutes(duration: str|int) -> int:
    if type(duration) == int:
        return duration
    assert(type(duration) == str)
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

    def _update_availability(self, faculty: str, available: list[TimeInterval]) -> None:
        if len(available) == 0:
            return

        week = 'MTWRFSU'
        all_intervals = [ [ -1 for interval in range(24*60//5) ] for day in range(7) ]

        # note: priority of -1 means unavailable, PRIORITY_LEVELS means no penalty
        def merge_interval(day_letter: str, start_time: int, end_time: int, priority: int) -> None:
            nonlocal week, all_intervals
            day_n = week.index(day_letter)
            for interval in range(start_time//5, end_time//5):
                if all_intervals[day_n][interval] >= 0:
                    all_intervals[day_n][interval] = min(all_intervals[day_n][interval], priority)
                else:
                    all_intervals[day_n][interval] = priority

        # get existing intervals from the db
        rows = self.db.execute('SELECT day_of_week, start_time, duration, availability_priority FROM faculty_availability WHERE faculty = ?', (faculty,)).fetchall()
        for (day_letter, start_time, duration, maybe_priority) in rows:
            if maybe_priority is None:
                priority = PRIORITY_LEVELS
            else:
                priority = maybe_priority
            merge_interval(day_letter, start_time, start_time+duration, priority)

        # merge the new intervals we were given
        for avail in available:
            for (day_letter, start_time, end_time, priority) in avail.intervals:
                merge_interval(day_letter, start_time, end_time, priority)

        # consolidate everything into database format
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

        # blow away old intervals
        self.db.execute('DELETE FROM faculty_availability WHERE faculty = ?', (faculty,))

        # insert new
        for (letter, start_minute, end_minute, priority) in entries:
            duration = end_minute - start_minute
            self.db.execute('INSERT INTO faculty_availability VALUES (?, ?, ?, ?, ?)', (faculty, letter, start_minute, duration, None if priority == PRIORITY_LEVELS else priority))

    @rollback_on_exception
    def make_faculty(self, faculty: str, department: str, available: list[TimeInterval]) -> None:
        self.db.execute('INSERT INTO faculty VALUES (?, ?)', (faculty, department))
        self._update_availability(faculty, available)

    @rollback_on_exception
    def faculty_preferences(self, faculty: str, days_to_check: str, *prefs: FacultyPreferences) -> None:
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
            return priority

        for elt in prefs:
            match elt:
                case WantADayOff(p):
                    days_off = 1
                    days_off_priority = next_priority(p)

                    # day off request uses two priority slots
                    next_priority(None)
                case DoNotWantADayOff(p):
                    days_off = 0
                    days_off_priority = next_priority(p)
                case WantClassesEvenlySpreadAcrossDays(p):
                    evenly_spread_priority = next_priority(p)
                case WantBackToBackClassesInTheSameRoom(p):
                    no_room_switch_priority = next_priority(p)
                case WantClassesPackedIntoAsFewRoomsAsPossible(n):
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
                    (days, start_time, duration) = rows[0]
                    interval = TimeInterval(days, start_time, start_time+duration, next_priority(p))
                    self._update_availability(faculty, [interval])
                case AvoidSectionInTimeSlots(section, time_slot_tags, p):
                    once = next_priority(p)
                    for tag in time_slot_tags:
                        self.db.execute('DELETE FROM section_time_slot_tags WHERE section = ? AND time_slot_tag = ?', (section, tag))
                        self.db.execute('INSERT INTO section_time_slot_tags VALUES (?, ?, ?)', (section, tag, once))
                case AvoidSectionInRooms(section, room_tags, p):
                    # section/room requests do not "use up" a priority slot
                    if p is None:
                        once = priority + 1
                    else:
                        once = p
                    for tag in room_tags:
                        self.db.execute('DELETE FROM section_room_tags WHERE section = ? AND room_tag = ?', (section, tag))
                        self.db.execute('INSERT INTO section_room_tags VALUES (?, ?, ?)', (section, tag, once))
                case UnavailableTimeSlot(time_slot):
                    # it has to be an existing time slot, so check it and let the db do the parsing
                    rows = self.db.execute('SELECT days, start_time, duration FROM time_slots WHERE time_slot = ?', (time_slot,)).fetchall()
                    if len(rows) != 1:
                        raise RuntimeError(f'preference {elt} is invalid because {time_slot} is not a valid time slot')
                    (days, start_time, duration) = rows[0]
                    interval = TimeInterval(days, start_time, start_time+duration, -1)
                    self._update_availability(faculty, [interval])
                case UseSameTimePattern(sections, p):
                    sections.sort()
                    self.db.execute('INSERT INTO time_pattern_matches VALUES (?, ?)', (sections[0], next_priority(p)))
                    for section in sections:
                        self.db.execute('INSERT INTO time_pattern_match_sections VALUES (?, ?)', (sections[0], section))
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

    @rollback_on_exception
    def make_course(self, department: str, course: str, course_name: str) -> None:
        self.db.execute('INSERT INTO courses VALUES (?, ?, ?)', (course, department, course_name))

    @rollback_on_exception
    def add_course_rotation(self, course: str, term: str) -> None:
        self.db.execute('INSERT INTO course_rotations VALUES (?, ?)', (course, term))

    @rollback_on_exception
    def add_prereqs(self, course: str, prereqs: list[str]) -> None:
        for elt in prereqs:
            self.db.execute('INSERT INTO prereqs VALUES (?, ?)', (course, elt))

    @rollback_on_exception
    def add_coreqs(self, course: str, coreqs: list[str]) -> None:
        for elt in coreqs:
            self.db.execute('INSERT INTO coreqs VALUES (?, ?)', (course, elt))

    @rollback_on_exception
    def make_section_with_no_faculty(self, section: str, *tags: str) -> None:
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
                # try creating a new time slot to match (this will validate the name)
                self.make_time_slot(tag, [])
                time_slot_tags += 1
                #print(f'created new time slot: {tag}')
            elif room_tags > 0 and time_slot_tags > 0:
                raise RuntimeError(f'section {section} tag "{tag}" found as both room_tag and time_slot_tag, unable to proceed')

            if room_tags > 0:
                self.db.execute('INSERT INTO section_room_tags VALUES (?, ?, ?)', (section, tag, None if priority == 0 else priority))
            elif time_slot_tags > 0:
                self.db.execute('INSERT INTO section_time_slot_tags VALUES (?, ?, ?)', (section, tag, None if priority == 0 else priority))

    @rollback_on_exception
    def assign_faculty_to_existing_section(self, faculty: str, section: str) -> None:
        self.db.execute('INSERT INTO faculty_sections VALUES (?, ?)', (faculty, section))

        # add time slots to faculty availability if applicable
        # this adds any time slot tag that name a specific time slot to the faculty availability
        rows = self.db.execute(
            '''SELECT days, start_time, duration
            FROM section_time_slot_tags
            NATURAL JOIN time_slots_time_slot_tags
            NATURAL JOIN time_slots
            WHERE time_slot_tag = time_slot
            AND section = ?''', (section,)).fetchall()
        intervals = [TimeInterval(days, start_time, start_time+duration) 
            for (days, start_time, duration) in rows]
        if len(intervals) > 0:
            self._update_availability(faculty, intervals)

    @rollback_on_exception
    def make_faculty_section(self, faculty: str, section: str, *tags: str) -> None:
        self.make_section_with_no_faculty(section, *tags)
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
    def add_multiple_section_override(self, course: str, section_count: int) -> None:
        self.db.execute('INSERT INTO multiple_section_overrides VALUES (?, ?)', (course, section_count))

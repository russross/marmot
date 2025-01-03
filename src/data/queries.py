import sqlite3
from typing import Any, Callable, TypeVar, ParamSpec, Protocol, Self

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
    def make_faculty(self, faculty: str, department: str, availability: str) -> None:
        week = 'MTWRFSU'

        # parse the availability string: "MWF 0900-1200 with priority 10, ..."
        all_intervals = [ [ -1 for interval in range(24*60//5) ] for day in range(7) ]
        for span in availability.split(','):
            days = []
            span = span.strip()
            while len(span) > 0 and span[0].upper() in week:
                days.append(week.index(span[0].upper()))
                span = span[1:]
            if len(days) == 0:
                raise RuntimeError(f'faculty {faculty} availability span must start with days of week, e.g., MWF')
            span = span.strip()

            start = ''
            while len(span) > 0 and span[0].isdigit():
                start += span[0]
                span = span[1:]
            if span[0] != '-':
                raise RuntimeError(f'faculty {faculty} availability span time must be of form start-end, e.g., 0800-1030')
            span = span[1:]
            end = ''
            while len(span) > 0 and span[0].isdigit():
                end += span[0]
                span = span[1:]
            if len(start) < 3 or len(start) > 4 or len(end) < 3 or len(end) > 4:
                raise RuntimeError(f'faculty {faculty} availability span time must be of form start-end, e.g., 0800-1030')
            start = ('0000' + start)[-4:]
            end = ('0000' + end)[-4:]
            start_time = int(start[:2]) * 60 + int(start[2:])
            end_time = int(end[:2]) * 60 + int(end[2:])
            if start_time % 5 != 0 or end_time % 5 != 0 or start_time >= end_time or end_time > 24*60:
                raise RuntimeError(f'faculty {faculty} start must come before end time and end time must be before midnight')

            span = span.strip()
            if span.startswith('with priority '):
                span = span[len('with priority '):].strip()
                priority = int(span)
                if priority < 10 or priority >= 20:
                    raise RuntimeError(f'faculty {faculty} availability span priority must be between 10 and 19')
            elif span != '':
                raise RuntimeError(f'faculty {faculty} availability span must end with no priority or "with priority xyz"')
            else:
                priority = 20

            for day in days:
                for interval in range(start_time//5, end_time//5):
                    if all_intervals[day][interval] >= 0:
                        all_intervals[day][interval] = min(all_intervals[day][interval], priority)
                    else:
                        all_intervals[day][interval] = priority

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
    def faculty_default_clustering(self, faculty: str, days_to_check: str, days_off: int) -> None:
        self.db.execute('INSERT INTO faculty_preferences VALUES (?, ?, ?, ?, ?, ?)',
                (faculty, days_to_check, None if days_off < 0 else days_off, None if days_off < 0 else 11, 11, 15))
        intervals = (
            (True, True, 110, 15),
            (True, False, 165, 11),
            (False, True, 60, 11),
            (False, False, 105, 15),
            (False, False, 195, 11),
        )
        for (is_cluster, is_too_short, interval_minutes, interval_priority) in intervals:
            self.db.execute('INSERT INTO faculty_preference_intervals VALUES (?, ?, ?, ?, ?)',
                (faculty, is_cluster, is_too_short, interval_minutes, None if interval_priority == 0 else interval_priority))

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
    def make_section(self, section: str, tags: list[str]) -> None:
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
    def assign_faculty_sections(self, faculty: str, sections: list[str]) -> None:
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

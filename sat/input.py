"""
Input processing for the Marmot SAT-based timetabling system.

This module is responsible for loading data from the database into the
data structures defined in data.py.
"""
import sqlite3
import sys
import os.path
from itertools import combinations
from collections import defaultdict

from data import (
    TimetableData, Days, Time, Duration, Room, TimeSlot, Section, Faculty,
    SectionName, TimeSlotName, RoomName, FacultyName, Day, Priority,
    Conflict, AntiConflict, RoomPreference, TimeSlotPreference,
    FacultyDaysOff, FacultyEvenlySpread, FacultyNoRoomSwitch, FacultyTooManyRooms,
    FacultyGapTooShort, FacultyGapTooLong, FacultyClusterTooShort, FacultyClusterTooLong,
    TimePatternMatch
)


def load_timetable_data(db_path: str) -> TimetableData:
    """
    Load timetable data from the database.
    
    Args:
        db_path: Path to the SQLite database file
        
    Returns:
        A populated TimetableData object
    """
    # Connect to the database
    if not os.path.exists(db_path):
        print(f'Database file not found: {db_path}')
        sys.exit(1)
    conn = sqlite3.connect(f'file:{db_path}?mode=ro', uri=True)
    conn.execute("PRAGMA foreign_keys = ON")
    conn.execute("PRAGMA temp_store = memory")
    conn.execute("PRAGMA mmap_size = 100000000")
    
    # Create a cursor
    cursor = conn.cursor()
    
    # Get term name
    cursor.execute("SELECT term FROM terms")
    term_name = cursor.fetchone()[0]
    
    # Create temporary data containers for building the immutable structures
    temp_rooms: dict[RoomName, Room] = {}
    temp_time_slots: dict[TimeSlotName, TimeSlot] = {}
    temp_time_slot_conflicts: set[tuple[TimeSlotName, TimeSlotName]] = set()
    
    # Temporary containers for sections with mutable attributes
    temp_section_available_rooms: dict[SectionName, set[RoomName]] = defaultdict(set)
    temp_section_available_time_slots: dict[SectionName, set[TimeSlotName]] = defaultdict(set)
    temp_section_room_preferences: dict[SectionName, dict[RoomName, Priority]] = defaultdict(dict)
    temp_section_time_slot_preferences: dict[SectionName, dict[TimeSlotName, Priority]] = defaultdict(dict)
    temp_section_faculty: dict[SectionName, set[FacultyName]] = defaultdict(set)
    
    # Temporary containers for faculty with mutable attributes
    temp_faculty_sections: dict[FacultyName, set[SectionName]] = defaultdict(set)
    
    # Temporary containers for constraints
    temp_conflicts: set[Conflict] = set()
    temp_anti_conflicts: set[AntiConflict] = set()
    temp_room_preferences: set[RoomPreference] = set()
    temp_time_slot_preferences: set[TimeSlotPreference] = set()
    temp_faculty_days_off: set[FacultyDaysOff] = set()
    temp_faculty_evenly_spread: set[FacultyEvenlySpread] = set()
    temp_faculty_no_room_switch: set[FacultyNoRoomSwitch] = set()
    temp_faculty_too_many_rooms: set[FacultyTooManyRooms] = set()
    temp_faculty_gap_too_short: set[FacultyGapTooShort] = set()
    temp_faculty_gap_too_long: set[FacultyGapTooLong] = set()
    temp_faculty_cluster_too_short: set[FacultyClusterTooShort] = set()
    temp_faculty_cluster_too_long: set[FacultyClusterTooLong] = set()
    temp_time_pattern_matches: set[TimePatternMatch] = set()
    
    # Load data into temporary structures
    load_rooms(cursor, temp_rooms)
    load_time_slots(cursor, temp_time_slots)
    load_time_slot_conflicts(cursor, temp_time_slots, temp_time_slot_conflicts)
    load_sections_and_time_slots(cursor, temp_time_slots, temp_section_available_time_slots, 
                                temp_section_time_slot_preferences, temp_time_slot_preferences)
    load_sections_and_rooms(cursor, temp_rooms, temp_section_available_rooms, 
                           temp_section_room_preferences, temp_room_preferences)
    load_faculty(cursor, temp_faculty_sections)
    load_conflicts(cursor, temp_section_available_time_slots, temp_conflicts)
    load_anti_conflicts(cursor, temp_section_available_time_slots, temp_anti_conflicts)
    load_time_pattern_matches(cursor, temp_section_available_time_slots, temp_time_pattern_matches)
    load_faculty_section_assignments(cursor, temp_faculty_sections, temp_section_faculty)
    load_faculty_preferences(cursor, temp_faculty_sections, temp_section_available_rooms, 
                            temp_faculty_days_off, temp_faculty_evenly_spread, 
                            temp_faculty_no_room_switch, temp_faculty_too_many_rooms,
                            temp_faculty_gap_too_short, temp_faculty_gap_too_long,
                            temp_faculty_cluster_too_short, temp_faculty_cluster_too_long)
    
    # Now build the final immutable sections
    sections: dict[SectionName, Section] = {}
    for section_name in set(temp_section_available_rooms.keys()) | set(temp_section_available_time_slots.keys()):
        sections[section_name] = Section(
            name=section_name,
            available_rooms=frozenset(temp_section_available_rooms.get(section_name, set())),
            available_time_slots=frozenset(temp_section_available_time_slots.get(section_name, set())),
            room_preferences=dict(temp_section_room_preferences.get(section_name, {})),
            time_slot_preferences=dict(temp_section_time_slot_preferences.get(section_name, {})),
            faculty=frozenset(temp_section_faculty.get(section_name, set()))
        )
    
    # Build the final immutable faculty
    faculty: dict[FacultyName, Faculty] = {}
    for faculty_name in temp_faculty_sections:
        faculty[faculty_name] = Faculty(
            name=faculty_name,
            sections=frozenset(temp_faculty_sections[faculty_name])
        )
    
    # Build the final immutable TimetableData
    timetable = TimetableData(
        term_name=term_name,
        rooms=dict(temp_rooms),
        time_slots=dict(temp_time_slots),
        sections=sections,
        faculty=faculty,
        time_slot_conflicts=frozenset(temp_time_slot_conflicts),
        conflicts=frozenset(temp_conflicts),
        anti_conflicts=frozenset(temp_anti_conflicts),
        room_preferences=frozenset(temp_room_preferences),
        time_slot_preferences=frozenset(temp_time_slot_preferences),
        faculty_days_off=frozenset(temp_faculty_days_off),
        faculty_evenly_spread=frozenset(temp_faculty_evenly_spread),
        faculty_no_room_switch=frozenset(temp_faculty_no_room_switch),
        faculty_too_many_rooms=frozenset(temp_faculty_too_many_rooms),
        faculty_gap_too_short=frozenset(temp_faculty_gap_too_short),
        faculty_gap_too_long=frozenset(temp_faculty_gap_too_long),
        faculty_cluster_too_short=frozenset(temp_faculty_cluster_too_short),
        faculty_cluster_too_long=frozenset(temp_faculty_cluster_too_long),
        time_pattern_matches=frozenset(temp_time_pattern_matches)
    )
    
    # Close the connection
    conn.close()
    
    return timetable


def load_rooms(cursor: sqlite3.Cursor, temp_rooms: dict[RoomName, Room]) -> None:
    """Load rooms from the database."""
    cursor.execute("""
        SELECT DISTINCT room
        FROM rooms_used_by_departments
        ORDER BY building, CAST (room_number AS INTEGER), room_number
    """)
    
    for (room_name,) in cursor.fetchall():
        temp_rooms[room_name] = Room(name=room_name)


def load_time_slots(cursor: sqlite3.Cursor, temp_time_slots: dict[TimeSlotName, TimeSlot]) -> None:
    """Load time slots from the database."""
    cursor.execute("""
        SELECT DISTINCT time_slot, days, start_time, duration
        FROM time_slots_used_by_departments_materialized
        ORDER BY first_day, start_time, duration, duration * LENGTH(days)
    """)
    
    for (name, days_str, start_time_minutes, duration_minutes) in cursor.fetchall():
        days = Days.from_string(days_str)
        start_time = Time(minutes=start_time_minutes)
        duration = Duration(minutes=duration_minutes)
        
        temp_time_slots[name] = TimeSlot(
            name=name,
            days=days,
            start_time=start_time,
            duration=duration
        )


def load_time_slot_conflicts(
    cursor: sqlite3.Cursor, 
    temp_time_slots: dict[TimeSlotName, TimeSlot],
    temp_time_slot_conflicts: set[tuple[TimeSlotName, TimeSlotName]]
) -> None:
    """Load time slot conflicts from the database."""
    cursor.execute("""
        SELECT time_slot_a, time_slot_b
        FROM conflicting_time_slots
        JOIN time_slots_used_by_departments_materialized AS ts_a
            ON  time_slot_a = ts_a.time_slot
        JOIN time_slots_used_by_departments_materialized AS ts_b
            ON  time_slot_b = ts_b.time_slot
    """)
    
    for (time_slot_a, time_slot_b) in cursor.fetchall():
        if time_slot_a in temp_time_slots and time_slot_b in temp_time_slots:
            temp_time_slot_conflicts.add((time_slot_a, time_slot_b))
            temp_time_slot_conflicts.add((time_slot_b, time_slot_a))


def load_sections_and_time_slots(
    cursor: sqlite3.Cursor,
    temp_time_slots: dict[TimeSlotName, TimeSlot],
    temp_section_available_time_slots: dict[SectionName, set[TimeSlotName]],
    temp_section_time_slot_preferences: dict[SectionName, dict[TimeSlotName, Priority]],
    temp_time_slot_preferences: set[TimeSlotPreference]
) -> None:
    """Load sections and their available time slots."""
    cursor.execute("""
        SELECT DISTINCT section, time_slot, time_slot_priority
        FROM time_slots_available_to_sections_materialized
        ORDER BY section
    """)
    
    for (section_name, time_slot_name, priority) in cursor.fetchall():
        if time_slot_name not in temp_time_slots:
            continue
            
        # Add the time slot to the section's available time slots
        temp_section_available_time_slots[section_name].add(time_slot_name)
        
        # If there's a priority, add it to the section's time slot preferences
        if priority is not None:
            temp_section_time_slot_preferences[section_name][time_slot_name] = priority
            temp_time_slot_preferences.add(
                TimeSlotPreference(section=section_name, time_slot=time_slot_name, priority=priority)
            )


def load_sections_and_rooms(
    cursor: sqlite3.Cursor,
    temp_rooms: dict[RoomName, Room],
    temp_section_available_rooms: dict[SectionName, set[RoomName]],
    temp_section_room_preferences: dict[SectionName, dict[RoomName, Priority]],
    temp_room_preferences: set[RoomPreference]
) -> None:
    """Load rooms for sections."""
    cursor.execute("""
        SELECT DISTINCT section, room, room_priority
        FROM rooms_available_to_sections
        ORDER BY section
    """)
    
    for (section_name, room_name, priority) in cursor.fetchall():
        if room_name not in temp_rooms:
            continue
            
        # Add the room to the section's available rooms
        temp_section_available_rooms[section_name].add(room_name)
        
        # If there's a priority, add it to the section's room preferences
        if priority is not None:
            temp_section_room_preferences[section_name][room_name] = priority
            temp_room_preferences.add(
                RoomPreference(section=section_name, room=room_name, priority=priority)
            )


def load_faculty(
    cursor: sqlite3.Cursor,
    temp_faculty_sections: dict[FacultyName, set[SectionName]]
) -> None:
    """Load faculty from the database."""
    cursor.execute("""
        SELECT DISTINCT faculty
        FROM faculty_sections_to_be_scheduled
        ORDER BY faculty
    """)
    
    for (name,) in cursor.fetchall():
        # Just initialize the entry in the dictionary
        if name not in temp_faculty_sections:
            temp_faculty_sections[name] = set()


def load_conflicts(
    cursor: sqlite3.Cursor,
    temp_section_available_time_slots: dict[SectionName, set[TimeSlotName]],
    temp_conflicts: set[Conflict]
) -> None:
    """Load conflict pairs from the database."""
    cursor.execute("""
        SELECT DISTINCT section_a, section_b, priority
        FROM conflict_pairs_materialized
        WHERE section_a < section_b
        ORDER BY section_a, section_b
    """)
    
    for (section_a, section_b, priority) in cursor.fetchall():
        if section_a not in temp_section_available_time_slots or section_b not in temp_section_available_time_slots:
            continue
            
        # Add to conflicts list regardless of priority
        temp_conflicts.add(Conflict(
            sections=(section_a, section_b),
            priority=priority
        ))


def load_anti_conflicts(
    cursor: sqlite3.Cursor,
    temp_section_available_time_slots: dict[SectionName, set[TimeSlotName]],
    temp_anti_conflicts: set[AntiConflict]
) -> None:
    """Load anti-conflict pairs from the database."""
    cursor.execute("""
        SELECT DISTINCT single_section, group_section, priority
        FROM anti_conflict_pairs
        ORDER BY single_section, priority, group_section
    """)
    
    # Group anti-conflicts by single section and priority
    anti_conflict_groups: dict[tuple[SectionName, int], set[SectionName]] = defaultdict(set)
    
    for (single, group_section, priority) in cursor.fetchall():
        if single not in temp_section_available_time_slots or group_section not in temp_section_available_time_slots:
            continue
            
        # Add to the group
        anti_conflict_groups[(single, priority)].add(group_section)
    
    # Create AntiConflict objects from the groups
    for (single, priority), group in anti_conflict_groups.items():
        if group:  # Only add if there are sections in the group
            temp_anti_conflicts.add(AntiConflict(
                single=single,
                group=frozenset(group),
                priority=priority
            ))


def load_time_pattern_matches(
    cursor: sqlite3.Cursor,
    temp_section_available_time_slots: dict[SectionName, set[TimeSlotName]],
    temp_time_pattern_matches: set[TimePatternMatch]
) -> None:
    """Load time pattern match constraints from the database."""
    cursor.execute("""
        SELECT DISTINCT time_pattern_match_name, time_pattern_match_priority, time_pattern_match_section
        FROM time_pattern_matches
        NATURAL JOIN time_pattern_match_sections
        JOIN sections_to_be_scheduled
            ON time_pattern_match_section = section
        ORDER BY time_pattern_match_name, time_pattern_match_priority, time_pattern_match_section
    """)
    
    # Group sections by pattern name and priority
    pattern_groups: dict[tuple[str, int], set[SectionName]] = defaultdict(set)
    
    for (name, priority, section_str) in cursor.fetchall():
        assert(type(name) == str and type(priority) == int and type(section_str) == str)
        section = SectionName(section_str)
        if section not in temp_section_available_time_slots:
            continue
            
        # Add to the group
        pattern_groups[(name, priority)].add(section)
    
    # Create TimePatternMatch objects from the groups
    for (_, priority), sections in pattern_groups.items():
        if len(sections) > 1:  # Only add if there are at least 2 sections in the group
            temp_time_pattern_matches.add(TimePatternMatch(
                sections=frozenset(sections),
                priority=priority
            ))


def load_faculty_section_assignments(
    cursor: sqlite3.Cursor,
    temp_faculty_sections: dict[FacultyName, set[SectionName]],
    temp_section_faculty: dict[SectionName, set[FacultyName]]
) -> None:
    """Link faculty to their assigned sections."""
    cursor.execute("""
        SELECT DISTINCT faculty, section
        FROM faculty_sections_to_be_scheduled
        ORDER BY faculty, section
    """)
    
    for (faculty_name, section_name) in cursor.fetchall():
        # Add the section to the faculty's list
        temp_faculty_sections[faculty_name].add(section_name)
        
        # Add the faculty to the section's list
        temp_section_faculty[section_name].add(faculty_name)


def load_faculty_preferences(
    cursor: sqlite3.Cursor,
    temp_faculty_sections: dict[FacultyName, set[SectionName]],
    temp_section_available_rooms: dict[SectionName, set[RoomName]],
    temp_faculty_days_off: set[FacultyDaysOff],
    temp_faculty_evenly_spread: set[FacultyEvenlySpread],
    temp_faculty_no_room_switch: set[FacultyNoRoomSwitch],
    temp_faculty_too_many_rooms: set[FacultyTooManyRooms],
    temp_faculty_gap_too_short: set[FacultyGapTooShort],
    temp_faculty_gap_too_long: set[FacultyGapTooLong],
    temp_faculty_cluster_too_short: set[FacultyClusterTooShort],
    temp_faculty_cluster_too_long: set[FacultyClusterTooLong]
) -> None:
    """Load faculty preferences from the database."""
    # First, load general faculty preferences
    cursor.execute("""
        SELECT
            faculty, days_to_check,
            days_off, days_off_priority,
            evenly_spread_priority,
            no_room_switch_priority, too_many_rooms_priority,
            max_gap_within_cluster
        FROM faculty_to_be_scheduled_preference_intervals
        GROUP BY faculty
    """)

    faculty_max_gap = {}  # Store max gap for each faculty for later use

    for (faculty, days_to_check_str, days_off, days_off_priority,
         evenly_spread_priority, no_room_switch_priority, too_many_rooms_priority,
         max_gap_within_cluster) in cursor.fetchall():

        if faculty not in temp_faculty_sections:
            continue

        days_to_check = Days.from_string(days_to_check_str)
        max_gap = Duration(minutes=max_gap_within_cluster)
        faculty_max_gap[faculty] = max_gap

        # Days off preference
        if days_off is not None and days_off_priority is not None:
            temp_faculty_days_off.add(FacultyDaysOff(
                faculty=faculty,
                days_to_check=days_to_check,
                desired_days_off=days_off,
                priority=days_off_priority
            ))

        # Evenly spread preference
        if evenly_spread_priority is not None:
            temp_faculty_evenly_spread.add(FacultyEvenlySpread(
                faculty=faculty,
                days_to_check=days_to_check,
                priority=evenly_spread_priority
            ))

        # No room switch preference
        if no_room_switch_priority is not None:
            temp_faculty_no_room_switch.add(FacultyNoRoomSwitch(
                faculty=faculty,
                days_to_check=days_to_check,
                max_gap_within_cluster=max_gap,
                priority=no_room_switch_priority
            ))

        # Too many rooms preference - exact calculation
        if too_many_rooms_priority is not None:
            # Get the faculty's sections
            faculty_sections = list(temp_faculty_sections[faculty])
            section_count = len(faculty_sections)

            # Skip if only 0 or 1 sections
            if section_count <= 1:
                continue

            # Calculate the minimum possible number of rooms
            # This is an exact calculation that checks all possible combinations
            min_rooms = section_count  # Worst case: one room per section

            # Get all possible rooms for each section
            section_rooms = {}
            for section_name in faculty_sections:
                if section_name not in temp_section_available_rooms:
                    continue
                
                # Get rooms with no penalty for this section
                section_rooms[section_name] = [
                    room for room in temp_section_available_rooms[section_name]
                ]

            # Only proceed if all sections have available rooms
            if len(section_rooms) != section_count:
                continue

            # Get all possible rooms
            all_rooms = set()
            for rooms in section_rooms.values():
                all_rooms.update(rooms)

            # Try each possible number of rooms from 1 to section count
            for k in range(1, section_count):
                # Try each subset of this size
                for room_subset_tup in combinations(all_rooms, k):
                    room_subset = set(room_subset_tup)

                    # Check if this subset can accommodate all sections
                    all_sections_can_use = True
                    for section_name, rooms in section_rooms.items():
                        # If the section can't use any room in the subset, this subset won't work
                        if not any(room in room_subset for room in rooms):
                            all_sections_can_use = False
                            break

                    if all_sections_can_use:
                        min_rooms = k
                        break

                # If we found a valid number of rooms, we don't need to check larger counts
                if min_rooms == k:
                    break

            temp_faculty_too_many_rooms.add(FacultyTooManyRooms(
                faculty=faculty,
                desired_max_rooms=min_rooms,
                priority=too_many_rooms_priority
            ))

    # Now load distribution intervals
    cursor.execute("""
        SELECT
            faculty, days_to_check,
            is_cluster, is_too_short, interval_minutes, interval_priority
        FROM faculty_to_be_scheduled_preference_intervals
        WHERE is_cluster IS NOT NULL
        ORDER BY faculty, is_cluster, is_too_short, interval_minutes
    """)

    for (faculty, days_to_check_str, is_cluster, is_too_short, interval_minutes, interval_priority) in cursor.fetchall():
        if faculty not in temp_faculty_sections:
            continue

        days_to_check = Days.from_string(days_to_check_str)
        duration = Duration(minutes=interval_minutes)
        max_gap = faculty_max_gap.get(faculty, Duration(minutes=30))  # Default to 30 minutes if not found

        # Determine the interval type
        if is_cluster:
            if is_too_short:
                temp_faculty_cluster_too_short.add(FacultyClusterTooShort(
                    faculty=faculty,
                    days_to_check=days_to_check,
                    duration=duration,
                    max_gap_within_cluster=max_gap,
                    priority=interval_priority
                ))
            else:
                temp_faculty_cluster_too_long.add(FacultyClusterTooLong(
                    faculty=faculty,
                    days_to_check=days_to_check,
                    duration=duration,
                    max_gap_within_cluster=max_gap,
                    priority=interval_priority
                ))
        else:
            if is_too_short:
                temp_faculty_gap_too_short.add(FacultyGapTooShort(
                    faculty=faculty,
                    days_to_check=days_to_check,
                    duration=duration,
                    max_gap_within_cluster=max_gap,
                    priority=interval_priority
                ))
            else:
                temp_faculty_gap_too_long.add(FacultyGapTooLong(
                    faculty=faculty,
                    days_to_check=days_to_check,
                    duration=duration,
                    max_gap_within_cluster=max_gap,
                    priority=interval_priority
                ))

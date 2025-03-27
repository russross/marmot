"""
Input processing for the Marmot SAT-based timetabling system.

This module is responsible for loading data from the database into the
data structures defined in data.py.
"""
import sqlite3
from typing import List, Set, Tuple
from itertools import combinations

from data import (
    TimetableData, Days, Time, Duration, Room, TimeSlot, Section, Faculty,
    Conflict, AntiConflict, RoomPreference, TimeSlotPreference,
    FacultyDaysOff, FacultyEvenlySpread, FacultyNoRoomSwitch, FacultyTooManyRooms,
    FacultyDistributionInterval, TimePatternMatch, DistributionIntervalType,
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
    conn = sqlite3.connect(db_path)
    conn.execute("PRAGMA foreign_keys = ON")
    conn.execute("PRAGMA temp_store = memory")
    conn.execute("PRAGMA mmap_size = 100000000")
    
    # Create a cursor
    cursor = conn.cursor()
    
    # Get term name
    cursor.execute("SELECT term FROM terms")
    term_name = cursor.fetchone()[0]
    
    # Create the timetable data container
    timetable = TimetableData(term_name=term_name)
    
    # Load the basic entities
    load_rooms(cursor, timetable)
    load_time_slots(cursor, timetable)
    load_time_slot_conflicts(cursor, timetable)
    load_faculty(cursor, timetable)
    load_sections(cursor, timetable)
    load_conflicts(cursor, timetable)
    load_anti_conflicts(cursor, timetable)
    load_time_pattern_matches(cursor, timetable)
    load_faculty_section_assignments(cursor, timetable)
    load_faculty_preferences(cursor, timetable)
    
    # Close the connection
    conn.close()
    
    return timetable


def load_rooms(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
    """Load rooms from the database."""
    cursor.execute("""
        SELECT DISTINCT room
        FROM rooms_used_by_departments
        ORDER BY building, CAST (room_number AS INTEGER), room_number
    """)
    
    for (room_name,) in cursor.fetchall():
        timetable.rooms[room_name] = Room(name=room_name)


def load_time_slots(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
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
        
        timetable.time_slots[name] = TimeSlot(
            name=name,
            days=days,
            start_time=start_time,
            duration=duration
        )


# The database is the source of truth on whether or not two time slots conflict.
def load_time_slot_conflicts(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
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
        if time_slot_a in timetable.time_slots and time_slot_b in timetable.time_slots:
            timetable.time_slot_conflicts[(time_slot_a, time_slot_b)] = True
            timetable.time_slot_conflicts[(time_slot_b, time_slot_a)] = True


def load_faculty(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
    """Load faculty from the database."""
    cursor.execute("""
        SELECT DISTINCT faculty
        FROM faculty_sections_to_be_scheduled
        ORDER BY faculty
    """)
    
    for (name,) in cursor.fetchall():
        timetable.faculty[name] = Faculty(name=name)


def load_sections(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
    """Load sections and their available time slots and rooms."""
    # First, load sections and their time slots
    cursor.execute("""
        SELECT DISTINCT section, time_slot, time_slot_priority
        FROM time_slots_available_to_sections_materialized
        ORDER BY section
    """)
    
    for (section_name, time_slot_name, priority) in cursor.fetchall():
        # If this is a new section, create it
        if section_name not in timetable.sections:
            timetable.sections[section_name] = Section(name=section_name)
        
        # Add the time slot to the section's available time slots
        timetable.sections[section_name].available_time_slots.add(time_slot_name)
        
        # If there's a priority, add it to the section's time slot preferences
        if priority is not None:
            timetable.sections[section_name].time_slot_preferences[time_slot_name] = priority
            timetable.time_slot_preferences.append(
                TimeSlotPreference(section=section_name, time_slot=time_slot_name, priority=priority)
            )
    
    # Next, load rooms for sections
    cursor.execute("""
        SELECT DISTINCT section, room, room_priority
        FROM rooms_available_to_sections
        ORDER BY section
    """)
    
    for (section_name, room_name, priority) in cursor.fetchall():
        if section_name not in timetable.sections:
            # This shouldn't normally happen, but just in case
            continue
            
        # Add the room to the section's available rooms
        timetable.sections[section_name].available_rooms.add(room_name)
        
        # If there's a priority, add it to the section's room preferences
        if priority is not None:
            timetable.sections[section_name].room_preferences[room_name] = priority
            timetable.room_preferences.append(
                RoomPreference(section=section_name, room=room_name, priority=priority)
            )


def load_conflicts(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
    """Load conflict pairs from the database."""
    cursor.execute("""
        SELECT DISTINCT section_a, section_b, priority
        FROM conflict_pairs_materialized
        WHERE section_a < section_b
        ORDER BY section_a, section_b
    """)
    
    for (section_a, section_b, priority) in cursor.fetchall():
        if section_a not in timetable.sections or section_b not in timetable.sections:
            continue
            
        # Add to conflicts list regardless of priority
        timetable.conflicts.append(Conflict(
            sections=(section_a, section_b),
            priority=priority
        ))


def load_anti_conflicts(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
    """Load anti-conflict pairs from the database."""
    cursor.execute("""
        SELECT DISTINCT single_section, group_section, priority
        FROM anti_conflict_pairs
        ORDER BY single_section, priority, group_section
    """)
    
    current_single = None
    current_priority = -1
    current_group = set()
    
    for (single, group_section, priority) in cursor.fetchall():
        assert(type(single) == str and type(group_section) == str and type(priority) == int)
        if single not in timetable.sections or group_section not in timetable.sections:
            continue
            
        # If this is a new single section or priority, store the previous group and start a new one
        if single != current_single or priority != current_priority:
            if current_single is not None and current_group:
                timetable.anti_conflicts.append(AntiConflict(
                    single=current_single,
                    group=current_group,
                    priority=current_priority
                ))
            
            current_single = single
            current_priority = priority
            current_group = {group_section}
        else:
            current_group.add(group_section)
    
    # Don't forget to add the last group
    if current_single is not None and current_group:
        timetable.anti_conflicts.append(AntiConflict(
            single=current_single,
            group=current_group,
            priority=current_priority
        ))


def load_time_pattern_matches(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
    """Load time pattern match constraints from the database."""
    cursor.execute("""
        SELECT DISTINCT time_pattern_match_name, time_pattern_match_priority, time_pattern_match_section
        FROM time_pattern_matches
        NATURAL JOIN time_pattern_match_sections
        JOIN sections_to_be_scheduled
            ON time_pattern_match_section = section
        ORDER BY time_pattern_match_name, time_pattern_match_priority, time_pattern_match_section
    """)
    
    current_name = None
    current_priority = -1
    current_sections = set()
    
    for (name, priority, section) in cursor.fetchall():
        assert(type(name) == str and type(priority) == int and type(section) == str)
        if section not in timetable.sections:
            continue
            
        # If this is a new pattern name or priority, store the previous group and start a new one
        if name != current_name or priority != current_priority:
            if current_name is not None and len(current_sections) > 1:
                timetable.time_pattern_matches.append(TimePatternMatch(
                    sections=current_sections,
                    priority=current_priority
                ))
            
            current_name = name
            current_priority = priority
            current_sections = {section}
        else:
            current_sections.add(section)
    
    # Don't forget to add the last group
    if current_name is not None and len(current_sections) > 1:
        timetable.time_pattern_matches.append(TimePatternMatch(
            sections=current_sections,
            priority=current_priority
        ))


def load_faculty_section_assignments(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
    """Link faculty to their assigned sections."""
    cursor.execute("""
        SELECT DISTINCT faculty, section
        FROM faculty_sections_to_be_scheduled
        ORDER BY faculty, section
    """)
    
    for (faculty_name, section_name) in cursor.fetchall():
        if faculty_name not in timetable.faculty or section_name not in timetable.sections:
            continue
            
        # Add the section to the faculty's list
        timetable.faculty[faculty_name].sections.add(section_name)
        
        # Add the faculty to the section's list
        timetable.sections[section_name].faculty.add(faculty_name)

def load_faculty_preferences(cursor: sqlite3.Cursor, timetable: TimetableData) -> None:
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

        if faculty not in timetable.faculty:
            continue

        days_to_check = Days.from_string(days_to_check_str)
        max_gap = Duration(minutes=max_gap_within_cluster)
        faculty_max_gap[faculty] = max_gap

        # Days off preference
        if days_off is not None and days_off_priority is not None:
            timetable.faculty_days_off.append(FacultyDaysOff(
                faculty=faculty,
                days_to_check=days_to_check,
                desired_days_off=days_off,
                priority=days_off_priority
            ))

        # Evenly spread preference
        if evenly_spread_priority is not None:
            timetable.faculty_evenly_spread.append(FacultyEvenlySpread(
                faculty=faculty,
                days_to_check=days_to_check,
                priority=evenly_spread_priority
            ))

        # No room switch preference
        if no_room_switch_priority is not None:
            timetable.faculty_no_room_switch.append(FacultyNoRoomSwitch(
                faculty=faculty,
                days_to_check=days_to_check,
                max_gap_within_cluster=max_gap,
                priority=no_room_switch_priority
            ))

        # Too many rooms preference - exact calculation
        if too_many_rooms_priority is not None:
            # Get the faculty's sections
            faculty_sections = list(timetable.faculty[faculty].sections)
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
                section = timetable.sections[section_name]
                section_rooms[section_name] = [
                    room for room in section.available_rooms
                    if room not in section.room_preferences  # Only consider rooms with no penalty
                ]

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

            timetable.faculty_too_many_rooms.append(FacultyTooManyRooms(
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
        if faculty not in timetable.faculty:
            continue

        days_to_check = Days.from_string(days_to_check_str)
        duration = Duration(minutes=interval_minutes)
        max_gap = faculty_max_gap.get(faculty, Duration(minutes=30))  # Default to 30 minutes if not found

        # Determine the interval type
        if is_cluster:
            if is_too_short:
                interval_type = DistributionIntervalType.CLUSTER_TOO_SHORT
            else:
                interval_type = DistributionIntervalType.CLUSTER_TOO_LONG
        else:
            if is_too_short:
                interval_type = DistributionIntervalType.GAP_TOO_SHORT
            else:
                interval_type = DistributionIntervalType.GAP_TOO_LONG

        timetable.faculty_distribution_intervals.append(FacultyDistributionInterval(
            faculty=faculty,
            days_to_check=days_to_check,
            interval_type=interval_type,
            duration=duration,
            max_gap_within_cluster=max_gap,
            priority=interval_priority
        ))

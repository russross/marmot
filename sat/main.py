#!./.venv/bin/python3
"""
Main entry point for the Marmot SAT-based timetabling system.
Loads data, performs consistency checks, and solves the timetabling problem.
"""

import argparse
import sys
import time
from collections import Counter, defaultdict
from typing import Dict, List, Set, Tuple, Any, Optional, cast

from data import (
    TimetableData, Section, Room, TimeSlot, 
    Conflict, AntiConflict, RoomPreference, TimeSlotPreference,
    FacultyDaysOff, FacultyEvenlySpread, FacultyNoRoomSwitch, 
    FacultyTooManyRooms, FacultyDistributionInterval, TimePatternMatch,
    ConstraintType
)
from input import load_timetable_data
from search import solve_timetable
from print import print_schedule


def perform_consistency_checks(timetable: TimetableData) -> bool:
    """
    Verify data consistency and report any issues found.
    Returns True if all checks pass, False otherwise.
    """
    all_checks_passed = True
    
    # Check that all time slots referenced by sections exist
    for section_name, section in timetable.sections.items():
        for time_slot_name in section.available_time_slots:
            if time_slot_name not in timetable.time_slots:
                print(f"ERROR: Section {section_name} references non-existent time slot {time_slot_name}")
                all_checks_passed = False
    
    # Check that all rooms referenced by sections exist
    for section_name, section in timetable.sections.items():
        for room_name in section.available_rooms:
            if room_name not in timetable.rooms:
                print(f"ERROR: Section {section_name} references non-existent room {room_name}")
                all_checks_passed = False
    
    # Check that all faculty referenced by sections exist
    for section_name, section in timetable.sections.items():
        for faculty_name in section.faculty:
            if faculty_name not in timetable.faculty:
                print(f"ERROR: Section {section_name} references non-existent faculty {faculty_name}")
                all_checks_passed = False
    
    # Check that all sections referenced by faculty exist
    for faculty_name, faculty in timetable.faculty.items():
        for section_name in faculty.sections:
            if section_name not in timetable.sections:
                print(f"ERROR: Faculty {faculty_name} references non-existent section {section_name}")
                all_checks_passed = False
    
    # Check that all sections have at least one available time slot
    for section_name, section in timetable.sections.items():
        if not section.available_time_slots:
            print(f"ERROR: Section {section_name} has no available time slots")
            all_checks_passed = False
    
    # Check that all sections have at least one available room
    for section_name, section in timetable.sections.items():
        if not section.available_rooms:
            print(f"WARNING: Section {section_name} has no available rooms")
    
    # Make sure time slot conflicts are symmetric
    for (ts1, ts2), conflicts in timetable.time_slot_conflicts.items():
        if conflicts and (ts2, ts1) not in timetable.time_slot_conflicts:
            print(f"ERROR: Time slot conflict ({ts1}, {ts2}) is not symmetric")
            all_checks_passed = False
    
    # Check constraints reference valid sections
    for conflict in timetable.conflicts:
        for section_name in conflict.sections:
            if section_name not in timetable.sections:
                print(f"ERROR: Conflict references non-existent section {section_name}")
                all_checks_passed = False
    
    # Check anti-conflicts
    for anti_conflict in timetable.anti_conflicts:
        if anti_conflict.single not in timetable.sections:
            print(f"ERROR: Anti-conflict references non-existent section {anti_conflict.single}")
            all_checks_passed = False
        
        for section_name in anti_conflict.group:
            if section_name not in timetable.sections:
                print(f"ERROR: Anti-conflict references non-existent section {section_name}")
                all_checks_passed = False
    
    # Check time pattern matches
    for pattern_match in timetable.time_pattern_matches:
        for section_name in pattern_match.sections:
            if section_name not in timetable.sections:
                print(f"ERROR: Time pattern match references non-existent section {section_name}")
                all_checks_passed = False
    
    # Check faculty preferences
    for days_off in timetable.faculty_days_off:
        if days_off.faculty not in timetable.faculty:
            print(f"ERROR: Faculty days off preference references non-existent faculty {days_off.faculty}")
            all_checks_passed = False
    
    for evenly_spread in timetable.faculty_evenly_spread:
        if evenly_spread.faculty not in timetable.faculty:
            print(f"ERROR: Faculty evenly spread preference references non-existent faculty {evenly_spread.faculty}")
            all_checks_passed = False
    
    for no_room_switch in timetable.faculty_no_room_switch:
        if no_room_switch.faculty not in timetable.faculty:
            print(f"ERROR: Faculty no room switch preference references non-existent faculty {no_room_switch.faculty}")
            all_checks_passed = False
    
    for too_many_rooms in timetable.faculty_too_many_rooms:
        if too_many_rooms.faculty not in timetable.faculty:
            print(f"ERROR: Faculty too many rooms preference references non-existent faculty {too_many_rooms.faculty}")
            all_checks_passed = False
    
    for interval in timetable.faculty_distribution_intervals:
        if interval.faculty not in timetable.faculty:
            print(f"ERROR: Faculty distribution interval references non-existent faculty {interval.faculty}")
            all_checks_passed = False
    
    return all_checks_passed


def print_data_summary(timetable: TimetableData) -> None:
    """Print a compact summary of the timetable data."""
    print(f"\n=== TIMETABLE SUMMARY FOR TERM: {timetable.term_name} ===")
    
    # Basic counts
    print(f"\nBasic entities:")
    print(f"  Rooms: {len(timetable.rooms)}")
    print(f"  Time slots: {len(timetable.time_slots)}")
    print(f"  Sections: {len(timetable.sections)}")
    print(f"  Faculty: {len(timetable.faculty)}")
    
    # Time slot patterns
    time_patterns = Counter(ts.time_pattern for ts in timetable.time_slots.values())
    print(f"\nTime slot patterns (# days, duration):")
    for (days, duration), count in time_patterns.most_common():
        print(f"  {days} days, {duration}: {count} time slots")
    
    # Room statistics
    print("\nRoom availability:")
    room_usage: Counter[str] = Counter()
    for section in timetable.sections.values():
        room_usage.update(section.available_rooms)
    
    print(f"  Most used rooms (top 5):")
    for room, count in room_usage.most_common(5):
        print(f"    {room}: available for {count} sections")
    
    print(f"  Least used rooms (bottom 5):")
    for room, count in room_usage.most_common()[:-6:-1]:
        print(f"    {room}: available for {count} sections")
    
    # Faculty statistics
    print("\nFaculty statistics:")
    faculty_section_counts = {f.name: len(f.sections) for f in timetable.faculty.values()}
    avg_sections = sum(faculty_section_counts.values()) / len(faculty_section_counts) if faculty_section_counts else 0
    print(f"  Average sections per faculty: {avg_sections:.2f}")
    
    section_faculty_counts = Counter(len(s.faculty) for s in timetable.sections.values())
    print(f"\nSections with faculty count:")
    for count, num_sections in sorted(section_faculty_counts.items()):
        print(f"  {count} faculty: {num_sections} sections")
    
    # Constraint statistics by priority
    constraints_by_priority: Dict[int, Counter[str]] = defaultdict(Counter)
    
    for conflict in timetable.conflicts:
        constraints_by_priority[conflict.priority]["conflicts"] += 1
    
    for anticonflict in timetable.anti_conflicts:
        constraints_by_priority[anticonflict.priority]["anti_conflicts"] += 1
    
    for roompref in timetable.room_preferences:
        constraints_by_priority[roompref.priority]["room_preferences"] += 1
    
    for timepref in timetable.time_slot_preferences:
        constraints_by_priority[timepref.priority]["time_slot_preferences"] += 1
    
    for daysoff in timetable.faculty_days_off:
        constraints_by_priority[daysoff.priority]["faculty_days_off"] += 1
    
    for even in timetable.faculty_evenly_spread:
        constraints_by_priority[even.priority]["faculty_evenly_spread"] += 1
    
    for switch in timetable.faculty_no_room_switch:
        constraints_by_priority[switch.priority]["faculty_no_room_switch"] += 1
    
    for toomany in timetable.faculty_too_many_rooms:
        constraints_by_priority[toomany.priority]["faculty_too_many_rooms"] += 1
    
    for interval in timetable.faculty_distribution_intervals:
        constraints_by_priority[interval.priority]["faculty_distribution_intervals"] += 1
    
    for patterns in timetable.time_pattern_matches:
        constraints_by_priority[patterns.priority]["time_pattern_matches"] += 1
    
    print("\nConstraints by priority:")
    for priority in sorted(constraints_by_priority.keys()):
        total = sum(constraints_by_priority[priority].values())
        print(f"  Priority {priority}: {total} total constraints")
        for constraint_type, count in sorted(constraints_by_priority[priority].items()):
            print(f"    {constraint_type}: {count}")
    
    # Sample time slot details
    if timetable.time_slots:
        sample_time_slot = next(iter(timetable.time_slots.values()))
        print(f"\nSample time slot ({sample_time_slot.name}):")
        print(f"  Days: {sample_time_slot.days}")
        print(f"  Start time: {sample_time_slot.start_time}")
        print(f"  Duration: {sample_time_slot.duration}")
    
    # Sample room details
    if timetable.rooms:
        sample_room = next(iter(timetable.rooms.values()))
        print(f"\nSample room: {sample_room.name}")
    
    # Sample conflict
    if timetable.conflicts:
        sample_conflict = timetable.conflicts[0]
        print(f"\nSample conflict (priority {sample_conflict.priority}):")
        print(f"  Sections: {sample_conflict.sections[0]} and {sample_conflict.sections[1]}")
    
    # Sample anti-conflict
    if timetable.anti_conflicts:
        sample_anti = timetable.anti_conflicts[0]
        print(f"\nSample anti-conflict (priority {sample_anti.priority}):")
        print(f"  Single: {sample_anti.single}")
        print(f"  Group: {', '.join(list(sample_anti.group)[:3])}{'...' if len(sample_anti.group) > 3 else ''}")


def main() -> None:
    """Main entry point for the Marmot SAT timetabling system."""
    parser = argparse.ArgumentParser(description="Marmot SAT-based timetabling system")
    parser.add_argument("db_path", nargs="?", default="timetable.db", help="Path to the timetable database (default: timetable.db)")
    parser.add_argument("--solver", default="cd", choices=["cd", "g3", "g4", "m22", "mgh"], 
                        help="SAT solver to use: cd (Cadical), g3/g4 (Glucose), m22/mgh (MiniSat) (default: cd)")
    parser.add_argument("--time-limit", type=int, default=3600, 
                        help="Time limit in seconds (default: 3600)")
    parser.add_argument("--summary-only", action="store_true", 
                        help="Only print data summary without solving")
    parser.add_argument("--limit-priority", type=int, 
                        help="Limit solving to up to this priority level")
    parser.add_argument("--quiet", action="store_true",
                        help="Run with minimal output")
    args = parser.parse_args()
    
    print(f"Loading timetable data from {args.db_path}...")
    
    try:
        timetable = load_timetable_data(args.db_path)
        print(f"Successfully loaded timetable data for term: {timetable.term_name}")
        
        print("\nPerforming consistency checks...")
        all_checks_passed = perform_consistency_checks(timetable)
        
        if all_checks_passed:
            print("All consistency checks passed!")
        else:
            print("WARNING: Some consistency checks failed!")
        
        print_data_summary(timetable)
        
        if args.summary_only:
            print("\nSummary-only mode, skipping solving.")
            return
        
        # Solve the timetable
        print(f"\nSolving timetable with {args.solver} solver (time limit: {args.time_limit}s)...")
        start_time = time.time()
        
        schedule = solve_timetable(
            timetable=timetable,
            solver_name=args.solver,
            max_time_seconds=args.time_limit,
            verbose=True
        )
        
        solve_time = time.time() - start_time
        print(f"\nSolving completed in {solve_time:.2f} seconds")
        
        if schedule:
            print("\nFound schedule:")
            print_schedule(timetable, schedule)
        else:
            print("\nNo feasible schedule found!")
            
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()

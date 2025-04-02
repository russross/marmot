#!./.venv/bin/python3
"""
Main entry point for the Marmot SAT-based timetabling system.
Loads data, performs consistency checks, and solves the timetabling problem.
"""

import argparse
import sys
import time
from collections import Counter, defaultdict
from typing import Any, Optional, cast
import encoders

from data import (
    TimetableData, Section, Room, TimeSlot, 
    Conflict, AntiConflict, RoomPreference, TimeSlotPreference,
    FacultyDaysOff, FacultyEvenlySpread, FacultyNoRoomSwitch,  FacultyTooManyRooms,
    FacultyGapTooShort, FacultyGapTooLong, FacultyClusterTooShort, FacultyClusterTooLong,
    TimePatternMatch, ConstraintType
)
from input import load_timetable_data
from search import solve_timetable
from print import print_schedule
from save import save_schedule

def print_data_summary(timetable: TimetableData) -> None:
    """Print a detailed summary of the timetable data."""
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
    constraints_by_priority: dict[int, Counter[str]] = defaultdict(Counter)
    
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
    
    for gapshort in timetable.faculty_gap_too_short:
        constraints_by_priority[gapshort.priority]["faculty_gap_too_short"] += 0
    
    for gaplong in timetable.faculty_gap_too_long:
        constraints_by_priority[gaplong.priority]["faculty_gap_too_long"] += 0
    
    for clustershort in timetable.faculty_cluster_too_short:
        constraints_by_priority[clustershort.priority]["faculty_cluster_too_short"] += 0
    
    for clusterlong in timetable.faculty_cluster_too_long:
        constraints_by_priority[clusterlong.priority]["faculty_cluster_too_long"] += 0
    
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
        sample_conflict = list(timetable.conflicts)[0]
        print(f"\nSample conflict (priority {sample_conflict.priority}):")
        print(f"  Sections: {sample_conflict.sections[0]} and {sample_conflict.sections[1]}")
    
    # Sample anti-conflict
    if timetable.anti_conflicts:
        sample_anti = list(timetable.anti_conflicts)[0]
        print(f"\nSample anti-conflict (priority {sample_anti.priority}):")
        print(f"  Single: {sample_anti.single}")
        print(f"  Group: {', '.join(list(sample_anti.group)[:3])}{'...' if len(sample_anti.group) > 3 else ''}")


def main() -> None:
    """Main entry point for the Marmot SAT timetabling system."""
    parser = argparse.ArgumentParser(description="Marmot SAT-based timetabling system")
    parser.add_argument("db_path", nargs="?", default="../data/timetable.db", help="Path to the timetable database (default: ../data/timetable.db)")
    parser.add_argument("--solver", default="cd", choices=["cd", "g3", "g4", "m22", "mgh"], 
                        help="SAT solver to use: cd (Cadical), g3/g4 (Glucose), m22/mgh (MiniSat) (default: cd)")
    parser.add_argument("--time-limit", type=int, default=3600, 
                        help="Time limit in seconds (default: 3600)")
    parser.add_argument("--summary-only", action="store_true", 
                        help="Only print data summary without solving")
    parser.add_argument("--limit-priority", type=int, 
                        help="Limit solving to up to this priority level")
    parser.add_argument("--verbose", action="store_true",
                        help="Print detailed output")
    args = parser.parse_args()
    
    try:
        # Load timetable data
        timetable = load_timetable_data(args.db_path)
        
        # Print a concise summary of the loaded data
        constraints = timetable.get_all_constraints()
        priorities = sorted({c.priority for c in constraints})
        
        print(f"Loaded {timetable.term_name}: {len(timetable.sections)} sections, {len(timetable.rooms)} rooms, "
              f"{len(timetable.time_slots)} time slots, {len(timetable.faculty)} faculty, "
              f"{len(constraints)} constraints across {len(priorities)} priority levels")
        
        # Print detailed data summary if requested
        if args.verbose or args.summary_only:
            print_data_summary(timetable)
        
        if args.summary_only:
            return
        
        # Solve the timetable
        print(f"\nGenerating schedule using {args.solver} solver (time limit: {args.time_limit}s)...")
        start_time = time.time()
        
        schedule = solve_timetable(
            timetable=timetable,
            solver_name=args.solver,
            max_time_seconds=args.time_limit,
            verbose=args.verbose
        )
        
        solve_time = time.time() - start_time
        print(f"Solving completed in {solve_time:.2f} seconds")
        
        if schedule:
            save_schedule(args.db_path, timetable, schedule, 'Generated by SAT solver')
            print_schedule(timetable, schedule)
            print(f"Saved schedule with placement ID: {placement_id}")
        else:
            print("\nNo feasible schedule found!")
            
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()

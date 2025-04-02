# save.py
"""
Save utility for schedule storage in the database.

This module provides functions to save a generated schedule to the database.
"""
import sqlite3
from typing import Optional, Set, Tuple, Dict
from datetime import datetime

from data import TimetableData, SectionName, RoomName, TimeSlotName, Priority
from encoding import Placement


def sortable_score(problems: Set[Tuple[int, str]]) -> str:
    """
    Generate a sortable string representation of the score.
    
    Format: <<99×00,98×00,...>> where first number is inverted priority level
    and second is the count of violations at that level.
    Lower values sort first, so better scores come earlier.
    
    Args:
        problems: Set of (priority, description) tuples
        
    Returns:
        String representation for sorting scores
    """
    # Count problems by priority level
    counts_by_priority: Dict[int, int] = {}
    for priority, _ in problems:
        counts_by_priority[priority] = counts_by_priority.get(priority, 0) + 1
    
    if not counts_by_priority:
        return "<<00:00>>"
    
    parts = []
    for priority, count in sorted(counts_by_priority.items()):
        parts.append(f"{99-priority:02}×{count:02}")
    
    return "<<" + ",".join(parts) + ">>"


def score_string(problems: Set[Tuple[int, str]]) -> str:
    """
    Generate a human-readable string representation of the score.
    
    Format: <0×1,5×2> where the first number is priority level and 
    the second is the count of violations at that level.
    
    Args:
        problems: Set of (priority, description) tuples
        
    Returns:
        String representation of the score
    """
    # Count problems by priority level
    counts_by_priority: Dict[int, int] = {}
    for priority, _ in problems:
        counts_by_priority[priority] = counts_by_priority.get(priority, 0) + 1
    
    if not counts_by_priority:
        return "zero"
    
    parts = []
    for priority, count in sorted(counts_by_priority.items()):
        parts.append(f"{priority}×{count}")
    
    return "<" + ",".join(parts) + ">"


def save_schedule(
    db_path: str,
    timetable: TimetableData,
    schedule: Tuple[Placement, Set[Tuple[int, str]]],
    comment: str = ""
) -> int:
    """
    Save a schedule to the database.
    
    Args:
        db_path: Path to the SQLite database
        timetable: The timetable data
        schedule: Tuple of (placement, problems) where placement maps sections to 
                 (room, time_slot) pairs and problems is a set of (priority, msg) tuples
        comment: Optional comment to store with the schedule
        
    Returns:
        The ID of the newly created placement record
    """
    placement, problems = schedule
    
    # Connect to the database
    conn = sqlite3.connect(db_path)
    conn.execute("PRAGMA foreign_keys = ON")
    conn.execute("PRAGMA busy_timeout = 10000")
    
    try:
        # Begin transaction
        conn.execute("BEGIN")
        
        # Create the base record
        score = score_string(problems)
        sort_score = sortable_score(problems)
        current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        
        cursor = conn.execute(
            """
            INSERT INTO placements (score, sort_score, comment, created_at, modified_at)
            VALUES (?, ?, ?, ?, ?)
            """,
            (score, sort_score, comment, current_time, current_time)
        )
        
        # Get the new placement ID
        placement_id = cursor.lastrowid
        assert(type(placement_id) == int)
        
        # Store all section assignments
        for section_name, (room_name, time_slot_name) in placement.items():
            conn.execute(
                """
                INSERT INTO placement_sections (placement_id, section, time_slot, room)
                VALUES (?, ?, ?, ?)
                """,
                (placement_id, section_name, time_slot_name, room_name)
            )
        
        # Store all problems/penalties
        for priority, msg in problems:
            conn.execute(
                """
                INSERT INTO placement_penalties (placement_id, priority, message)
                VALUES (?, ?, ?) 
                """,
                (placement_id, priority, msg)
            )
        
        # Commit the transaction
        conn.execute("COMMIT")
        print(f"Saved schedule with placement ID: {placement_id}")
        
        return placement_id
    
    except Exception as e:
        # Roll back the transaction on error
        conn.execute("ROLLBACK")
        raise e
    
    finally:
        # Close the connection
        conn.close()

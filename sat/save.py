# save.py
"""
Save utility for schedule storage in the database.

This module provides functions to save a generated schedule to the database.
"""
import sqlite3
from typing import Optional
from datetime import datetime

from data import TimetableData, SectionName, RoomName, TimeSlotName, Schedule, Placement

def save_schedule(
    db_path: str,
    timetable: TimetableData,
    schedule: Schedule,
    comment: str = ""
) -> int:
    """
    Save a schedule to the database.
    
    Args:
        db_path: Path to the SQLite database
        timetable: The timetable data
        schedule: Schedule object containing placements and score information
        comment: Optional comment to store with the schedule
        
    Returns:
        The ID of the newly created placement record
    """
    # Connect to the database
    conn = sqlite3.connect(db_path)
    conn.execute("PRAGMA foreign_keys = ON")
    conn.execute("PRAGMA busy_timeout = 10000")
    
    try:
        # Begin transaction
        conn.execute("BEGIN")
        
        # Create the base record
        score = str(schedule.score)  # Use Score.__str__
        sort_score = schedule.score.sortable()  # Use Score.sortable
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
        for section_name, placement in schedule.placements.items():
            conn.execute(
                """
                INSERT INTO placement_sections (placement_id, section, time_slot, room)
                VALUES (?, ?, ?, ?)
                """,
                (placement_id, section_name, placement.time_slot, placement.room)
            )
        
        # Store all problems/penalties
        for priority, msg in schedule.problems:
            conn.execute(
                """
                INSERT INTO placement_penalties (placement_id, priority, message)
                VALUES (?, ?, ?) 
                """,
                (placement_id, priority, msg)
            )
        
        # Commit the transaction
        conn.execute("COMMIT")
        
        return placement_id
    
    except Exception as e:
        # Roll back the transaction on error
        conn.execute("ROLLBACK")
        raise e
    
    finally:
        # Close the connection
        conn.close()


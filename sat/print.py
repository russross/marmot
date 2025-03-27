# printschedule.py
"""
Printing utilities for schedule visualization.

This module provides functions to print a schedule in a grid format 
similar to the format used in the Rust version.
"""
from typing import Dict, List, Optional, Tuple, Set
import textwrap

from data import TimetableData

# Type alias for schedule representation
Schedule = Dict[str, Tuple[Optional[str], str]]


def print_schedule(timetable: TimetableData, schedule: Schedule) -> None:
    """
    Print the schedule in a grid format.
    
    Args:
        timetable: The timetable data
        schedule: The schedule mapping sections to (room, time_slot) pairs
    """
    # Extract rooms and time slots used in the schedule
    used_rooms: Set[str] = set()
    used_time_slots: Set[str] = set()
    
    for section, (room, time_slot) in schedule.items():
        if room is not None:
            used_rooms.add(room)
        used_time_slots.add(time_slot)
    
    # Convert to sorted lists
    rooms_list = sorted(list(used_rooms))
    time_slots_list = sorted(list(used_time_slots))
    
    # Create grid - first dimension is time slots, second is rooms
    # Each cell contains (section_name, faculty_name)
    grid = [[("", "") for _ in range(len(rooms_list) + 1)] for _ in range(len(time_slots_list) + 1)]
    
    # Fill in the headers
    for i, room in enumerate(rooms_list):
        grid[0][i + 1] = (room, "")
    
    for i, time_slot in enumerate(time_slots_list):
        grid[i + 1][0] = (time_slot, "")
    
    # Fill in the schedule
    for section, (room, time_slot) in schedule.items():
        if room is None:
            continue  # Skip sections without rooms
            
        room_idx = rooms_list.index(room) + 1
        time_idx = time_slots_list.index(time_slot) + 1
        
        # Get faculty for this section
        faculty_list = list(timetable.sections[section].faculty)
        faculty_name = ""
        if len(faculty_list) == 1:
            faculty_name = faculty_list[0]
        elif len(faculty_list) > 1:
            faculty_name = f"{faculty_list[0]}+"
        
        grid[time_idx][room_idx] = (section, faculty_name)
    
    # Determine column width based on the longest entry
    width = 1  # Minimum width
    for row in grid:
        for section_name, faculty_name in row:
            width = max(width, len(section_name), len(faculty_name))
    
    width += 2  # Add padding
    
    # Print the grid
    for i, row in enumerate(grid):
        # Print divider
        divider = "+"
        for _ in row:
            divider += "-" * width + "+"
        print(divider)
        
        # Print section names
        section_line = "|"
        for section_name, _ in row:
            section_line += f" {section_name:<{width-1}}|"
        print(section_line)
        
        # Print faculty names
        faculty_line = "|"
        for _, faculty_name in row:
            faculty_line += f" {faculty_name:<{width-1}}|"
        print(faculty_line)
    
    # Print final divider
    divider = "+"
    for _ in grid[0]:
        divider += "-" * width + "+"
    print(divider)
    
    # Print sections with time slots but no rooms
    for section, (room, time_slot) in schedule.items():
        if room is None:
            print(f"{section} at {time_slot} with no room")


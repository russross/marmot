# printschedule.py
"""
Printing utilities for schedule visualization.

This module provides functions to print a schedule in a grid format 
similar to the format used in the Rust version.
"""
from data import TimetableData, Schedule, Placement

def print_schedule(timetable: TimetableData, schedule: Schedule) -> None:
    """
    Print the schedule in a grid format.
    
    Args:
        timetable: The timetable data
        schedule: Schedule object containing placements and problems
    """
    # Extract rooms and time slots used in the placement
    used_rooms = { placement.room for placement in schedule.placements.values() 
                  if placement.room is not None }
    used_time_slots = { placement.time_slot for placement in schedule.placements.values() }
    
    # Convert to sorted lists
    rooms_list = sorted(list(used_rooms))
    time_slots_list = sorted(list(used_time_slots))
    
    # Create grid - first dimension is time slots, second is rooms
    # Each cell contains (section_name, faculty_name)
    grid = [[("", "") for _ in range(len(rooms_list) + 1)] for _ in range(len(time_slots_list) + 1)]
    
    # Fill in the headers
    for i, room_name in enumerate(rooms_list):
        grid[0][i + 1] = (room_name, "")
    
    for i, time_slot in enumerate(time_slots_list):
        grid[i + 1][0] = (time_slot, "")
    
    # Fill in the schedule
    for section, placement in schedule.placements.items():
        if placement.room is None:
            continue  # Skip sections without rooms
            
        room_idx = rooms_list.index(placement.room) + 1
        time_idx = time_slots_list.index(placement.time_slot) + 1
        
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
    for section, placement in schedule.placements.items():
        if placement.room is None:
            print(f"{section} at {placement.time_slot} with no room")

    for (priority, msg) in sorted(schedule.problems):
        print(f'{priority:2}: {msg}')

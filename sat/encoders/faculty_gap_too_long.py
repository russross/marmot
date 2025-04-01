# encoders/faculty_gap_too_long.py
"""
Faculty gap too long constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty gap too long constraint:
ensuring faculty don't have gaps between teaching clusters that exceed a specified duration.
"""

from data import TimetableData, Priority
from data import FacultyGapTooLong, Time, Duration, Day
from encoding import Encoding
from encoders.helpers import encode_faculty_cluster_helper

def encode_faculty_gap_too_long(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: FacultyGapTooLong
) -> None:
    """
    Encode a single faculty gap too long constraint.
    
    A faculty gap too long constraint specifies that a faculty member should not
    have gaps between teaching clusters that exceed a specified duration.
    
    This function creates callback functions for violation detection and description
    generation, then delegates to a helper function that handles the common encoding
    structure for faculty cluster constraints.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    max_gap_duration = constraint.duration
    max_gap_within_cluster = constraint.max_gap_within_cluster

    # Validate specific inputs for this constraint type
    assert max_gap_duration.minutes > 0, f"Non-positive maximum gap duration for faculty {faculty}"
    
    # Create a function that detects "too long" gaps
    def count_too_long_gaps(clusters: list[tuple[Time, Time]], day: Day) -> int:
        """
        Count gaps between clusters that exceed the maximum duration.
        
        Args:
            clusters: List of (start_time, end_time) tuples representing time clusters
            day: The day being analyzed (not used in this detector but included for API consistency)
            
        Returns:
            Number of gaps that exceed max_gap_duration
        """
        violation_count = 0
        
        # Need at least 2 clusters to have a gap
        if len(clusters) < 2:
            return 0
            
        # Check each gap between clusters
        for i in range(1, len(clusters)):
            prev_end_time = clusters[i-1][1]
            curr_start_time = clusters[i][0]
            
            # Calculate the gap duration
            gap_duration = curr_start_time - prev_end_time
            assert(type(gap_duration) == Duration)
            
            # Check if this gap exceeds the maximum allowed duration
            if gap_duration > max_gap_duration:
                violation_count += 1
        
        return violation_count
    
    # Create a function that generates descriptions for violations
    def generate_too_long_gap_description(i: int, day: Day) -> str:
        """
        Generate a description for a specific violation.
        
        Args:
            i: The violation number (1-based)
            day: The day of the violation
            
        Returns:
            A human-readable description of the violation
        """
        if i == 1:
            return f"has a gap longer than {max_gap_duration} on {day}"
        elif i == 2:
            return f"has a 2nd long gap on {day}"
        elif i == 3:
            return f"has a 3rd long gap on {day}"
        else:
            return f"has a {i}th long gap on {day}"
    
    # Use the helper function to handle the encoding
    encode_faculty_cluster_helper(
        timetable=timetable,
        encoding=encoding,
        faculty=faculty,
        days=days,
        max_gap=max_gap_within_cluster,
        priority=priority,
        violation_counter=count_too_long_gaps,
        description_generator=generate_too_long_gap_description
    )

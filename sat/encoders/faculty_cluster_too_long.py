# encoders/faculty_cluster_too_long.py
"""
Faculty cluster too long constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty cluster too long constraint:
ensuring faculty don't have continuous teaching blocks that exceed a specified duration.
"""

from data import TimetableData, Priority
from data import FacultyClusterTooLong, Time, Duration, Day
from encoding import Encoding
from encoders.helpers import encode_faculty_cluster_helper

def encode_faculty_cluster_too_long(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: FacultyClusterTooLong
) -> None:
    """
    Encode a single faculty cluster too long constraint.
    
    A faculty cluster too long constraint specifies that a faculty member should not
    have continuous teaching blocks (clusters) that exceed a specified duration.
    
    This function creates callback functions for violation detection and description
    generation, then delegates to a helper function that handles the common encoding
    structure for faculty cluster constraints.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    max_duration = constraint.duration
    max_gap = constraint.max_gap_within_cluster

    # Validate specific inputs for this constraint type
    assert max_duration.minutes > 0, f"Non-positive maximum duration for faculty {faculty}"
    
    # Create a function that detects "too long" clusters
    def count_too_long_clusters(clusters: list[tuple[Time, Time]], day: Day) -> int:
        """
        Count clusters that exceed the maximum duration.
        
        Args:
            clusters: List of (start_time, end_time) tuples representing time clusters
            day: The day being analyzed (not used in this detector but included for API consistency)
            
        Returns:
            Number of clusters that exceed max_duration
        """
        violation_count = 0
        
        for start_time, end_time in clusters:
            # Calculate the duration of this cluster
            cluster_duration = end_time - start_time
            assert(type(cluster_duration) == Duration)
            
            # Check if this cluster exceeds the maximum allowed duration
            if cluster_duration > max_duration:
                violation_count += 1
        
        return violation_count
    
    # Create a function that generates descriptions for violations
    def generate_too_long_description(i: int, day: Day) -> str:
        """
        Generate a description for a specific violation.
        
        Args:
            i: The violation number (1-based)
            day: The day of the violation
            
        Returns:
            A human-readable description of the violation
        """
        if i == 1:
            return f"has a teaching cluster longer than {max_duration} on {day}"
        elif i == 2:
            return f"has a 2nd long teaching cluster on {day}"
        elif i == 3:
            return f"has a 3rd long teaching cluster on {day}"
        else:
            return f"has a {i}th long teaching cluster on {day}"
    
    # Use the helper function to handle the encoding
    encode_faculty_cluster_helper(
        timetable=timetable,
        encoding=encoding,
        faculty=faculty,
        days=days,
        max_gap=max_gap,
        priority=priority,
        violation_counter=count_too_long_clusters,
        description_generator=generate_too_long_description
    )


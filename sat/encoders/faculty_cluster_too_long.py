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
    
    This function checks for any higher-priority constraints of the same type and
    adjusts the current constraint to avoid double-counting violations.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    max_duration = constraint.duration
    max_gap = constraint.max_gap_within_cluster

    # Find any constraints of the same type for this faculty
    same_faculty_constraints = [
        c for c in timetable.faculty_cluster_too_long
        if c.faculty == faculty
    ]
    
    # Ensure there are no duplicate constraints at the same priority level
    same_priority_constraints = [
        c for c in same_faculty_constraints
        if c.priority == priority and c is not constraint
    ]
    assert not same_priority_constraints, f"Multiple faculty_cluster_too_long constraints for {faculty} at priority {priority}"
    
    # Find higher-priority constraints of the same type for this faculty
    higher_priority_constraints = [
        c for c in same_faculty_constraints
        if c.priority < priority
    ]
    
    # Get the durations of higher-priority constraints (if any)
    higher_priority_durations = [c.duration for c in higher_priority_constraints]
    
    # Validate specific inputs for this constraint type
    assert max_duration.minutes > 0, f"Non-positive maximum duration for faculty {faculty}"
    
    # Create a function that detects "too long" clusters, only counting those
    # that aren't already covered by higher-priority constraints
    def count_too_long_clusters(clusters: list[tuple[Time, Time]], day: Day) -> int:
        """
        Count clusters that exceed the maximum duration for this constraint,
        but don't exceed the maximum duration of any higher-priority constraint.
        
        Args:
            clusters: List of (start_time, end_time) tuples representing time clusters
            day: The day being analyzed (not used in this detector but included for API consistency)
            
        Returns:
            Number of clusters that exceed max_duration but not any higher priority threshold
        """
        violation_count = 0
        
        for start_time, end_time in clusters:
            # Calculate the duration of this cluster
            cluster_duration = end_time - start_time
            assert(type(cluster_duration) == Duration)
            
            # Check if this cluster exceeds the maximum allowed duration for this constraint
            if cluster_duration > max_duration:
                # Now check if it's already caught by a higher-priority constraint
                already_caught = any(cluster_duration > higher_dur for higher_dur in higher_priority_durations)
                
                # Only count as a violation if not already caught by higher priority constraint
                if not already_caught:
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

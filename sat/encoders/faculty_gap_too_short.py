# encoders/faculty_gap_too_short.py
"""
Faculty gap too short constraint encoder for the Marmot timetabling system.

This module provides a function to encode a faculty gap too short constraint:
ensuring faculty don't have gaps between teaching clusters that are shorter than a specified duration.
"""

from data import TimetableData, Priority
from data import FacultyGapTooShort, Time, Duration, Day
from encoding import Encoding
from encoders.helpers import encode_faculty_cluster_helper

def encode_faculty_gap_too_short(
    timetable: TimetableData,
    encoding: Encoding,
    priority: Priority,
    constraint: FacultyGapTooShort
) -> None:
    """
    Encode a single faculty gap too short constraint.
    
    A faculty gap too short constraint specifies that a faculty member should not
    have gaps between teaching clusters that are shorter than a specified duration.
    Unlike the cluster too short constraint, there is no exception for the first violation.
    
    This function checks for any higher-priority constraints of the same type and
    adjusts the current constraint to avoid double-counting violations.
    """
    faculty = constraint.faculty
    days = constraint.days_to_check
    min_gap_duration = constraint.duration
    max_gap_within_cluster = constraint.max_gap_within_cluster

    # Find any constraints of the same type for this faculty
    same_faculty_constraints = [
        c for c in timetable.faculty_gap_too_short
        if c.faculty == faculty
    ]
    
    # Ensure there are no duplicate constraints at the same priority level
    same_priority_constraints = [
        c for c in same_faculty_constraints
        if c.priority == priority and c is not constraint
    ]
    assert not same_priority_constraints, f"Multiple faculty_gap_too_short constraints for {faculty} at priority {priority}"
    
    # Find higher-priority constraints of the same type for this faculty
    higher_priority_constraints = [
        c for c in same_faculty_constraints
        if c.priority < priority
    ]
    
    # Get the durations of higher-priority constraints (if any)
    higher_priority_durations = [c.duration for c in higher_priority_constraints]
    
    # Validate specific inputs for this constraint type
    assert min_gap_duration.minutes > 0, f"Non-positive minimum gap duration for faculty {faculty}"
    
    # Create a function that detects "too short" gaps
    def count_too_short_gaps(clusters: list[tuple[Time, Time]], day: Day) -> int:
        """
        Count gaps between clusters that are shorter than the minimum duration,
        only counting those that aren't already covered by higher-priority constraints.
        
        Args:
            clusters: List of (start_time, end_time) tuples representing time clusters
            day: The day being analyzed (not used in this detector but included for API consistency)
            
        Returns:
            Number of gaps that are shorter than min_gap_duration but not already caught
            by a higher priority constraint
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
            
            # Check if this gap is shorter than the minimum allowed duration
            if gap_duration < min_gap_duration:
                # Now check if it's already caught by a higher-priority constraint
                # For "too short" gaps, a higher priority constraint would have a LARGER minimum
                # (i.e., it's more restrictive if it requires a longer minimum duration)
                already_caught = any(gap_duration < higher_dur for higher_dur in higher_priority_durations)
                
                # Only count as a violation if not already caught by higher priority constraint
                if not already_caught:
                    violation_count += 1
        
        # Unlike cluster_too_short, we don't ignore the first violation for gaps
        return violation_count
    
    # Create a function that generates descriptions for violations
    def generate_too_short_gap_description(i: int, day: Day) -> str:
        """
        Generate a description for a specific violation.
        
        Args:
            i: The violation number (1-based)
            day: The day of the violation
            
        Returns:
            A human-readable description of the violation
        """
        if i == 1:
            return f"has a gap shorter than {min_gap_duration} on {day}"
        elif i == 2:
            return f"has a 2nd short gap on {day}"
        elif i == 3:
            return f"has a 3rd short gap on {day}"
        else:
            return f"has a {i}th short gap on {day}"
    
    # Use the helper function to handle the encoding
    encode_faculty_cluster_helper(
        timetable=timetable,
        encoding=encoding,
        faculty=faculty,
        days=days,
        max_gap=max_gap_within_cluster,
        priority=priority,
        violation_counter=count_too_short_gaps,
        description_generator=generate_too_short_gap_description
    )

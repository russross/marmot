"""
Faculty scheduling constraint encoders for the Marmot timetabling system.

This module provides encoders for faculty schedule constraints:
- Faculty Days Off: faculty members should have a specific number of days off
- Faculty Evenly Spread: faculty's classes should be evenly spread across working days
"""
from typing import Set, Optional, Callable, Any
from itertools import product
from pysat.formula import CNF, IDPool # type: ignore

from data import TimetableData, FacultyDaysOff, FacultyEvenlySpread, Days
from encoder_types import SectionTimeVars, SectionRoomVars, ConstraintEncoder
from encoder_registry import register_encoder
from faculty_utils import get_faculty_day_vars


class FacultyScheduleEncoder(ConstraintEncoder):
    """Combined encoder for faculty scheduling constraints (days off and evenly spread)."""
    
    def encode(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        section_room_vars: SectionRoomVars,
        priority: int,
        allow_violations: bool = False
    ) -> list[int]:
        """
        Encode faculty scheduling constraints at a specific priority level.
        
        This encoder handles both days off and evenly spread constraints.
        
        Args:
            timetable: The timetable data
            cnf: The CNF formula to add clauses to
            pool: The ID pool for variable creation
            section_time_vars: Mapping from (section, time_slot) to variable IDs
            section_room_vars: Mapping from (section, room) to variable IDs
            priority: The priority level to encode
            allow_violations: Whether to allow violations of these constraints
            
        Returns:
            List of criterion variables if violations are allowed, empty list otherwise
        """
        criterion_vars = []
        
        # Process days off constraints
        criterion_vars.extend(self._encode_days_off(
            timetable, cnf, pool, section_time_vars, priority, allow_violations
        ))
        
        # Process evenly spread constraints
        criterion_vars.extend(self._encode_evenly_spread(
            timetable, cnf, pool, section_time_vars, priority, allow_violations
        ))
        
        return criterion_vars
    
    def _encode_days_off(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        priority: int,
        allow_violations: bool
    ) -> list[int]:
        """Encode faculty days off constraints."""
        # Get all faculty days off constraints at this priority level
        constraints = [c for c in timetable.faculty_days_off if c.priority == priority]
        criterion_vars = []
        
        for constraint in constraints:
            criterion_var = self._process_faculty_constraint(
                timetable, cnf, pool, section_time_vars,
                constraint.faculty, 
                constraint.days_to_check,
                allow_violations,
                lambda day_vars_map, days: not self._is_valid_days_off(day_vars_map, days, constraint.desired_days_off),
                ("days_off", tuple(sorted(constraint.days_to_check.days)), constraint.desired_days_off)
            )
            if criterion_var is not None:
                criterion_vars.append(criterion_var)
                
        return criterion_vars
    
    def _encode_evenly_spread(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        priority: int,
        allow_violations: bool
    ) -> list[int]:
        """Encode faculty evenly spread constraints."""
        # Get all faculty evenly spread constraints at this priority level
        constraints = [c for c in timetable.faculty_evenly_spread if c.priority == priority]
        criterion_vars = []
        
        for constraint in constraints:
            criterion_var = self._process_faculty_constraint(
                timetable, cnf, pool, section_time_vars,
                constraint.faculty, 
                constraint.days_to_check,
                allow_violations,
                lambda day_vars_map, days: not self._is_valid_evenly_spread(day_vars_map, days),
                ("evenly_spread", tuple(sorted(constraint.days_to_check.days)))
            )
            if criterion_var is not None:
                criterion_vars.append(criterion_var)
                
        return criterion_vars
    
    def _process_faculty_constraint(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        faculty: str,
        days_to_check: Days,
        allow_violations: bool,
        violation_checker: Callable[[dict[int, bool], list[int]], bool],
        criterion_key: tuple[Any, ...]
    ) -> Optional[int]:
        """
        Process a generic faculty constraint with a callback for violation checking.
        
        Args:
            timetable: The timetable data
            cnf: The CNF formula to add clauses to
            pool: The ID pool for variable creation
            section_time_vars: Mapping from (section, time_slot) to variable IDs
            faculty: Faculty name
            days_to_check: Days to check for this constraint
            allow_violations: Whether to allow violations
            violation_checker: Function that determines if a configuration violates the constraint
            criterion_key: Tuple to use for the criterion variable ID
            
        Returns:
            Criterion variable if violations are allowed, None otherwise
        """
        # Verify the faculty exists
        assert faculty in timetable.faculty, f"Faculty {faculty} in constraint not found"
            
        # Get the sections assigned to this faculty
        faculty_sections = timetable.faculty[faculty].sections
        assert faculty_sections, f"Faculty {faculty} has no assigned sections"
        
        # Get the days to check
        days = list(days_to_check.days)
        assert days, f"Days to check is empty for faculty {faculty}"
        
        # Skip if faculty has only one section (for evenly spread)
        if len(faculty_sections) <= 1 and "evenly_spread" in str(criterion_key):
            return None
        
        # Get section-day variables for this faculty
        faculty_day_vars = get_faculty_day_vars(
            timetable, cnf, pool, section_time_vars, faculty, days_to_check
        )
        
        # Organize section-day variables by day
        day_vars_map = self._organize_section_day_vars(faculty_sections, days, faculty_day_vars)
        
        # Create a criterion variable if violations are allowed
        criterion_var = None
        if allow_violations:
            criterion_var = pool.id((faculty,) + criterion_key)
        
        # Encode the constraint
        self._encode_day_configuration_constraint(
            cnf, 
            day_vars_map, 
            days, 
            lambda config: violation_checker({day: has_class for day, has_class in config.items()}, days),
            allow_violations, 
            criterion_var
        )
        
        return criterion_var
    
    def _organize_section_day_vars(
        self,
        faculty_sections: Set[str],
        days: list[int],
        faculty_day_vars: dict[tuple[str, int], int]
    ) -> dict[int, list[int]]:
        """
        Organize section-day variables by day.
        
        Args:
            faculty_sections: Set of sections assigned to the faculty
            days: List of days to check
            faculty_day_vars: Mapping from (section, day) pairs to variable IDs
            
        Returns:
            Dictionary mapping days to lists of section-day variables
        """
        day_vars_map = {}
        for day in days:
            day_vars = []
            for section in faculty_sections:
                if (section, day) in faculty_day_vars:
                    day_vars.append(faculty_day_vars[(section, day)])
            day_vars_map[day] = day_vars
        return day_vars_map
    
    def _encode_day_configuration_constraint(
        self,
        cnf: CNF,
        day_vars_map: dict[int, list[int]],
        days: list[int],
        is_violation: Callable[[dict[int, bool]], bool],
        allow_violations: bool,
        criterion_var: Optional[int]
    ) -> None:
        """
        Encode constraints for all possible day configurations.
        
        This method enumerates all possible truth assignments to days
        and adds a clause for configurations that violate the constraint.
        
        Args:
            cnf: The CNF formula to add clauses to
            day_vars_map: Mapping from days to lists of section-day variables
            days: The days to check
            is_violation: Function that determines if a configuration violates the constraint
            allow_violations: Whether to allow violations of this constraint
            criterion_var: The criterion variable for this constraint if violations allowed
        """
        # For each possible assignment of classes to days
        for day_values in product([False, True], repeat=len(days)):
            # Create a mapping from day to whether there's a class scheduled
            config = {day: day_values[i] for i, day in enumerate(days)}
            
            # Check if this configuration violates the constraint
            if is_violation(config):
                # This configuration violates the constraint
                # Create a clause that disallows this configuration
                clause = []
                
                for day, has_class in config.items():
                    section_vars = day_vars_map[day]
                    
                    if not section_vars:
                        # No sections can be scheduled on this day
                        if has_class:
                            # Impossible assignment, skip this combination
                            break
                    else:
                        # Add appropriate literals for this day
                        if has_class:
                            # At least one section must have class on this day
                            # Negate: !section1 & !section2 & ... & !sectionN
                            # Becomes: section1 | section2 | ... | sectionN
                            clause.extend(section_vars)
                        else:
                            # No sections can have class on this day
                            # Negate: section1 | section2 | ... | sectionN
                            # Becomes: !section1 & !section2 & ... & !sectionN
                            for var in section_vars:
                                clause.append(-var)
                else:
                    # Only add the clause if we didn't break from the loop
                    if allow_violations and criterion_var is not None:
                        # Add criterion variable to allow violation
                        clause.append(criterion_var)
                    
                    # Add the clause to the formula
                    if clause:  # Only add non-empty clauses
                        cnf.append(clause)
    
    def _is_valid_days_off(
        self,
        day_vars_map: dict[int, list[int]],
        days: list[int],
        desired_days_off: int
    ) -> bool:
        """
        Check if a day configuration satisfies the days off requirement.
        
        Args:
            day_vars_map: Mapping from days to section variables
            days: List of days to check
            desired_days_off: Number of desired days off
            
        Returns:
            True if the configuration satisfies the days off requirement
        """
        days_with_class = sum(1 for day in days if day_vars_map[day])
        days_off = len(days) - days_with_class
        return days_off == desired_days_off
    
    def _is_valid_evenly_spread(
        self,
        day_vars_map: dict[int, list[int]],
        days: list[int]
    ) -> bool:
        """
        Check if a day configuration satisfies the evenly spread requirement.
        
        This checks if the difference between the max and min number of sections
        on any working day is at most 1.
        
        Args:
            day_vars_map: Mapping from days to section variables
            days: List of days to check
            
        Returns:
            True if the configuration satisfies the evenly spread requirement
        """
        # Count number of sections on each day
        sections_per_day = []
        for day in days:
            if day_vars_map[day]:  # Only consider days with classes
                sections_per_day.append(len(day_vars_map[day]))
                
        # If no classes on any day, it's evenly spread
        if not sections_per_day:
            return True
            
        # Check if max-min <= 1
        return max(sections_per_day) - min(sections_per_day) <= 1


# Register encoder with the registry
register_encoder("FacultyDaysOff", FacultyScheduleEncoder)
register_encoder("FacultyEvenlySpread", FacultyScheduleEncoder)

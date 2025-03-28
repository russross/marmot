"""
Faculty scheduling constraint encoders for the Marmot timetabling system.

This module provides encoders for faculty schedule constraints:
- Faculty Days Off: Ensures faculty members have a specific number of days without classes.
- Faculty Evenly Spread: Ensures faculty classes are evenly distributed across days with classes.

The encoders use a truth table approach, enumerating all possible section-day assignments
and adding CNF clauses to forbid configurations that violate the constraints.
"""
from typing import Optional, FrozenSet
from itertools import product
from pysat.formula import CNF, IDPool  # type: ignore

from data import TimetableData, FacultyDaysOff, FacultyEvenlySpread, Days
from encoder_types import SectionTimeVars, SectionRoomVars, ConstraintEncoder
from encoder_registry import register_encoder
from faculty_utils import get_faculty_day_vars


class FacultyScheduleEncoder(ConstraintEncoder):
    """Encoder for faculty scheduling constraints."""

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

        Args:
            timetable: The timetable data containing faculty and constraint information.
            cnf: The CNF formula to append clauses to.
            pool: The ID pool for creating unique variable IDs.
            section_time_vars: Mapping of (section, time_slot) to SAT variable IDs.
            section_room_vars: Mapping of (section, room) to SAT variable IDs (unused here).
            priority: The priority level of constraints to encode.
            allow_violations: If True, encode as soft constraints with criterion variables.

        Returns:
            List of criterion variables for soft constraints; empty list if hard constraints.
        """
        criterion_vars: list[int] = []
        criterion_vars.extend(
            self._encode_days_off(timetable, cnf, pool, section_time_vars, priority, allow_violations)
        )
        criterion_vars.extend(
            self._encode_evenly_spread(timetable, cnf, pool, section_time_vars, priority, allow_violations)
        )
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
        """
        Encode Faculty Days Off constraints for all faculty at the given priority.

        For each constraint:
        - Creates auxiliary variables for section-day assignments.
        - Enumerates all possible day configurations (days with/without classes).
        - Adds clauses to forbid configurations where the number of days off differs from desired.

        Args:
            timetable: The timetable data.
            cnf: The CNF formula to append clauses to.
            pool: The ID pool for variable creation.
            section_time_vars: Mapping of (section, time_slot) to SAT variables.
            priority: The priority level to encode.
            allow_violations: If True, adds criterion variables for soft constraints.

        Returns:
            List of criterion variables if violations are allowed; empty list otherwise.
        """
        constraints: list[FacultyDaysOff] = [
            c for c in timetable.faculty_days_off if c.priority == priority
        ]
        criterion_vars: list[int] = []

        for constraint in constraints:
            faculty: str = constraint.faculty
            days_to_check: Days = constraint.days_to_check
            desired_days_off: int = constraint.desired_days_off

            # Validate inputs
            assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
            assert days_to_check.days, f"Empty days_to_check for faculty {faculty}"
            assert desired_days_off >= 0, f"Negative desired_days_off for faculty {faculty}"
            assert desired_days_off <= len(days_to_check.days), (
                f"Desired days off {desired_days_off} exceeds possible days "
                f"{len(days_to_check.days)} for faculty {faculty}"
            )

            # Get faculty sections and auxiliary variables
            sections: set[str] = timetable.faculty[faculty].sections
            assert sections, f"Faculty {faculty} has no sections"
            day_vars: dict[tuple[str, int], int] = get_faculty_day_vars(
                timetable, cnf, pool, section_time_vars, faculty, days_to_check
            )

            # Organize variables by day
            day_map: dict[int, list[int]] = self._map_section_day_vars(sections, days_to_check.days, day_vars)

            # Create criterion variable for soft constraints
            criterion_var: Optional[int] = None
            if allow_violations:
                criterion_var = pool.id((faculty, "days_off", tuple(sorted(days_to_check.days)), desired_days_off))

            # Encode the constraint
            self._encode_days_off_constraint(
                cnf, day_map, list(days_to_check.days), desired_days_off, allow_violations, criterion_var
            )

            if criterion_var is not None:
                criterion_vars.append(criterion_var)

        return criterion_vars

    def _encode_days_off_constraint(
        self,
        cnf: CNF,
        day_map: dict[int, list[int]],
        days: list[int],
        desired_days_off: int,
        allow_violations: bool,
        criterion_var: Optional[int]
    ) -> None:
        """
        Encode the Faculty Days Off constraint for a single faculty member.

        Enumerates all 2^n configurations of days having classes or not (where n is the number of days).
        For each configuration violating the desired number of days off, adds a clause to forbid it.

        Args:
            cnf: The CNF formula to append clauses to.
            day_map: Mapping of day to list of section-day variables.
            days: List of days to check.
            desired_days_off: Number of days that should have no classes.
            allow_violations: If True, adds criterion variable to clauses.
            criterion_var: The criterion variable for soft constraints, if any.
        """
        # For each possible configuration of days with/without classes
        for config_tuple in product([False, True], repeat=len(days)):
            config: dict[int, bool] = {days[i]: config_tuple[i] for i in range(len(days))}
            days_off: int = sum(1 for day in days if not config[day])

            # Check if this configuration violates the constraint
            if days_off != desired_days_off:
                clause: list[int] = []
                skip: bool = False

                for day in days:
                    section_vars: list[int] = day_map.get(day, [])
                    has_class: bool = config[day]

                    if not section_vars and has_class:
                        # Impossible: no sections can be scheduled, but config requires a class
                        skip = True
                        break
                    elif section_vars:
                        if has_class:
                            # At least one section must be true; negate "all false"
                            clause.extend(section_vars)  # section1 ∨ section2 ∨ ...
                        else:
                            # All sections must be false; negate "any true"
                            clause.extend(-var for var in section_vars)  # ¬section1 ∧ ¬section2 → ¬section1 ∨ ¬section2 in clause

                if not skip and clause:  # Only add clause if valid and non-empty
                    if allow_violations and criterion_var is not None:
                        clause.append(criterion_var)  # Allows violation if criterion_var is true
                    cnf.append(clause)

    def _encode_evenly_spread(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        priority: int,
        allow_violations: bool
    ) -> list[int]:
        """
        Encode Faculty Evenly Spread constraints for all faculty at the given priority.

        For each constraint:
        - Creates auxiliary variables for section-day assignments.
        - Enumerates all 2^n possible section-day assignments (where n is sections × days).
        - Adds clauses to forbid configurations where working days' section counts differ by more than 1.

        Args:
            timetable: The timetable data.
            cnf: The CNF formula to append clauses to.
            pool: The ID pool for variable creation.
            section_time_vars: Mapping of (section, time_slot) to SAT variables.
            priority: The priority level to encode.
            allow_violations: If True, adds criterion variables for soft constraints.

        Returns:
            List of criterion variables if violations are allowed; empty list otherwise.
        """
        constraints: list[FacultyEvenlySpread] = [
            c for c in timetable.faculty_evenly_spread if c.priority == priority
        ]
        criterion_vars: list[int] = []

        for constraint in constraints:
            faculty: str = constraint.faculty
            days_to_check: Days = constraint.days_to_check

            # Validate inputs
            assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
            assert days_to_check.days, f"Empty days_to_check for faculty {faculty}"

            # Get faculty sections and auxiliary variables
            sections: set[str] = timetable.faculty[faculty].sections
            assert sections, f"Faculty {faculty} has no sections"
            if len(sections) <= 1:
                continue  # Evenly spread is trivial with 0 or 1 section

            day_vars: dict[tuple[str, int], int] = get_faculty_day_vars(
                timetable, cnf, pool, section_time_vars, faculty, days_to_check
            )

            # Organize variables by section and day for full enumeration
            section_day_map: dict[tuple[str, int], int] = day_vars
            all_vars: list[tuple[str, int, int]] = [
                (section, day, var) for (section, day), var in section_day_map.items()
            ]

            # Create criterion variable for soft constraints
            criterion_var: Optional[int] = None
            if allow_violations:
                criterion_var = pool.id((faculty, "evenly_spread", tuple(sorted(days_to_check.days))))

            # Encode the constraint
            self._encode_evenly_spread_constraint(
                cnf, all_vars, sections, list(days_to_check.days), allow_violations, criterion_var
            )

            if criterion_var is not None:
                criterion_vars.append(criterion_var)

        return criterion_vars

    def _encode_evenly_spread_constraint(
        self,
        cnf: CNF,
        all_vars: list[tuple[str, int, int]],
        sections: set[str],
        days: list[int],
        allow_violations: bool,
        criterion_var: Optional[int]
    ) -> None:
        """
        Encode the Faculty Evenly Spread constraint for a single faculty member.

        Enumerates all 2^n configurations of section-day assignments (where n is sections × days).
        For each configuration violating the max-min ≤ 1 condition on working days, adds a clause to forbid it.

        Args:
            cnf: The CNF formula to append clauses to.
            all_vars: List of (section, day, variable) tuples for all section-day assignments.
            sections: Set of faculty section names.
            days: List of days to check.
            allow_violations: If True, adds criterion variable to clauses.
            criterion_var: The criterion variable for soft constraints, if any.
        """
        var_map: dict[tuple[str, int], int] = {(s, d): v for s, d, v in all_vars}
        var_keys: list[tuple[str, int]] = list(var_map.keys())

        # Enumerate all possible assignments to section-day variables
        for assignment_tuple in product([False, True], repeat=len(var_keys)):
            assignment: dict[tuple[str, int], bool] = {
                var_keys[i]: assignment_tuple[i] for i in range(len(var_keys))
            }

            # Count sections per day
            sections_per_day: dict[int, int] = {day: 0 for day in days}
            for (section, day) in assignment:
                if day in days and assignment[(section, day)]:
                    sections_per_day[day] += 1

            # Identify working days and check spread
            working_days: list[tuple[int, int]] = [
                (day, count) for day, count in sections_per_day.items() if count > 0
            ]
            if not working_days:
                continue  # No classes scheduled; trivially satisfied

            counts: list[int] = [count for _, count in working_days]
            if max(counts) - min(counts) > 1:
                # Violation: Add clause to forbid this configuration
                clause: list[int] = []
                for (section, day), value in assignment.items():
                    var: int = var_map[(section, day)]
                    clause.append(var if not value else -var)  # Negate the assignment

                if allow_violations and criterion_var is not None:
                    clause.append(criterion_var)
                if clause:
                    cnf.append(clause)

    def _map_section_day_vars(
        self,
        sections: set[str],
        days: FrozenSet[int],
        day_vars: dict[tuple[str, int], int]
    ) -> dict[int, list[int]]:
        """
        Map section-day variables by day.

        Args:
            sections: Set of faculty section names.
            days: Set of days to check.
            day_vars: Mapping of (section, day) to SAT variables.

        Returns:
            Dictionary mapping each day to a list of section-day variables.
        """
        day_map: dict[int, list[int]] = {day: [] for day in days}
        for section in sections:
            for day in days:
                key: tuple[str, int] = (section, day)
                if key in day_vars:
                    day_map[day].append(day_vars[key])
        return day_map


# Register the encoder
register_encoder("FacultyDaysOff", FacultyScheduleEncoder)
register_encoder("FacultyEvenlySpread", FacultyScheduleEncoder)

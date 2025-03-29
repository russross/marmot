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
from registry import SectionTimeVars, SectionRoomVars, ConstraintEncoder, register_encoder
from faculty_utils import get_faculty_section_day_vars


class FacultyScheduleEncoder(ConstraintEncoder):
    """Encoder for faculty scheduling constraints."""

    def encode(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        section_room_vars: SectionRoomVars,
        priority: int
    ) -> list[int]:
        """
        Encode faculty scheduling constraints at a specific priority level.

        This encoder handles two types of faculty scheduling constraints:
        1. Faculty Days Off: Ensures faculty members have a specific number of days without classes
        2. Faculty Evenly Spread: Ensures faculty classes are evenly distributed across days

        Args:
            timetable: The timetable data containing faculty and constraint information
            cnf: The CNF formula to append clauses to
            pool: The ID pool for creating unique variable IDs
            section_time_vars: Mapping of (section, time_slot) to SAT variable IDs
            section_room_vars: Mapping of (section, room) to SAT variable IDs (unused here)
            priority: The priority level of constraints to encode

        Returns:
            List of criterion variables that can be set to true to allow a violation
        """
        criterion_vars: list[int] = []
        criterion_vars.extend(
            self._encode_days_off(timetable, cnf, pool, section_time_vars, priority)
        )
        criterion_vars.extend(
            self._encode_evenly_spread(timetable, cnf, pool, section_time_vars, priority)
        )
        return criterion_vars

    def _encode_days_off(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        priority: int
    ) -> list[int]:
        """
        Encode Faculty Days Off constraints for all faculty at the given priority.

        For each constraint:
        - Creates auxiliary variables for section-day assignments
        - Enumerates all possible day configurations (days with/without classes)
        - Adds clauses to forbid configurations where the number of days off differs from desired

        Args:
            timetable: The timetable data
            cnf: The CNF formula to append clauses to
            pool: The ID pool for variable creation
            section_time_vars: Mapping of (section, time_slot) to SAT variables
            priority: The priority level to encode

        Returns:
            List of criterion variables that can be set to true to allow a violation
        """
        constraints: list[FacultyDaysOff] = [
            c for c in timetable.faculty_days_off if c.priority == priority
        ]
        hallpass_vars: list[int] = []

        for constraint in constraints:
            faculty: str = constraint.faculty
            days: FrozenSet[int] = constraint.days_to_check.days
            desired_days_off: int = constraint.desired_days_off

            # Validate inputs
            assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
            assert days, f"Empty days_to_check for faculty {faculty}"
            assert desired_days_off >= 0, f"Negative desired_days_off for faculty {faculty}"
            assert desired_days_off <= len(days), (
                f"Desired days off {desired_days_off} exceeds possible days "
                f"{len(days)} for faculty {faculty}"
            )

            # get faculty sections and auxiliary variables
            #   (section_name, day) -> variable
            section_day_to_var = get_faculty_section_day_vars(timetable, cnf, pool, section_time_vars, faculty, days)
            section_day_list = list(section_day_to_var.keys())
            day_list = list(days)

            # only create one hallpass variable for the entire constraint
            # we create this the first time it is needed
            # just in case we find a constraint that is trivially met
            hallpass_var: Optional[int] = None

            # iterate through a truth table of all 2**n possible section_day combinations
            # note: this could be refined by filtering out the impossible combinations
            for combo in range(2**len(section_day_list)):
                # figure out what days are scheduled for this combo
                scheduled_days: set[int] = set()
                for (i, (_, day)) in enumerate(section_day_list):
                    # is this section_day var true in this combo?
                    if combo & (1<<i) != 0:
                        scheduled_days.add(day)

                # is this combo a violation?
                if len(days) - len(scheduled_days) != desired_days_off:
                    # encode that this should not happen
                    clause = []
                    for (i, key) in enumerate(section_day_list):
                        var = section_day_to_var[key]
                        if combo & (1<<i) == 0:
                            clause.append(var)
                        else:
                            clause.append(-var)

                    # create the hallpass variable lazily
                    if hallpass_var is None:
                        hallpass_var = pool.id((faculty, "days_off", days, desired_days_off))
                        hallpass_vars.append(hallpass_var)
                    clause.append(hallpass_var)
                    cnf.append(clause)

        return hallpass_vars

    def _encode_evenly_spread(
        self,
        timetable: TimetableData,
        cnf: CNF,
        pool: IDPool,
        section_time_vars: SectionTimeVars,
        priority: int
    ) -> list[int]:
        """
        Encode Faculty Days Off constraints for all faculty at the given priority.

        For each constraint:
        - Creates auxiliary variables for section-day assignments
        - Enumerates all possible day configurations (days with/without classes)
        - Adds clauses to forbid configurations where the number of days off differs from desired

        Args:
            timetable: The timetable data
            cnf: The CNF formula to append clauses to
            pool: The ID pool for variable creation
            section_time_vars: Mapping of (section, time_slot) to SAT variables
            priority: The priority level to encode

        Returns:
            List of criterion variables that can be set to true to allow a violation
        """
        constraints: list[FacultyEvenlySpread] = [
            c for c in timetable.faculty_evenly_spread if c.priority == priority
        ]
        hallpass_vars: list[int] = []

        for constraint in constraints:
            faculty: str = constraint.faculty
            days: FrozenSet[int] = constraint.days_to_check.days

            # Validate inputs
            assert faculty in timetable.faculty, f"Faculty {faculty} not found in timetable"
            assert days, f"Empty days_to_check for faculty {faculty}"
            assert len(days) > 1, f"Need at least two days to spread out classes for faculty {faculty}"

            # get faculty sections and auxiliary variables
            #   (section_name, day) -> variable
            section_day_to_var = get_faculty_section_day_vars(timetable, cnf, pool, section_time_vars, faculty, days)
            section_day_list = list(section_day_to_var.keys())
            day_list = list(days)

            # only create one hallpass variable for the entire constraint
            # we create this the first time it is needed
            # just in case we find a constraint that is trivially met
            hallpass_var: Optional[int] = None

            # iterate through a truth table of all 2**n possible section_day combinations
            # note: this could be refined by filtering out the impossible combinations
            for combo in range(2**len(section_day_list)):
                # count the sections on each day for this combo
                scheduled_days: dict[int, int] = {}
                for (i, (_, day)) in enumerate(section_day_list):
                    # is this section_day var true in this combo?
                    if combo & (1<<i) != 0:
                        if day not in scheduled_days:
                            scheduled_days[day] = 0
                        scheduled_days[day] += 1

                # is this combo a violation?
                (min_sections, max_sections) = (len(section_day_list), -1)
                for n in scheduled_days.values():
                    min_sections = min(min_sections, n)
                    max_sections = max(max_sections, n)
                if max_sections - min_sections > 1:
                    # encode that this should not happen
                    clause = []
                    for (i, key) in enumerate(section_day_list):
                        var = section_day_to_var[key]
                        if combo & (1<<i) == 0:
                            clause.append(var)
                        else:
                            clause.append(-var)

                    # create the hallpass variable lazily
                    if hallpass_var is None:
                        hallpass_var = pool.id((faculty, "evenly_spread", days))
                        hallpass_vars.append(hallpass_var)
                    clause.append(hallpass_var)
                    cnf.append(clause)

        return hallpass_vars


# Register the encoder
register_encoder("FacultyDaysOff", FacultyScheduleEncoder)
register_encoder("FacultyEvenlySpread", FacultyScheduleEncoder)

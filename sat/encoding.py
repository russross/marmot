from itertools import combinations

from data import TimetableData, SectionName, FacultyName, TimeSlotName, RoomName, Days, Day
from typing import Optional, NewType, Union

# section-room -> variable mapping
SectionRoomVars = NewType('SectionRoomVars', dict[tuple[SectionName, RoomName], int])

# section-time -> variable mapping
SectionTimeVars = NewType('SectionTimeVars', dict[tuple[SectionName, TimeSlotName], int])

# a placement is a generated schedule
Placement = NewType('Placement', dict[SectionName, tuple[Optional[RoomName], TimeSlotName]])


class Encoding:
    def __init__(self) -> None:
        self.last_var = 0
        self.clauses: set[frozenset[int]] = set()
        self.section_room_vars = SectionRoomVars({})
        self.section_time_vars = SectionTimeVars({})
        self.problems: dict[int, str] = {}
        self.hallpass: set[int] = set()

    def new_var(self) -> int:
        self.last_var += 1
        return self.last_var

    def add_clause(self, clause: set[int]) -> None:
        self.clauses.add(frozenset(clause))

    def pairwise_at_most_one(self, literals: set[int]) -> None:
        """
        Basic pairwise encoding for an at-most-1 constraint in SAT solving.
        """
        # No constraints needed if there are 0 or 1 literals
        if len(literals) <= 1:
            return

        # For each pair of literals, add a clause stating they can't both be True
        for (a, b) in combinations(literals, 2):
            self.add_clause({-a, -b})

    def totalizer_at_most_k(self, literals: set[int], k: int) -> None:
        """
        Encodes the constraint that at most k literals in the input list can be true.
        Uses the Totalizer encoding (a recursive, tree-based unary sum encoding).
        Builds the full sum representation first, then adds the constraint.

        Args:
            literals: A set of input literals (positive integers representing
                      variables whose sum is being constrained).
            k: The maximum number of literals allowed to be true.
        """
        n = len(literals)

        if k < 0:
            raise ValueError("k cannot be negative")
        if k >= n:
            # Constraint is trivially true, no clauses needed.
            return
        if not literals:
            # No literals to constrain, nothing to do.
            return
        if k == 0:
            # If k=0, all literals must be false. Add unit clauses.
            for lit in literals:
                self.add_clause({-lit})
            return

        # Build the full totalizer tree recursively.
        # The result is a list of n output variables representing the unary sum.
        # output_vars[i] means "at least i+1 literals are true".
        output_vars = self._build_full_totalizer_tree(list(literals))

        # Ensure the output_vars list has the expected length n
        # (Can happen if input literals list was empty, though handled above)
        if len(output_vars) != n:
             # This case should ideally not be reached if literals is not empty
             if not literals: # If input was empty, output is empty, k>=0 is trivial
                 return
             else: # Should not happen with non-empty literals
                 raise RuntimeError(f"Internal Error: Totalizer tree construction failed. Expected {n} outputs, got {len(output_vars)}")

        # Add the final constraint: "not (at least k+1 literals are true)"
        # This corresponds to negating the (k+1)-th output variable, which is
        # at index k in the 0-indexed list.
        # Since we checked k < n earlier, output_vars[k] is guaranteed to exist.
        self.add_clause({-output_vars[k]})


    def _build_full_totalizer_tree(self, input_literals: list[int]) -> list[int]:
        """
        Recursively builds the *full* Totalizer tree node, representing sums up to n.

        Args:
            input_literals: The list of literals for this node's subtree.

        Returns:
            A list of output variables representing the full unary sum.
            output[i] means "at least i+1 input literals are true".
            The length of the list is len(input_literals).
        """
        n = len(input_literals)

        # Base Case: If only one literal, the sum is just that literal itself.
        if n == 1:
            return [input_literals[0]]
        # Handle empty input case within recursion if needed, though outer func checks
        if n == 0:
            return []

        # Recursive Step: Split literals and build subtrees
        mid = n // 2
        left_lits = input_literals[:mid]
        right_lits = input_literals[mid:]

        # Recursively build the full trees for children
        left_outputs = self._build_full_totalizer_tree(left_lits)
        right_outputs = self._build_full_totalizer_tree(right_lits)

        # Merge the results from left and right subtrees without k-pruning
        merged_outputs = self._merge_full_totalizer_nodes(left_outputs, right_outputs)

        return merged_outputs

    def _merge_full_totalizer_nodes(self, left_outputs: list[int], right_outputs: list[int]) -> list[int]:
        """
        Merges the outputs of two child nodes in the Totalizer tree for the full sum.

        Args:
            left_outputs: Output variables from the left child. left_outputs[i]
                          means "at least i+1 true" in the left input set. Length n_left.
            right_outputs: Output variables from the right child. right_outputs[j]
                           means "at least j+1 true" in the right input set. Length n_right.

        Returns:
            A list of new output variables for the merged node, representing sum up to n_left + n_right.
            output[i] means "at least i+1 true" in the combined input set. Length n_left + n_right.
        """
        n_left = len(left_outputs)
        n_right = len(right_outputs)
        # The maximum possible sum from this node is n_left + n_right.
        max_output_index = n_left + n_right # Number of output vars needed

        # Create fresh output variables for this merge node for the full sum
        current_outputs = [self.new_var() for _ in range(max_output_index)]

        # Add merging clauses (implements adder logic)
        # Formula: (a_i AND b_j) => c_{i+j}  which is equivalent to
        # CNF: (~a_i OR ~b_j OR c_{i+j})
        # Indices: a_i corresponds to left_outputs[i-1] ("at least i")
        #          b_j corresponds to right_outputs[j-1] ("at least j")
        #          c_{i+j} corresponds to current_outputs[i+j-1] ("at least i+j")

        for i in range(n_left + 1): # i represents the count from the left (0 to n_left)
            for j in range(n_right + 1): # j represents the count from the right (0 to n_right)
                target_sum = i + j
                # Skip if the target sum is 0 (no constraint needed)
                # No upper bound (k) check here - build the full adder
                if target_sum == 0:
                    continue
                # Ensure the target sum does not exceed the bounds of the created output vars
                if target_sum > max_output_index:
                    continue # Should not happen if max_output_index = n_left + n_right

                # Clause: ~a_i V ~b_j V c_{i+j}
                clause = set()

                # Add ~a_i if i > 0
                if i > 0:
                    # left_outputs[i-1] represents "at least i"
                    clause.add(-left_outputs[i-1])

                # Add ~b_j if j > 0
                if j > 0:
                    # right_outputs[j-1] represents "at least j"
                    clause.add(-right_outputs[j-1])

                # Add c_{i+j}
                # current_outputs[target_sum-1] represents "at least target_sum"
                clause.add(current_outputs[target_sum-1])

                # Add the clause to the solver
                self.add_clause(clause)

        return current_outputs

    def make_faculty_section_day_vars(self, 
        timetable: TimetableData,
        faculty: FacultyName,
        days_to_check: Days
    ) -> dict[tuple[SectionName, Day], int]:
        """
        Get or create variables that represent when a faculty member's sections are scheduled on specific days.
        """
        
        # create the set of vars we return
        section_day_to_var: dict[tuple[SectionName, Day], int] = {}

        # create mappings to help with encoding
        var_to_time_slot_vars: dict[int, set[int]] = {}

        # for each section
        for section_name in timetable.faculty[faculty].sections:
            section = timetable.sections[section_name]

            # for each day of interest
            for day in days_to_check:

                # for each time slot of the section
                for time_slot_name in section.available_time_slots:
                    time_slot = timetable.time_slots[time_slot_name]

                    # but only the ones that cover this day
                    if day not in time_slot.days:
                        continue

                    # get a variable for this (section, day)
                    if (section_name, day) not in section_day_to_var:
                        var = self.new_var()
                        section_day_to_var[(section_name, day)] = var
                        var_to_time_slot_vars[var] = set()

                    # record this for encoding
                    time_var = self.section_time_vars[(section_name, time_slot_name)]
                    var_to_time_slot_vars[var].add(time_var)

        for (var, time_slot_vars) in var_to_time_slot_vars.items():
            # encode var -> (time_slot_1 OR time_slot_2 OR ...)
            # i.e. !var OR time_slot_1 OR time_slot_2 OR ...
            self.add_clause({-var} | time_slot_vars)

            # encode: (any of the time slots) -> var
            # i.e.: (!time_slot_1 AND !time_slot_2 AND ...) OR section_day_var
            # i.e.: (!time_slot_1 OR var) AND (!time_slot_2 OR var) AND ...
            for time_var in time_slot_vars:
                self.add_clause({-time_var, var})
        
        return section_day_to_var

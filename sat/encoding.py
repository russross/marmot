from data import SectionName, TimeSlotName, RoomName
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

    def new_var(self) -> int:
        self.last_var += 1
        return self.last_var

    def add_clause(self, clause: Union[list[int], set[int]]) -> None:
        self.clauses.add(frozenset(clause))

    def totalizer(self, literals: Union[list[int], set[int]], k: int) -> None:
        """
        Totalizer encoding for an at-most-k constraint in SAT solving.

        This function enforces that at most `k` literals in the given list can be True,
        using a totalizer encoding.
        """
        if isinstance(literals, set):
            literals = list(literals)

        if k >= len(literals):
            return   # No constraint needed if the limit is greater than the number of literals

        if k == 0:
            # If k == 0, all literals must be False
            for lit in literals:
                self.add_clause({-lit})
            return

        def build_totalizer(nodes: list[int]) -> list[int]:
            """
            Recursively constructs a totalizer tree.
            Each node represents a sum variable tracking the number of True literals.
            """
            if len(nodes) == 1:
                return [nodes[0]]  # Base case: a single literal is its own sum

            mid = len(nodes) // 2
            left = build_totalizer(nodes[:mid])
            right = build_totalizer(nodes[mid:])
            
            # Output variables for the merged totalizer, limited to k+1 (for enforcement)
            bound = min(len(left) + len(right), k + 1)
            output = [self.new_var() for _ in range(bound)]

            # Monotonicity constraint: sum[i] → sum[i-1] (ensures sorted sum variables)
            for i in range(1, bound):
                self.add_clause([-output[i], output[i - 1]])

            # Merge constraints: output[i] is True if enough left and right inputs are True
            for i in range(bound):
                for j in range(max(0, i - len(right) + 1), min(i + 1, len(left))):
                    if i - j < len(right):
                        # If both left[j] and right[i-j] are True, output[i] must be True
                        self.add_clause([-left[j], -right[i - j], output[i]])
                        # If output[i] is True, at least one of (left[j], right[i-j]) must be True
                        self.add_clause([-output[i], left[j]])
                        self.add_clause([-output[i], right[i - j]])
            
            return output

        # Build totalizer tree and enforce at-most-k constraint
        top_vars = build_totalizer(literals)
        if k < len(top_vars):
            self.add_clause({-top_vars[k]})  # Enforce at-most-k by disabling (k+1)th sum variable

    def pairwise(self, literals: Union[list[int], set[int]]) -> None:

        """
        Basic pairwise encoding for an at-most-1 constraint in SAT solving.
        """
        # No constraints needed if there are 0 or 1 literals
        if len(literals) <= 1:
            return

        if isinstance(literals, set):
            literals = list(literals)

        # For each pair of literals, add a clause stating they can't both be True
        for i in range(len(literals)):
            for j in range(i + 1, len(literals)):
                self.add_clause({-literals[i], -literals[j]})

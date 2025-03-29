from pysat.formula import IDPool # type: ignore

def totalizer_encode(literals: list[int], k: int, var_pool: IDPool) -> list[list[int]]:
    """Totalizer encoding for at-most-k constraint.

    Args:
        literals (list[int]): List of input literals to constrain.
        k (int): The maximum number of literals that can be True.
        var_pool (IDPool): IDPool for generating new variable IDs.

    Returns:
        list[list[int]]: List of clauses representing the encoding.
    """
    if k >= len(literals):
        return []  # No need to encode if k >= n

    clauses = []

    # Recursive function to build the totalizer tree
    def build_totalizer(nodes: list[int]) -> list[int]:
        if len(nodes) == 1:
            return nodes  # Leaf node

        mid = len(nodes) // 2
        left = build_totalizer(nodes[:mid])
        right = build_totalizer(nodes[mid:])

        # Create sum variables for parent node
        sum_vars = [var_pool.id() for _ in range(min(len(left) + len(right), k))]

        # Create clauses to combine child nodes
        for i, sv in enumerate(sum_vars):
            left_vars = left[:i+1]
            right_vars = right[:i+1]

            # Pairwise sum clauses
            for l in left_vars:
                for r in right_vars:
                    clauses.append([-sv, l, r])

            # At-least-one clauses
            if i < len(left):
                clauses.append([-left[i], sv])
            if i < len(right):
                clauses.append([-right[i], sv])

        return sum_vars

    # Build the tree and add the at-most-k constraint
    top_vars = build_totalizer(literals)

    # At most k by disallowing (k + 1)th variable to be true
    if len(top_vars) > k:
        clauses.append([-top_vars[k]])

    return clauses

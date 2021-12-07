import collections
from typing import Callable
import util


starting_positions = [int(s) for s in util.load_input(7).split(",")]

counts: dict[int, int] = collections.defaultdict(lambda: 0)

for p in starting_positions:
    counts[p] += 1


def compute_cost(
    counts: dict[int, int], target: int, cost_fn: Callable[[int], int]
) -> int:
    total = 0
    for pos, count in counts.items():
        total += cost_fn(abs(target - pos)) * count

    return total


def find_min(counts: dict[int, int], cost_fn: Callable[[int], int]) -> tuple[int, int]:
    min_cost: int = 2 ** 64
    min_pos = -1

    for pos in range(min(counts), max(counts) + 1):
        cost = compute_cost(counts, pos, cost_fn)
        if cost < min_cost:
            min_cost = cost
            min_pos = pos

    return (min_cost, min_pos)


min_cost, min_pos = find_min(counts, lambda dist: dist)

print(f"Part 1: {min_cost}")

min_cost, min_pos = find_min(counts, lambda dist: sum(range(1, dist + 1)))

print(f"Part 2: {min_cost}")

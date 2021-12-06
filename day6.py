import util

fish = [int(f) for f in util.load_input(6).split(",")]

fish_counts = [0 for _ in range(9)]

for f in fish:
    fish_counts[f] += 1


def step(counts: list[int]) -> list[int]:
    next_counts = [0 for _ in range(9)]
    for i in range(len(counts) - 1):
        next_counts[i] = counts[i + 1]

    next_counts[6] += counts[0]
    next_counts[8] = counts[0]

    return next_counts


def test():
    test_counts = [0, 1, 1, 2, 1, 0, 0, 0, 0]
    curr_test_counts = test_counts
    for _ in range(18):
        curr_test_counts = step(curr_test_counts)

    test_total = sum(curr_test_counts)
    assert test_total == 26


test()

curr_fish = fish_counts
for _ in range(80):
    curr_fish = step(curr_fish)

print(f"Part 1: {sum(curr_fish)}")

curr_fish = fish_counts
for _ in range(256):
    curr_fish = step(curr_fish)

print(f"Part 2: {sum(curr_fish)}")

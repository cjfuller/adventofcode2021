import numpy as np

import util

lines = util.input_lines(9)

grid = np.zeros((len(lines), len(lines[0])), dtype=int)

for r, line in enumerate(lines):
    for c, num in enumerate(line):
        grid[r, c] = int(num)

is_low = np.ones(grid.shape, dtype=bool)

for r in range(grid.shape[0]):
    for c in range(grid.shape[1]):
        if r > 0:
            is_low[r, c] = is_low[r, c] and (grid[r, c] < grid[r - 1, c])
        if c > 0:
            is_low[r, c] = is_low[r, c] and (grid[r, c] < grid[r, c - 1])
        if r < grid.shape[0] - 1:
            is_low[r, c] = is_low[r, c] and (grid[r, c] < grid[r + 1, c])
        if c < grid.shape[1] - 1:
            is_low[r, c] = is_low[r, c] and (grid[r, c] < grid[r, c + 1])

total_risk = ((grid + 1) * is_low).sum()

print(f"Part 1: {total_risk}")

flood = grid.copy()

flood[flood != 9] = 0
flood[flood == 9] = 2 ** 30

import skimage.morphology

seeds = skimage.morphology.label(is_low, connectivity=1)

flood[seeds != 0] = seeds[seeds != 0]
flooded = flood.copy()

for (r, c) in np.argwhere(seeds > 0):
    value = seeds[r, c]
    temp = skimage.morphology.flood_fill(
        flood, seed_point=(r, c), new_value=value, tolerance=2 ** 20, connectivity=1
    )
    flooded[temp == value] = temp[temp == value]

flooded[flooded == 2 ** 30] = 0

sizes = []

for label in np.unique(flooded):
    if label == 0:
        continue
    sizes.append((flooded == label).sum())

top_sizes = sorted(sizes, reverse=True)[:3]
result = top_sizes[0] * top_sizes[1] * top_sizes[2]

print(f"Part 2: {result}")

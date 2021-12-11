import numpy as np
import skimage.morphology as skm

import util

starting_grid = np.array([[int(i) for i in l] for l in util.input_lines(11)], dtype=int)


def iter(grid: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    flashed_at_this_iter = np.zeros(grid.shape, dtype=bool)
    last_flash = np.zeros(grid.shape, dtype=bool)
    grid += 1

    last_flash = (grid > 9) & ~flashed_at_this_iter

    while last_flash.sum() > 0:
        flashed_at_this_iter |= last_flash

        for (r, c) in np.argwhere(last_flash):
            mask = np.zeros(grid.shape, dtype=bool)
            mask[r, c] = 1
            mask = skm.binary_dilation(mask, footprint=skm.square(3))
            grid += mask

        last_flash = (grid > 9) & ~flashed_at_this_iter

    grid[grid > 9] = 0

    return grid, flashed_at_this_iter


def test_iter_small():
    grid = np.array(
        [
            [1, 1, 1, 1, 1],
            [1, 9, 9, 9, 1],
            [1, 9, 1, 9, 1],
            [1, 9, 9, 9, 1],
            [1, 1, 1, 1, 1],
        ]
    )
    next_grid, flashed = iter(grid)
    expected = np.array(
        [
            [3, 4, 5, 4, 3],
            [4, 0, 0, 0, 4],
            [5, 0, 0, 0, 5],
            [4, 0, 0, 0, 4],
            [3, 4, 5, 4, 3],
        ]
    )

    # print(next_grid)
    # print(flashed)
    assert np.all(next_grid == expected)


test_iter_small()


def test_iter():
    grid = np.array(
        [
            [5, 5, 9, 5, 2, 5, 5, 1, 1, 1],
            [3, 1, 5, 5, 2, 5, 5, 2, 2, 2],
            [3, 3, 6, 4, 4, 4, 4, 6, 0, 5],
            [2, 2, 6, 3, 4, 4, 4, 4, 9, 6],
            [2, 2, 9, 8, 4, 1, 4, 3, 9, 6],
            [2, 2, 7, 5, 7, 4, 4, 3, 4, 4],
            [2, 2, 6, 4, 5, 8, 3, 3, 4, 2],
            [7, 7, 5, 4, 4, 6, 3, 3, 4, 4],
            [3, 7, 5, 4, 4, 6, 9, 4, 3, 3],
            [3, 3, 5, 4, 4, 5, 2, 4, 3, 3],
        ]
    )

    next_grid, flashed = iter(grid)
    # print(next_grid)
    # print(flashed)
    assert flashed[0, 2] == 1
    assert flashed[0, 3] == 0
    assert next_grid[4, 1] == 5


test_iter()

total_flashes = 0

next_grid = starting_grid.copy()

for _ in range(100):
    next_grid, flashes = iter(next_grid)
    total_flashes += flashes.sum()

print(f"Part 1: {total_flashes}")

iter_number = 0
all_flashed = False

next_grid = starting_grid.copy()

while not all_flashed:
    iter_number += 1
    next_grid, flashed = iter(next_grid)

    all_flashed = np.all(flashed[:])

print(f"Part 2: {iter_number}")

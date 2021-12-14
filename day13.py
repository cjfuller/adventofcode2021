import re

import numpy as np
import skimage.io
import skimage.util

import util

points = []
folds = []

for line in util.input_lines(13):
    if m := re.match(r"(\d+),(\d+)", line):
        points.append((int(m.group(1)), int(m.group(2))))
    elif mm := re.match(r"fold along (x|y)=(\d+)", line):
        folds.append((mm.group(1), int(mm.group(2))))

x_max = max(x for x, _ in points)
y_max = max(y for _, y in points)

grid = np.zeros((y_max + 1, x_max + 1), dtype=bool)

for x, y in points:
    grid[y, x] = True


def x_fold(grid, x):
    new_grid = grid[:, :x].copy()
    for xi in range(x + 1, grid.shape[1]):
        assert 2 * x - xi >= 0
        new_grid[:, 2 * x - xi] |= grid[:, xi]

    return new_grid


def y_fold(grid, y):
    new_grid = grid[:y, :].copy()
    for yi in range(y + 1, grid.shape[0]):
        assert 2 * y - yi >= 0
        new_grid[2 * y - yi, :] |= grid[yi, :]

    return new_grid


def fold(grid, instruction):
    xy, coord = instruction

    if xy == "x":
        return x_fold(grid, coord)

    assert xy == "y"

    return y_fold(grid, coord)


next_grid = grid.copy()

next_grid = fold(next_grid, folds[0])

print(f"Part 1: {next_grid.sum()}")

next_grid = grid.copy()

for f in folds:
    next_grid = fold(next_grid, f)


skimage.io.imsave("./day13_p2.png", skimage.util.img_as_ubyte(next_grid))

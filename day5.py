from dataclasses import dataclass
import re

import numpy as np

import util


@dataclass
class Line:
    start: tuple[int, int]
    end: tuple[int, int]

    @classmethod
    def parse(cls, s: str) -> "Line":
        parser = r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)"
        m = re.match(parser, s)
        if m is None:
            raise ValueError(f"Could not parse {s}")

        return Line(
            start=(int(m.group("x1")), int(m.group("y1"))),
            end=(int(m.group("x2")), int(m.group("y2"))),
        )

    @property
    def is_horiz_or_vert(self):
        x1, y1 = self.start
        x2, y2 = self.end

        return x1 == x2 or y1 == y2

    def __iter__(self):
        x1, y1 = self.start
        x2, y2 = self.end
        x_step = 1 if x2 >= x1 else -1
        y_step = 1 if y2 >= y1 else -1

        if self.is_horiz_or_vert:
            for x in range(x1, x2 + x_step, x_step):
                for y in range(y1, y2 + y_step, y_step):
                    yield (x, y)

        else:
            for n, x in enumerate(range(x1, x2 + x_step, x_step)):
                y = n * y_step + y1
                yield (x, y)


lines = [Line.parse(s) for s in util.input_lines(5)]

hv_lines = [l for l in lines if l.is_horiz_or_vert]

grid = np.zeros(dtype=np.int32, shape=(1000, 1000))

for line in hv_lines:
    for x, y in line:
        grid[y, x] += 1

print(f"Part 1: {(grid > 1).sum()}")

grid2 = np.zeros(dtype=np.int32, shape=(1000, 1000))

for line in lines:
    for x, y in line:
        grid2[y, x] += 1

print(f"Part 2: {(grid2 > 1).sum()}")

from dataclasses import dataclass
import re
from typing import Optional, Sequence

import numpy as np

import util


Ranges = tuple[tuple[int, int], tuple[int, int], tuple[int, int]]


def size(r: Ranges) -> int:
    ((x0, x1), (y0, y1), (z0, z1)) = r
    return (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1)


def intersect(ranges: Ranges, other: Ranges) -> Optional[Ranges]:
    ((x0, x1), (y0, y1), (z0, z1)) = ranges
    ((a0, a1), (b0, b1), (c0, c1)) = other
    nx0 = max(x0, a0)
    nx1 = min(x1, a1)
    ny0 = max(y0, b0)
    ny1 = min(y1, b1)
    nz0 = max(z0, c0)
    nz1 = min(z1, c1)
    if nx0 <= nx1 and ny0 <= ny1 and nz0 <= nz1:
        return ((nx0, nx1), (ny0, ny1), (nz0, nz1))
    else:
        return None


@dataclass
class Instruction:
    value: bool
    ranges: Ranges
    offset: int

    @classmethod
    def from_str(cls, s: str, offset: int) -> "Instruction":
        parser = re.compile(
            r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)"
        )
        if m := parser.match(s):
            on_off, x0, x1, y0, y1, z0, z1 = m.groups()
            return cls(
                value=on_off == "on",
                ranges=(
                    (int(x0) + offset, int(x1) + offset),
                    (int(y0) + offset, int(y1) + offset),
                    (int(z0) + offset, int(z1) + offset),
                ),
                offset=offset,
            )
        else:
            raise ValueError(f"Could not parse {s}")

    def apply(self, matrix, limit_to: Optional[Ranges]):
        ((x0, x1), (y0, y1), (z0, z1)) = self.ranges
        if limit_to:
            ((xl0, xl1), (yl0, yl1), (zl0, zl1)) = limit_to
            x0 = max(x0, xl0 + self.offset)
            x1 = min(x1, xl1 + self.offset)
            y0 = max(y0, yl0 + self.offset)
            y1 = min(y1, yl1 + self.offset)
            z0 = max(z0, zl0 + self.offset)
            z1 = min(z1, zl1 + self.offset)
        if x0 <= x1 and y0 <= y1 and z0 <= z1:
            matrix[x0 : (x1 + 1), y0 : (y1 + 1), z0 : (z1 + 1)] = self.value


def parse_input(offset: int):
    return [Instruction.from_str(ln, offset) for ln in util.input_lines(22)]


instructions = parse_input(50)

matrix = np.zeros(shape=(101, 101, 101), dtype=bool)
for ins in instructions:
    ins.apply(matrix, limit_to=((-50, 50), (-50, 50), (-50, 50)))

on = matrix.sum()

print(f"Part 1: {on}")

instructions = parse_input(0)


def size_of(i: int, limit_to: Ranges, instructions: Sequence[Instruction]):
    me = instructions[i]
    limit_inter = intersect(me.ranges, limit_to)
    if limit_inter is None:
        return 0
    my_size = size(limit_inter) * me.value
    my_ranges = limit_inter
    for j, jns in enumerate(instructions[:i]):
        if me.value == jns.value:
            my_size -= size_of(j, my_ranges, instructions)
        else:
            my_size += size_of(j, my_ranges, instructions)

    return my_size


size_total = 0

for i, ins in enumerate(instructions):
    my_size = size_of(i, ((-50, 50), (-50, 50), (-50, 50)), instructions)
    if ins.value:
        size_total += my_size
    else:
        size_total -= my_size

print(f"Part 1 (alternate impl): {size_total}")


size_total = 0

for i, ins in enumerate(instructions):
    my_size = size_of(
        i, ((-100000, 100000), (-100000, 100000), (-100000, 100000)), instructions
    )
    if ins.value:
        size_total += my_size
    else:
        size_total -= my_size

print(f"Part 2: {size_total}")

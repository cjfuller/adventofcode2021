from dataclasses import dataclass, field, replace
from typing import Optional
from tqdm import tqdm
import util

lines = util.input_lines(8)

displayed = [l.split("|")[-1].strip().split(" ") for l in lines]

target_count = 0

for line in displayed:
    for item in line:
        if len(item) in set([2, 3, 4, 7]):
            target_count += 1

print(f"Part 1: {target_count}")

# Using caps to denote the non-scrambled wires.

segments = {
    0: set(["A", "B", "C", "E", "F", "G"]),
    1: set(["C", "F"]),
    2: set(["A", "C", "D", "E", "G"]),
    3: set(["A", "C", "D", "F", "G"]),
    4: set(["B", "C", "D", "F"]),
    5: set(["A", "B", "D", "F", "G"]),
    6: set(["A", "B", "D", "E", "F", "G"]),
    7: set(["A", "C", "F"]),
    8: set(["A", "B", "C", "D", "E", "F", "G"]),
    9: set(["A", "B", "C", "D", "F", "G"]),
}


def order(dig: str) -> str:
    return "".join(sorted(dig))


@dataclass
class Display:
    displayed: list[str]
    encoded: list[str]
    wire_mapping: dict[str, str] = field(default_factory=dict)
    digit_mapping: dict[str, int] = field(default_factory=dict)

    @classmethod
    def parse(cls, s: str) -> "Display":
        digits_s, displayed_s = tuple(it.strip() for it in s.split("|"))
        digits = digits_s.split(" ")
        displayed = displayed_s.split(" ")
        dsp = cls(displayed=displayed, encoded=digits)
        for dig in digits:
            possible = [num for num, seg in segments.items() if len(seg) == len(dig)]
            if len(possible) == 1:
                dsp.digit_mapping[order(dig)] = possible[0]
        dsp.assign_a()
        return dsp

    @property
    def rev_wires(self) -> dict[str, str]:
        return {v: k for k, v in self.wire_mapping.items()}

    @property
    def rev_digits(self) -> dict[int, str]:
        return {v: k for k, v in self.digit_mapping.items()}

    def assign_a(self) -> None:
        # You can immediately assign A based on the values of 7 / 1, which we
        # know, so do that.
        rd = self.rev_digits
        wire_a = list(set(rd[7]) - set(rd[1]))[0]
        self.wire_mapping["A"] = wire_a

    def try_assign(self, target: str, unknown: str) -> "Display":
        return replace(self, wire_mapping={**self.wire_mapping, target: unknown})

    def is_invalid(self) -> bool:
        substituted = self.encoded
        for sub, target in self.rev_wires.items():
            substituted = [s.replace(sub, target) for s in substituted]
        substituted = [s for s in substituted if s.isupper()]
        ok = all(
            any(
                len(set(s) - dig) == 0 and len(dig - set(s)) == 0
                for dig in segments.values()
            )
            for s in substituted
        )
        return not ok

    def assign_rest(self) -> Optional["Display"]:
        if self.is_invalid():
            return None

        remaining_unknowns = set(["a", "b", "c", "d", "e", "f", "g"]) - set(
            self.wire_mapping.values()
        )
        remaining_targets = set(["A", "B", "C", "D", "E", "F", "G"]) - set(
            self.wire_mapping.keys()
        )

        if len(remaining_unknowns) == 0:
            return self

        for unk in remaining_unknowns:
            for target in remaining_targets:
                tentative = self.try_assign(target, unk).assign_rest()
                if tentative is not None:
                    return tentative

        return None

    def decode(self) -> None:
        for dig in self.encoded:
            if order(dig) in self.digit_mapping:
                continue

            substituted = dig
            for sub, target in self.rev_wires.items():
                substituted = substituted.replace(sub, target)

            chars = set(substituted)

            for n, seg in segments.items():
                if len(seg - chars) == 0 and len(chars - seg) == 0:
                    self.digit_mapping[order(dig)] = n

    def number(self) -> int:
        digits = [self.digit_mapping[order(d)] for d in self.displayed]
        num = 0
        for i, d in enumerate(reversed(digits)):
            num += d * (10 ** i)
        return num


maybe_displays = [Display.parse(s).assign_rest() for s in tqdm(lines)]
displays = []

for d in maybe_displays:
    assert d is not None
    d.decode()
    displays.append(d)

print(f"Part 2: {sum(d.number() for d in displays)}")

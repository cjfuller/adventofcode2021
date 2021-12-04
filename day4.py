from dataclasses import dataclass
import re
from typing import cast

import numpy as np

import util


@dataclass
class Board:
    status: np.ndarray
    numbers: np.ndarray

    @classmethod
    def from_lines(cls, lines: list[str]) -> "Board":
        board_nums = np.array(
            [[int(s, 10) for s in re.split(r"\s+", ln)] for ln in lines]
        )
        return Board(
            status=np.zeros(board_nums.shape, np.dtype(bool)),
            numbers=board_nums,
        )

    def has_bingo(self) -> bool:
        for r in self.status:
            if r.sum() == len(r):
                return True

        for c in self.status.T:
            if c.sum() == len(c):
                return True

        return False

    def mark_number_mut(self, num: int) -> None:
        self.status[self.numbers == num] = 1

    @property
    def score(self) -> int:
        return cast(int, self.numbers[~self.status].sum())


def parse_input() -> tuple[list[int], list[Board]]:
    lines = util.input_lines(4)
    call_order = [int(s, 10) for s in lines[0].split(",")]
    boards = []

    board_cache: list[str] = []
    for line in lines[1:]:
        if len(line) == 0:
            if len(board_cache) > 0:
                boards.append(Board.from_lines(board_cache))
            board_cache = []
        else:
            board_cache.append(line)

    boards.append(Board.from_lines(board_cache))

    return (call_order, boards)


call_order, boards = parse_input()

last_num_called = None


def play_bingo(calls: list[int], boards: list[Board]):
    for call in calls:
        for board in boards:
            board.mark_number_mut(call)
            if board.has_bingo():
                return (call, board)


last_call, bingo_board = play_bingo(call_order, boards)

print(f"Part 1: {last_call * bingo_board.score}")


def last_bingo(calls: list[int], boards: list[Board]):
    next_boards = boards
    while len(next_boards) > 0:
        for call in calls:
            curr_boards = next_boards
            for board in curr_boards:
                board.mark_number_mut(call)
                if board.has_bingo():
                    if len(curr_boards) == 1:
                        return (call, board)
                    else:
                        next_boards = [b for b in next_boards if b is not board]


last_last_call, last_bingo_board = last_bingo(call_order, boards)

print(f"Part 2: {last_last_call * last_bingo_board.score}")

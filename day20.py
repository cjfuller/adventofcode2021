from dataclasses import dataclass

import numpy as np

import util


@dataclass
class Image:
    data: np.ndarray
    fill_value: int

    @property
    def shape(self) -> tuple[int, ...]:
        return self.data.shape


def read_input() -> tuple[np.ndarray, Image]:
    lines = util.input_lines(20)
    lookup = lines[0]
    image = lines[2:]

    lookup = [0 if c == "." else 1 for c in lookup]
    image = np.array(
        [[0 if c == "." else 1 for c in line] for line in image], dtype=int
    )

    return (lookup, Image(data=image, fill_value=0))


def iter(lookup, image) -> Image:
    next_im = np.zeros((image.shape[0] + 2, image.shape[1] + 2), dtype=int)
    big_im = (
        np.zeros((next_im.shape[0] + 2, next_im.shape[1] + 2), dtype=int)
        + image.fill_value
    )
    big_im[2:-2, 2:-2] = image.data

    for r in range(next_im.shape[0]):
        for c in range(next_im.shape[1]):
            window = big_im[r : r + 3, c : c + 3]
            num = 0
            vec = reversed(window.flatten())
            for idx, el in enumerate(vec):
                num += el << idx
            result = lookup[num]
            next_im[r, c] = result

    return Image(data=next_im, fill_value=(image.fill_value + 1) % 2)


lkp, im = read_input()

result = iter(lkp, iter(lkp, im))
num = result.data.sum()

print(f"Part 1: {num}")

for i in range(50):
    im = iter(lkp, im)

num = im.data.sum()
print(f"Part 2: {num}")

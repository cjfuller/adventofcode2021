import numpy as np

import util


def parse():
    return np.array([[int(c) for c in line] for line in util.input_lines(15)])


risk = parse()


def dyn_prog(danger):
    route_cache = np.ones(danger.shape, dtype=int) * 2 ** 30

    route_cache[-1, -1] = danger[-1, -1]

    for r in range(danger.shape[0] - 2, -1, -1):
        route_cache[r, -1] = route_cache[r + 1, -1] + danger[r, -1]

    for c in range(danger.shape[1] - 2, -1, -1):
        route_cache[-1, c] = route_cache[-1, c + 1] + danger[-1, c]

    assert danger.shape[0] == danger.shape[1]

    for offset in range(2, danger.shape[0]):
        route_cache[-offset, -offset] = (
            min(route_cache[-offset + 1, -offset], route_cache[-offset, -offset + 1])
            + danger[-offset, -offset]
        )
        for r in range(danger.shape[0] - offset - 1, -1, -1):
            route_cache[r, -offset] = (
                min(route_cache[r + 1, -offset], route_cache[r, -offset + 1])
                + danger[r, -offset]
            )

        for c in range(danger.shape[1] - offset - 1, -1, -1):
            route_cache[-offset, c] = (
                min(route_cache[-offset, c + 1], route_cache[-offset + 1, c])
                + danger[-offset, c]
            )

    route_cache[0, 0] = min(route_cache[0, 1], route_cache[1, 0]) + danger[0, 0]

    return route_cache


def iter_mod(dyn_prog_soln, danger):
    curr_soln = dyn_prog_soln.copy()
    changed = True
    while changed:
        changed = False

        for r in reversed(range(danger.shape[0])):
            for c in reversed(range(danger.shape[1])):
                options = []
                if r > 0:
                    options.append(curr_soln[r - 1, c])
                if r < danger.shape[0] - 1:
                    options.append(curr_soln[r + 1, c])
                if c > 0:
                    options.append(curr_soln[r, c - 1])
                if c < danger.shape[1] - 1:
                    options.append(curr_soln[r, c + 1])

                options = [o + danger[r, c] for o in options]
                best = min(options)
                if best < curr_soln[r, c]:
                    # print(f"Changing ({r}, {c}) from {curr_soln[r, c]} to {best}")
                    curr_soln[r, c] = best
                    changed = True

    return curr_soln


route_cache = dyn_prog(risk)
route_cache = iter_mod(route_cache, risk)
min_path = min(route_cache[0, 1], route_cache[1, 0])


print(f"Part 1: {min_path}")


def duplicate_board(risk):
    output = np.zeros((5 * risk.shape[0], 5 * risk.shape[1]), dtype=int)
    for r in range(5):
        for c in range(5):
            dist = r + c
            curr_risk = ((risk + dist - 1) % 9) + 1
            output[
                (r * risk.shape[0]) : ((r + 1) * risk.shape[0]),
                (c * risk.shape[1]) : ((c + 1) * risk.shape[1]),
            ] = curr_risk

    return output


big_risk = duplicate_board(risk)
route_cache = dyn_prog(big_risk)
route_cache = iter_mod(route_cache, big_risk)
min_path = min(route_cache[0, 1], route_cache[1, 0])
print(f"Part 2: {min_path}")


def test():
    risk = np.array(
        [
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
    )

    routes = dyn_prog(risk)
    min_path = min(routes[0, 1], routes[1, 0])
    assert min_path == 40


test()


def test2():
    risk = np.array(
        [
            [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
    )
    risk = duplicate_board(risk)

    routes = dyn_prog(risk)
    routes = iter_mod(routes, risk)
    min_path = min(routes[0, 1], routes[1, 0])
    assert min_path == 315


test2()

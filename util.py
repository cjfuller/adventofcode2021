import os


def load_input(day: int, strip: bool = True) -> str:
    filename = os.path.join(os.path.dirname(__file__), "inputs", f"day_{day}.txt")
    with open(filename) as f:
        s = f.read()
    if strip:
        return s.strip()
    return s


def input_lines(day: int, strip: bool = True) -> list[str]:
    lines = load_input(day, True).split("\n")
    if strip:
        return [line.strip() for line in lines]
    return lines

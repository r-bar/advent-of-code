from pprint import pprint, pformat
from collections import defaultdict, abc
from dataclasses import dataclass
import sys
import typing as t
import enum
import copy
import itertools as it


Coord: t.TypeAlias = tuple[int, int]


class Cell(enum.Enum):
    empty = enum.auto()
    beam = enum.auto()
    splitter = enum.auto()
    start = enum.auto()

    def __str__(self) -> str:
        match self:
            case self.empty:
                return "."
            case self.beam:
                return "|"
            case self.splitter:
                return "^"
            case self.start:
                return "S"


@dataclass
class Grid:
    rows: list[list[Cell]]

    def __repr__(self) -> str:
        name = self.__class__.__name__
        grid = "\n".join("".join(str(cell) for cell in row) for row in self.rows)
        return f"{name}:\n{grid}"

    def get(self, x: int, y: int) -> Cell | None:
        try:
            return self.rows[y][x]
        except IndexError:
            return None

    def set(self, x: int, y: int, cell: Cell) -> bool:
        """
        Sets the cell at (x, y) to the given cell. If the value is inside the
        grid and has been set the method returns True.
        """
        match self.get(x, y):
            case None | Cell.splitter:
                return False
            case to_set if to_set == cell:
                return True
            case _:
                self.rows[y][x] = cell
                return True

    def find_start(self) -> Coord | None:
        for y, row in enumerate(self.rows):
            for x, cell in enumerate(row):
                if cell == Cell.start:
                    return x, y
        return None

    def bottom_beams(self) -> abc.Iterator[Coord]:
        y = len(self.rows) - 1
        for x, cell in enumerate(self.rows[-1]):
            if cell == Cell.beam:
                yield x, y


def char_to_cell(char: str) -> Cell:
    match char:
        case ".":
            return Cell.empty
        case "S":
            return Cell.start
        case "^":
            return Cell.splitter
        case "|":
            return Cell.beam
        case _:
            raise ValueError(f"Unknown character: {char}")


def parse_input(content: str) -> Grid:
    rows = []
    for line in content.splitlines():
        row = [char_to_cell(char) for char in line.strip()]
        rows.append(row)
    return Grid(rows)


def part1(content: str) -> int:
    grid = parse_input(content)
    startx, starty = grid.find_start()
    work = [(startx, starty)]
    splits_hit = 0
    while work:
        x, y = work.pop()
        below = grid.get(x, y + 1)
        match below:
            case Cell.empty:
                was_set = grid.set(x, y + 1, Cell.beam)
                if was_set:
                    work.append((x, y + 1))
            case Cell.splitter:
                splits_hit += 1
                left_set = grid.set(x - 1, y + 1, Cell.beam)
                right_set = grid.set(x + 1, y + 1, Cell.beam)
                if left_set:
                    work.append((x - 1, y + 1))
                if right_set:
                    work.append((x + 1, y + 1))
            case _:
                pass
    return splits_hit


def part2(content: str) -> int:
    grid = parse_input(content)
    start: Coord = grid.find_start()
    beams = defaultdict(int)
    beams[start[0]] = 1
    for row in grid.rows:
        new_beams = defaultdict(int)
        for x, c in beams.items():
            if row[x] == Cell.splitter:
                new_beams[x - 1] += c
                new_beams[x + 1] += c
            else:
                new_beams[x] += c
        beams = new_beams
    return sum(beams.values())


def dfs(links: dict[Coord, abc.Iterable[Coord]], start: list[Coord], end: Coord) -> abc.Iterable[tuple[Coord, ...]]:
    outs = links[start[0]]
    for out in outs:
        if out == end:
            yield tuple((out, *start))
        else:
            yield from dfs(links, [out, *start], end)


def main():
    progname = sys.argv[0]
    answer: int
    match sys.argv[1:]:
        case ["1", filename] | ["part1", filename]:
            with open(filename, "r") as f:
                answer = part1(f.read())
        case ["2", filename] | ["part2", filename]:
            with open(filename, "r") as f:
                answer = part2(f.read())
        case _:
            print(f"{progname} <1|2> <filename>", file=sys.stderr)
            sys.exit(1)
    print(answer)


if __name__ == "__main__":
    main()

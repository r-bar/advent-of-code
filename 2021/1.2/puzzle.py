import sys
from collections import deque
from typing import TypeVar, Iterable, Iterator

T = TypeVar('T')

def window(size: int, iterator: Iterable[T]) -> Iterator:
    accum = deque()
    for i in iterator:
        accum.append(i)
        if len(accum) > size:
            accum.popleft()
        if len(accum) == size:
            yield tuple(accum)


def read_input(file: str) -> list[int]:
    output = []
    with open(file) as f:
        for line in f:
            output.append(int(line.strip()))
    return output


def main():
    try:
        file = sys.argv[1]
        depths = read_input(file)
    except IndexError:
        print("Please provide an input file", file=sys.stderr)
    increases = 0
    iterator = window(3, depths)
    p = sum(next(iterator))
    for dwindow in iterator:
        dsum = sum(dwindow)
        increases += int(dsum > p)
        p = dsum
    print(increases)


if __name__ == '__main__':
    main()

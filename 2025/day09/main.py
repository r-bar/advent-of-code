from dataclasses import dataclass
import sys


@dataclass(frozen=True)
class Input:
    lines: list[str]


def parse_input(content: str) -> Input:
    return Input(list(content.splitlines()))


def part1(content: Input) -> int:
    raise NotImplementedError


def part2(content: Input) -> int:
    raise NotImplementedError


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

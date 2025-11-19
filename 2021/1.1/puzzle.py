import sys

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
    p = depths[0]
    for depth in depths[1:]:
        increases += int(depth > p)
        p = depth
    print(increases)


if __name__ == '__main__':
    main()

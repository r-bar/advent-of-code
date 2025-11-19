import sys


def main():
    input_filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
    with open(input_filename) as f:
        lines = [line.strip() for line in f]
    print(len(lines))


if __name__ == '__main__':
    main()

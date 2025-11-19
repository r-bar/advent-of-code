import sys
import re

SPLIT_RE = re.compile("[0-9]+")

def overlap(r1, r2) -> bool:
    return r1[0] <= r2[0] <= r1[1] or r2[0] <= r1[0] <= r2[1]


def main():
    input_filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
    count = 0
    with open(input_filename) as f:
        for line in f:
            line = line.strip()
            s1, e1, s2, e2 = re.findall(SPLIT_RE, line)
            range1 = (int(s1), int(e1))
            range2 = (int(s2), int(e2))
            if overlap(range1, range2):
                count += 1
            # print(line, overlap(range1, range2))
    print(count)


if __name__ == '__main__':
    main()

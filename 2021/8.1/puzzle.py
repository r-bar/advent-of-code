import sys
from typing import NamedTuple


class Reading(NamedTuple):
    tests: list[set[str]]
    outputs: list[set[str]]

    @classmethod
    def from_str(cls, string):
        tests, outputs = string.strip().split(' | ')
        tests = [set(test) for test in tests.split()]
        outputs = [set(output) for output in outputs.split()]
        return cls(tests, outputs)

    def count_unique_segment_output(self):
        return sum(1 for out in self.outputs
                   if len(out) in {2, 3, 4, 7})


def main():
    with open(sys.argv[1]) as f:
        data = [Reading.from_str(line) for line in f if line]
    answer = sum(reading.count_unique_segment_output() for reading in data)
    print(answer)


if __name__ == '__main__':
    main()

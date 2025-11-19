"""
Correct mapping:
 aaa
b   c
b   c
 ddd
e   f
e   f
 ggg

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
"""
import sys
import itertools as it
from pprint import pprint
from typing import NamedTuple
from collections import Counter


CORRECT_MAPPING = [
    set('abcefg'),
    set('cf'),
    set('acdeg'),
    set('acdfg'),
    set('bcdf'),
    set('abdfg'),  # 5
    set('abdefg'),
    set('acf'),
    set('abcdefg'),
    set('abcdfg'),
]


class Solver:
    __slots__ = 'guesses', 'reading'

    def __init__(self, reading):
        self.reading = reading
        self.guesses = {
            'a': set('abcdefg'),
            'b': set('abcdefg'),
            'c': set('abcdefg'),
            'd': set('abcdefg'),
            'e': set('abcdefg'),
            'f': set('abcdefg'),
            'g': set('abcdefg'),
        }

    @staticmethod
    def light_overlaps(mapping: list[set[str]]) -> dict[int, set[str]]:
        output = []
        combinations = it.product(
            enumerate(mapping),
            enumerate(mapping),
        )
        for (ki, kv), (vi, vv) in combinations:
            if ki == len(output):
                output.append([])
            output[ki].append(kv & vv)
        return output

    @classmethod
    def light_overlap_counts(
        cls,
        mapping: list[set[str]]
    ) -> dict[int, set[str]]:
        return [Counter(map(len, val)) for val in cls.light_overlaps(mapping)]

    def match_by_counts(self):
        output = {}
        correct_counts = self.light_overlap_counts(CORRECT_MAPPING)
        test_counts = self.light_overlap_counts(self.reading.tests)
        for ti, test_count in enumerate(test_counts):
            for ci, correct_count in enumerate(correct_counts):
                if test_count == correct_count:
                    output[self.reading.get_test(ti)] = ci
                    break
        return output

    def known(self) -> dict[str, str]:
        return {
            k: next(iter(v)) for k, v in self.guesses.items()
            if len(v) == 1
        }

    def eliminate_known(self) -> bool:
        changed = False
        for known_signal, known_light in self.known().items():
            for signal, guess in self.guesses.items():
                if known_signal != signal and known_light in guess:
                    changed = True
                    guess.remove(known_light)
        return changed

    def cmp(self, n: int, lights: set[str]):
        signals = CORRECT_MAPPING[n]
        off_signals = CORRECT_MAPPING[8] - signals
        for signal in signals:
            self.guesses[signal] &= lights
        for signal in off_signals:
            self.guesses[signal] -= lights

    def solve(self) -> dict[str, str]:
        """Returns the mapping of signal wires to segment light"""
        known_tests = {
            0: self.reading.digit_test_1(),
            1: self.reading.digit_test_1(),
            4: self.reading.digit_test_4(),
            # 6: self.reading.digit_test_6(),
            7: self.reading.digit_test_7(),
            8: self.reading.digit_test_8(),
            9: self.reading.digit_test_9(),
        }
        for n, lights in known_tests.items():
            self.cmp(n, lights)

        while self.eliminate_known():
            continue

        return self.guesses

    def output(self):
        out = 0
        lightmap = self.match_by_counts()
        for i, output in enumerate(reversed(self.reading.str_outputs())):
            out += lightmap[output] * (10 ** i)
        return out


class Reading(NamedTuple):
    tests: list[set[str]]
    outputs: list[set[str]]

    @classmethod
    def from_str(cls, string) -> 'Reading':
        tests, outputs = string.strip().split(' | ')
        tests = [set(test) for test in tests.split()]
        outputs = [set(output) for output in outputs.split()]
        return cls(tests, outputs)

    def str_tests(self):
        return [''.join(sorted(test)) for test in self.tests]

    def str_outputs(self):
        return [''.join(sorted(output)) for output in self.outputs]

    def get_test(self, n):
        return ''.join(sorted(self.tests[n]))

    def get_output(self, n):
        return ''.join(sorted(self.outputs[n]))


def main():
    with open(sys.argv[1]) as f:
        data = [Reading.from_str(line) for line in f if line]
    print(sum(Solver(e).output() for e in data))


if __name__ == '__main__':
    main()

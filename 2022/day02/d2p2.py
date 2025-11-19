import sys
from enum import Enum

class Choice(Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

    @classmethod
    @property
    def _winners(cls):
        return {
            # x beats y
            (cls.ROCK, cls.SCISSORS),
            (cls.SCISSORS, cls.PAPER),
            (cls.PAPER, cls.ROCK),
        }

    @classmethod
    def parse(cls, char: str) -> 'Choice':
        mapping = {
            'A': cls.ROCK,
            'B': cls.PAPER,
            'C': cls.SCISSORS,
            'X': cls.ROCK,
            'Y': cls.PAPER,
            'Z': cls.SCISSORS,
        }
        return mapping[char]

    @classmethod
    def parse_line(cls, line: str) -> tuple['Choice', 'Choice']:
        left, outcome = line.strip().split()
        left = cls.parse(left)
        match outcome:
            case 'X':
                right = left.would_win_against()
            case 'Y':
                right = left
            case 'Z':
                right = left.would_lose_against()
        return left, right

    def would_win_against(self):
        return dict(self._winners)[self]

    def would_lose_against(self):
        winners = {loser: winner for winner, loser in self._winners}
        return winners[self]

    def beats(self, other):
        return (self, other) in self._winners

    def __int__(self):
        return self.value

    def __str__(self):
        return self.name

    def __gt__(self, other):
        return self.beats(other)

    def __lt__(self, other):
        return other.beats(self)

    def __eq__(self, other):
        return self.value == other.value

    def __hash__(self):
        return hash(self.name)


def play(left, right) -> tuple[int, int]:
    left_score, right_score = left.value, right.value
    if left == right:
        left_score += 3
        right_score += 3
    elif left > right:
        left_score += 6
    elif left < right:
        right_score += 6
    return left_score, right_score


def main():
    file = sys.argv[1]
    with open(file) as f:
        games = [Choice.parse_line(line) for line in f]
    left_total = 0
    right_total = 0
    for left, right in games:
        left_score, right_score = play(left, right)
        print(left_score, right_score)
        left_total += left_score
        right_total += right_score
    print(right_total)

if __name__ == '__main__':
    main()


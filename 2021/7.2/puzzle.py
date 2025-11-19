"""
Note: This just brute forces the solution. For the challenge input the answer
was using a target 1 less than that rounded average. It is possible we could
optimize this by better deriving the target. My intuition tells me it should be
the average of the dataset, but I struggled to prove this mathematically.
Computer go brrr
"""
import sys


def cost(distance):
    return sum(range(distance + 1))


def main():
    with open(sys.argv[1]) as f:
        data = [int(i) for i in f.read().strip().split(',')]
    options = []
    for target in range(min(data), max(data) + 1):
        fuel = sum(cost(abs(i - target)) for i in data)
        options.append((fuel, target))
    print(*min(options))


if __name__ == '__main__':
    main()

import sys


def main():
    with open(sys.argv[1]) as f:
        data = [int(i) for i in f.read().strip().split(',')]
    s = sorted(data)
    target = s[len(data) // 2]
    fuel = sum(abs(i - target) for i in data)
    print(target, fuel)


if __name__ == '__main__':
    main()

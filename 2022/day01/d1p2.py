def main():
    elves = []
    snacks = []
    with open('input.txt') as f:
        for line in f:
            line = line.strip()
            if line == "":
                elves.append(snacks)
                snacks = []
                continue
            snacks.append(int(line))
    elf_calories = sorted((sum(snacks) for snacks in elves), reverse=True)
    top3 = elf_calories[:3]
    print(f"{top3=}")
    print(sum(top3))

if __name__ == '__main__':
    main()

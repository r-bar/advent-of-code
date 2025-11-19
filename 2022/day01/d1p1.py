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
    elf_calories = [sum(snacks) for snacks in elves]
    print(max(elf_calories))

if __name__ == '__main__':
    main()

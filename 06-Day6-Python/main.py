def main(input_file: str):
    with open(input_file, 'r') as file:
        input = file.read()
        fishes = [int(i) for i in input.split(',')]

    print('Part 1: There are a total of {} fishes'.format(
        solution2(fishes, days=80)))
    print('Part 2: There are a total of {} fishes'.format(
        solution2(fishes, days=256)))


def solution2(fishes: list[int], days: int) -> int:
    fish_counter: list[int] = [0 for i in range(9)]
    for i in fishes:
        fish_counter[i] += 1

    for day in range(days):
        tmp: list[int] = [fish_counter[i] for i in range(len(fish_counter))]
        fish_counter[8] = fish_counter[0]
        tmp[7] += fish_counter[0]
        for idx, x in list(enumerate(tmp))[1:]:
            fish_counter[idx - 1] = x

    total: int = 0
    for i in fish_counter:
        total += i

    return total


if __name__ == '__main__':
    print('Sample:')
    main('sample.txt')
    print('Input:')
    main('input.txt')

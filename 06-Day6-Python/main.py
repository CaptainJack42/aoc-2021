import numpy as np
from collections import Counter


def main(input_file: str):
    with open(input_file, 'r') as file:
        input = file.read()
        fishes = np.array([int(i) for i in input.split(',')])

    print('Part 1: There are a total of {} fishes'.format(
        solution(fishes, days=80)))
    print('Part 2: There are a total of {} fishes'.format(
        solution(fishes, days=256)))


def solution(fishes: np.ndarray, days: int) -> int:
    fish_count = Counter(fishes)
    for day in range(days):
        tmp = [fish_count[i] for i in range(9)]
        fish_count[8] = fish_count[0]
        tmp[7] += fish_count[0]
        for i, x in list(enumerate(tmp))[1:]:
            fish_count[i-1] = x

    return fish_count.total()


if __name__ == '__main__':
    print('Sample:')
    main('sample.txt')
    print('Input:')
    main('input.txt')

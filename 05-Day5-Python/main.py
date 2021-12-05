from collections import namedtuple
from numpy import array, ndarray


Line = namedtuple('Line', ['x1', 'y1', 'x2', 'y2'])


def main(file_name: str):
    lines: list[Line] = []
    field_size: int = 0
    with open(file_name, 'r') as input:
        for line in input.readlines():
            coords = line.split('->')
            start = coords[0].split(',')
            end = coords[1].split(',')
            x1, y1, x2, y2 = int(start[0]), int(
                start[1]), int(end[0]), int(end[1])
            if field_size <= max(x1, y1, x2, y2):
                field_size = max(x1, y1, x2, y2) + 1
            vent = Line(x1, y1, x2, y2)
            lines.append(vent)

    print('Part 1: There are {} points where at least two lines overlap!'.format(
        part1(lines, field_size)))
    print('Part 2: There are {} points where at least two lines overlap!'.format(
        part2(lines, field_size)))


def part1(lines: list[Line], field_size: int) -> int:
    field = array([[0 for i in range(field_size)] for j in range(field_size)])
    for line in lines:
        if line.y1 == line.y2:
            field = draw_line(field, line)
        elif line.x1 == line.x2:
            field = draw_line(field, line)

    return count_overlaps(field)


def part2(lines: list[Line], field_size: int) -> int:
    field = array([[0 for i in range(field_size)] for j in range(field_size)])
    for line in lines:
        field = draw_line(field, line)

    print(field)
    return count_overlaps(field)


def draw_line(field: ndarray, line: Line) -> ndarray:
    x1, y1, x2, y2 = line.x1, line.y1, line.x2, line.y2
    if x1 > x2:
        x1, x2 = x2, x1

    if y1 > y2:
        y1, y2 = y2, y1

    if x1 == x2 or y1 == y2:
        for i in range(x1, x2 + 1):
            for j in range(y1, y2 + 1):
                field[j][i] += 1
    else:
        # FIXME: doesn't work smh
        i = 0
        while i <= x2 - x1 and i <= y2 - y1:
            field[y1 + i][x1 + i] += 1
            i += 1

    return field


def count_overlaps(field: ndarray) -> int:
    overlaps: int = 0
    for line in field:
        for cell in line:
            if cell >= 2:
                overlaps += 1

    return overlaps


if __name__ == '__main__':
    print('Sample:')
    main('sample.txt')
    # print('Input:')
    # main('input.txt')

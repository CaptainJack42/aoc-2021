def main():
    with open('input.txt', 'r') as file:
        draws = file.readline()
        draws_list = [int(elem) for elem in draws.split(',')]
        file.readline()
        board: list[int][int] = []
        boards: list[board] = []
        counter = 0
        num_boards = 0
        for line in file.readlines():
            if counter < 5:
                line_int = [int(elem) for elem in line.split()]
                board.append(line_int)
                counter += 1
            else:
                boards.append(board.copy())
                board.clear()
                num_boards += 1
                counter = 0

    print('Part 1: The score of the winning Board is: {}'.format(
        part_1(boards, draws_list)))
    print('Part 2: The score of the loosing Board is: {}'.format(
        part_2(boards, draws_list, num_boards)))


def part_1(boards, draws: list[int]) -> int:
    line_len = 5
    for draw in draws:
        for board in boards:
            for line in board:
                x = 0
                for x in range(line_len):
                    if line[x] == draw:
                        line[x] = 9999

            col_counter: list[int] = [0 for i in range(line_len)]
            for line in board:
                line_counter = 0
                y = 0
                for y in range(len(line)):
                    if line[y] == 9999:
                        col_counter[y] += 1
                        line_counter += 1

                if line_counter == 5:
                    return bingo(board, draw)

            for i in col_counter:
                if i == 5:
                    return bingo(board, draw)


def bingo(board, last_draw: int) -> int:
    unmarked: int = 0
    for line in board:
        for x in line:
            if x != 9999:
                unmarked += x

    return unmarked * last_draw


def part_2(boards, draws: list[int], num_boards: int) -> int:
    line_len = 5
    bingo_count = 0
    done_boards = [False for i in range(len(boards))]
    for draw in draws:
        if draw == 24:
            dummy = 5
        for index, board in enumerate(boards):
            if done_boards[index] == True:
                continue
            for line in board:
                x = 0
                for x in range(line_len):
                    if line[x] == draw:
                        line[x] = 9999

            col_counter: list[int] = [0 for i in range(line_len)]
            if draw == 24:
                dummy = 4
            for line in board:
                line_counter = 0
                y = 0
                for y in range(len(line)):
                    if line[y] == 9999:
                        col_counter[y] += 1
                        line_counter += 1

                if line_counter == 5:
                    done_boards[index] = True
                    bingo_count += 1
                    if bingo_count == num_boards:
                        return bingo(board, draw)
                    continue
        
            if not done_boards[index]:
                for i in col_counter:
                    if i == 5:
                        done_boards[index] = True
                        bingo_count += 1
                        if bingo_count == num_boards:
                            return bingo(board, draw)


if __name__ == '__main__':
    main()

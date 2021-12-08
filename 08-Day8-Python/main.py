def main(filename: str):
    with open(filename, 'r') as file:
        lines = file.readlines()

    print("Part 1: There are {} occurences of 1, 4, 7 and 8".format(part1(lines)))
    print("Part 2: The sum of all values is: {}".format(part2(lines)))


def part1(input: list[str]) -> int:
    counter: int = 0
    for line in input:
        output = line.split('|')
        digits = output[1].split()
        for digit in digits:
            if len(digit) <= 4 or len(digit) == 7:
                counter += 1

    return counter


def part2(input: list[str]) -> int:
    answer = 0
    for line in input:
        tmp = line.split('|')
        signals, digits = tmp[0], tmp[1]
        curr_signal = SignalMapping(signals.split())
        answer += curr_signal.solve(digits.split())

    return answer


class SignalMapping:
    def __init__(self, sig_pattern: list[str]) -> None:
        self.mapping: dict = {
            'sig_a': [],
            'sig_b': [],
            'sig_c': [],
            'sig_d': [],
            'sig_e': [],
            'sig_f': [],
            'sig_g': [],
        }

        self.identified: dict = {
            'A': '',
            'B': '',
            'C': '',
            'D': '',
            'E': '',
            'F': '',
            'G': '',
        }

        self.numbers: dict = {
            'zero': [],
            'one': [],
            'two': [],
            'three': [],
            'four': [],
            'five': [],
            'six': [],
            'seven': [],
            'eight': [],
            'nine': [],
        }
        for idx, sig in enumerate(sig_pattern):
            if len(sig) == 7:
                self.parse_8(sig)

        for idx, sig in enumerate(sig_pattern):
            if len(sig) == 2:
                self.parse_1(sig)

        for idx, sig in enumerate(sig_pattern):
            if len(sig) == 3:
                self.parse_7(sig)

        for idx, sig in enumerate(sig_pattern):
            if len(sig) == 4:
                self.parse_4(sig)

        for idx, sig in enumerate(sig_pattern):
            if len(sig) == 6:
                if self.mapping['sig_b'][0] in sig and self.mapping['sig_b'][1] in sig:
                    # count how often c and f appear in sig to identify 6 or 9
                    counter = sig.count(
                        self.mapping['sig_c'][0]) + sig.count(self.mapping['sig_c'][1])
                    if counter == 1:
                        self.parse_6(sig)
                    else:
                        self.parse_9(sig)
                else:
                    self.parse_0(sig)

        for idx, sig in enumerate(sig_pattern):
            if len(sig) == 5:
                # only 2 contains e
                if self.identified['E'] in sig:
                    self.parse_2(sig)
                # out of 3 and 5 only 3 contains c
                elif self.identified['C'] in sig:
                    self.parse_3(sig)
                else:
                    self.parse_5(sig)

    def solve(self, segments: list[str]) -> int:
        index_count = 3
        answer = 0
        for seg in segments:
            for idx, num in enumerate(self.numbers):
                if len(seg) == len(self.numbers[num]):
                    if self.compare(seg, self.numbers[num]):
                        answer += idx * pow(10, index_count)
            index_count -= 1

        return answer

    def compare(self, seg: str, expected: list[str]) -> bool:
        for char in seg:
            if char in expected:
                continue
            else:
                return False
        return True

    def parse_0(self, sig: str):
        for char in sig:
            self.numbers['zero'].append(char)
        for char in 'abcdefg':
            if char not in sig:
                self.identified['D'] = char

    def parse_1(self, sig: str):
        for char in sig:
            self.mapping['sig_c'].append(char)
            self.mapping['sig_f'].append(char)
            self.numbers['one'].append(char)

    def parse_2(self, sig: str):
        # since only 2 and 3 are left we can stop identifying stuff and use the numbers dict later to identify numbers
        for char in sig:
            self.numbers['two'].append(char)

    def parse_3(self, sig: str):
        # since only 3 is left we can stop identifying stuff and use the numbers dict later to identify numbers
        for char in sig:
            self.numbers['three'].append(char)

    def parse_4(self, sig: str):
        for char in sig:
            if char not in self.mapping['sig_c']:
                self.mapping['sig_b'].append(char)
                self.mapping['sig_d'].append(char)
            self.numbers['four'].append(char)

    def parse_5(self, sig: str):
        # since only 2, 3 and 5 are left we can stop identifying stuff and use the numbers dict later to identify numbers
        for char in sig:
            self.numbers['five'].append(char)

    def parse_6(self, sig: str):
        for char in sig:
            self.numbers['six'].append(char)
        for char in 'abcdefg':
            if char not in sig:
                self.identified['C'] = char
        for char in self.mapping['sig_f']:
            if char != self.identified['C']:
                self.identified['F'] = char

    def parse_7(self, sig: str):
        for char in sig:
            if char not in self.mapping['sig_c']:
                self.identified['A'] = char
            self.numbers['seven'].append(char)

    def parse_8(self, sig: str):
        for char in sig:
            self.numbers['eight'].append(char)

    def parse_9(self, sig: str):
        for char in sig:
            self.numbers['nine'].append(char)
        for char in 'abcdefg':
            if char not in sig:
                self.identified['E'] = char


if __name__ == '__main__':
    print("Sample:")
    main("sample.txt")
    print("Input:")
    main("input.txt")

from input_advent import read_file
from advent_1_1 import go_left, go_right


def rep_num(line):
    numbers = dict(
        one='o1e',
        two='t2o',
        three='t3e',
        four='4',
        five='5e',
        six='6',
        seven='7',
        eight='e8t',
        nine='9e'
    )
    for k, v in numbers.items():
        line = line.replace(k, v)
    return line


def advent_1_2(fname):
    return sum(
        int(f'{go_right(rep_l)}{go_left(rep_l)}')
        for rep_l in (rep_num(line) for line in read_file(fname))
    )


if __name__ == "__main__":
    print(advent_1_2('inputs/1_2_input'))

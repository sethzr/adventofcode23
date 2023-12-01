from input_advent import read_file


def go_right(word):
    for x in word:
        try:
            return int(x)
        except:
            continue


def go_left(word):
    return go_right(reversed(word))


def advent_1_1(fname):
    return sum(
        int(f'{go_right(word)}{go_left(word)}')
        for word in read_file(fname)
    )


if __name__ == "__main__":
    print(advent_1_1('inputs/1_1_input'))

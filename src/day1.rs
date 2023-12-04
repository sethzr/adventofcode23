use aoc_runner_derive::{aoc, aoc_generator};
use aho_corasick::AhoCorasick;

fn as_numeric(c: char) -> Option<u32> {

    if c.is_ascii_digit() {
        c.to_digit(10)
    } else {
        None
    }
}

fn go_right<I>(mut iter: I) -> u32
    where
        I: DoubleEndedIterator<Item = char>,
{
    iter.find_map(as_numeric).unwrap()
}

fn go_left<I>(iter: I) -> u32
    where
        I: DoubleEndedIterator<Item = char>,
{
    go_right(iter.rev())
}


#[aoc_generator(day1, part1)]
fn parse_part1(input: &str) -> Vec<(u32, u32)> {
    input
        .split('\n')
        .map(|line| {
            (
                go_right(line.chars()),
                go_left(line.chars()),
            )
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| {
        a * 10 + b
    }).sum()
}

const NUMBERS_IN_ENGLISH: &[&str; 9] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine",];
const REPLACEMENT_NUMBERS: &[&str; 9] = &["o1e", "t2o", "t3e", "4", "5e", "6", "7", "e8t", "9e",];

#[aoc_generator(day1, part2)]
fn parse_part2(input: &str) -> Vec<(u32, u32)> {

    let ac = AhoCorasick::new(NUMBERS_IN_ENGLISH).unwrap();
    let result = ac.replace_all(input, REPLACEMENT_NUMBERS);
    result
        .split('\n' )
        .map(|line| {
            (
                go_right(line.chars()),
                go_left(line.chars()),
            )
        })
        .collect()

}

#[aoc(day1, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| {
        a * 10 + b
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE_PART1)), 142);
    }

     const EXAMPLE_PART2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE_PART2)), 281);
    }
}
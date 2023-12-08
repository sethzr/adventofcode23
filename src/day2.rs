use aoc_runner_derive::{aoc, aoc_generator};
use derivative::Derivative;
use regex::Regex;

#[derive(Derivative)]
#[derivative(Default,Debug)]
struct CubeBag{
    #[derivative(Default(value = "0"))]
    red: u32,
    #[derivative(Default(value = "0"))]
    green: u32,
    #[derivative(Default(value = "0"))]
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeBag>,
}

impl Game {

    fn is_valid(&self, comp: &CubeBag) -> bool {
        self.sets.iter().all(
            |set| {
                set.is_valid(comp)
            }
        )
    }

    fn min_cubes(&self) -> CubeBag {
        self.sets
            .iter()
            .fold( CubeBag::default(), | result:CubeBag, bag: &CubeBag| {
                CubeBag {
                    red: result.red.max(bag.red),
                    green: result.green.max(bag.green),
                    blue: result.blue.max(bag.blue),
                }
            })
    }

    fn power_cubes(&self) -> u32 {
        let min_bag: CubeBag = self.min_cubes();
        min_bag.red * min_bag.green * min_bag.blue
    }
}

impl CubeBag {
    fn is_valid(&self, comp: &CubeBag) -> bool{
        self.red <= comp.red && self.green <= comp.green && self.blue <= comp.blue
    }
}



#[aoc_generator(day2, part1)]
#[aoc_generator(day2, part2)]
fn parse_input(input: &str) -> Vec<Game> {
    let game_regex = Regex::new(r"Game (\d+): (.*)").unwrap();
    let set_regex = Regex::new(r"(\d+) (\w+)").unwrap();
    let games: Vec<Game> = game_regex
        .captures_iter(input)
        .map(|cap| {
            let id: u32 = cap[1].parse().unwrap();
            let sets: Vec<CubeBag> = cap[2]
                .split(';')
                .filter_map(|str_sets| {
                    let mut cube_bag = CubeBag::default();
                    for set_cap in set_regex.captures_iter(str_sets) {
                        let count: u32 = set_cap[1].parse().unwrap();
                        let color = &set_cap[2];
                        match color {
                            "red" => cube_bag.red = count,
                            "green" => cube_bag.green = count,
                            "blue" => cube_bag.blue = count,
                            _ => {} // Handle other colors as needed
                        }
                    }
                    Some(cube_bag)
                })
                .collect();

            Game { id, sets }
        })
        .collect();

    return games

}


#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {

    let target = CubeBag {
        red: 12,
        green: 13,
        blue: 14
    };

    let result:u32 = input
        .iter()
        .filter(|game| game.is_valid(&target))
        .map(|game| game.id)
        .sum();

    return result
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {


    let result:u32 = input
        .iter()
        .map(|game| game.power_cubes())
        .sum();

    return result
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_PART1)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_PART1)), 2286);
    }
}
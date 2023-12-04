use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

// Test input - Part 1
#[allow(dead_code)]
const TEST_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[allow(dead_code)]
const INPUT: &str = include_str!("../../inputs/day-02.txt");

const MAX_R: i32 = 12;
const MAX_G: i32 = 13;
const MAX_B: i32 = 14;

fn main() {
    let games = INPUT
        .split('\n')
        .flat_map(parse_game)
        .map(|(_, game)| game)
        .collect_vec();

    println!("Day 02");
    println!("\t1: {}", part_1(&games));
    println!("\t2: {}", part_2(&games));
}

fn part_1(games: &[(i32, Vec<Draw>)]) -> i32 {
    games
        .iter()
        .filter(|(_, draws)| {
            draws
                .iter()
                .all(|draw| draw.is_possible(MAX_R, MAX_G, MAX_B))
        })
        .fold(0, |acc, (id, _)| acc + id)
}

fn part_2(games: &[(i32, Vec<Draw>)]) -> i32 {
    games
        .iter()
        .map(|(_, draws)| {
            let max = draws.iter().fold(Draw::default(), |acc, d| acc.max(d));
            max.red * max.green * max.blue
        })
        .sum()
}

#[derive(Default, Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

impl Draw {
    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_possible(&self, max_red: i32, max_green: i32, max_blue: i32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

// PARSING
fn parse_count(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_color(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(input)
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (remaining, counts) = separated_list1(
        tag(", "),
        separated_pair(parse_count, tag(" "), parse_color),
    )(input)?;

    Ok((
        remaining,
        counts.iter().fold(Draw::default(), |acc, (count, color)| {
            acc.max(
                &(match *color {
                    "red" => Draw {
                        red: *count,
                        ..Default::default()
                    },
                    "green" => Draw {
                        green: *count,
                        ..Default::default()
                    },
                    "blue" => Draw {
                        blue: *count,
                        ..Default::default()
                    },
                    _ => unreachable!(),
                }),
            )
        }),
    ))
}

fn parse_game(input: &str) -> IResult<&str, (i32, Vec<Draw>)> {
    separated_pair(
        preceded(tag("Game "), parse_count),
        tag(": "),
        separated_list1(tag("; "), parse_draw),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_games(input: &str) -> Vec<(i32, Vec<Draw>)> {
        input
            .split('\n')
            .flat_map(parse_game)
            .map(|(_, game)| game)
            .collect_vec()
    }

    #[test]
    fn test_1() {
        let games = parse_games(TEST_1);
        assert_eq!(8, part_1(&games));
    }
    #[test]
    fn test_2() {
        let games = parse_games(TEST_1);
        assert_eq!(2286, part_2(&games));
    }
}

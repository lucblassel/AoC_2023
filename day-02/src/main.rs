use anyhow::Result;
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

const MAX_R: usize = 12;
const MAX_G: usize = 13;
const MAX_B: usize = 14;

fn main() -> Result<()> {
    let games = INPUT
        .split('\n')
        .flat_map(parse_game)
        .map(|(_, game)| game)
        .collect_vec();

    println!("Day 02");
    println!(
        "\t1: {}",
        games
            .iter()
            .filter(|(_, draws)| {
                draws
                    .iter()
                    .all(|draw| draw.is_possible(MAX_R, MAX_G, MAX_B))
            })
            .fold(0, |acc, (id, _)| acc + id)
    );

    println!(
        "\t2: {}",
        games
            .iter()
            .map(|(_, draws)| Draw::get_power(draws))
            .sum::<usize>()
    );

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Default, Debug)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    fn from_vec(counts: &[(usize, Color)]) -> Self {
        let mut draw = Self::default();
        for (count, col) in counts {
            match col {
                Color::Red => draw.red = *count,
                Color::Green => draw.green = *count,
                Color::Blue => draw.blue = *count,
            }
        }

        draw
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_possible(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }

    fn get_power(draws: &[Self]) -> usize {
        let mut counter = Self::default();
        for draw in draws {
            counter = counter.max(draw);
        }

        counter.red * counter.green * counter.blue
    }
}

// PARSING
fn parse_count(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (remaining, col) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    let color = match col {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => unreachable!(),
    };
    Ok((remaining, color))
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (remaining, counts) = separated_list1(
        tag(", "),
        separated_pair(parse_count, tag(" "), parse_color),
    )(input)?;

    Ok((remaining, Draw::from_vec(&counts)))
}

fn parse_game(input: &str) -> IResult<&str, (usize, Vec<Draw>)> {
    separated_pair(
        preceded(tag("Game "), parse_count),
        tag(": "),
        separated_list1(tag("; "), parse_draw),
    )(input)
}

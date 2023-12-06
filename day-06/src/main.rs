use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[allow(dead_code)]
const TEST_1: &str = "Time:      7  15   30
Distance:  9  40  200";

const INPUT: &str = include_str!("../../inputs/day-06.txt");

fn main() -> Result<()> {
    println!("Day 06");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &'static str) -> Result<i64> {
    let (r, races) = parse_races(input)?;
    assert!(r.is_empty(), "Did not parse everything: {r}");

    Ok(races.iter().flat_map(|race| race.solve()).product())
}

fn part_2(input: &str) -> Result<i64> {
    let parsed = input
        .lines()
        .flat_map(|line| line.split_whitespace().skip(1).join("").parse())
        .collect_vec();

    Race {
        time: parsed[0],
        record: parsed[1],
    }
    .solve()
    .context("Could not solve race")
}

fn find_roots(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    let det = b.pow(2) - (4 * a * c);

    match det.cmp(&0) {
        std::cmp::Ordering::Less => None,
        std::cmp::Ordering::Equal => Some((-b / (2 * a), -b / (2 * a))),
        std::cmp::Ordering::Greater => {
            let r1: f64 = (-(b as f64) - f64::sqrt(det as f64)) / (2. * a as f64);
            let r2: f64 = (-(b as f64) + f64::sqrt(det as f64)) / (2. * a as f64);

            Some((f64::ceil(r1.min(r2)) as i64, f64::floor(r1.max(r2)) as i64))
        }
    }
}

#[derive(Debug)]
struct Race {
    time: i64,
    record: i64,
}

impl Race {
    fn solve(&self) -> Option<i64> {
        find_roots(-1, self.time, -self.record).map(|(r1, r2)| {
            let r1 = (r1 - 1..=r1 + 1)
                .filter(|&hold| (self.time - hold) * hold > self.record)
                .min()
                .unwrap();
            let r2 = (r2 - 1..=r2 + 1)
                .filter(|&hold| (self.time - hold) * hold > self.record)
                .max()
                .unwrap();

            r2 - r1 + 1
        })
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, map_res(digit1, str::parse))(input)
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (r, races) = separated_list1(
        newline,
        separated_pair(tuple((alpha1, tag(":"))), space1, parse_numbers),
    )(input)?;

    Ok((
        r,
        races[0]
            .1
            .clone()
            .into_iter()
            .zip(races[1].1.clone())
            .map(|(time, record)| Race { time, record })
            .collect_vec(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(288, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(1710720, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_2() {
        assert_eq!(71503, part_2(TEST_1).unwrap());
    }

    #[test]
    fn test_input_2() {
        assert_eq!(35349468, part_2(INPUT).unwrap());
    }
}

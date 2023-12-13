use std::collections::HashMap;

use anyhow::{bail, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

#[allow(dead_code)]
const TEST_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

const INPUT: &str = include_str!("../../inputs/day-04.txt");

type Cards = Vec<(usize, Vec<usize>, Vec<usize>)>;

fn main() -> Result<()> {
    let (remaining, cards) = parse_cards(INPUT)?;
    assert!(remaining.is_empty(), "Remaining: '{remaining}'");

    println!("Day 04");
    println!("\t1: {}", part_1(&cards)?);
    println!("\t2: {}", part_2(&cards)?);

    Ok(())
}

fn part_1(cards: &Cards) -> Result<usize> {
    Ok(cards
        .iter()
        .map(|(_, win, draw)| draw.iter().filter(move |d| win.contains(d)).count())
        .flat_map(|c| (c > 0).then_some(1 << c.saturating_sub(1)))
        .sum())
}

fn part_2(cards: &Cards) -> Result<usize> {
    let wins: HashMap<usize, usize> = HashMap::from_iter(
        cards
            .iter()
            .map(|(id, wins, draw)| (*id, draw.iter().filter(move |d| wins.contains(d)).count())),
    );

    let mut cache = HashMap::new();
    let res: usize = cards
        .iter()
        .flat_map(|(id, _, _)| get_subtree(id, &wins, &mut cache))
        .sum();

    Ok(res + cards.len())
}

// Compute the total number of cards returned from a single scratchcard recursively (with caching)
fn get_subtree(
    id: &usize,
    wins: &HashMap<usize, usize>,
    cache: &mut HashMap<usize, usize>,
) -> Result<usize> {
    match wins.get(id) {
        None => bail!("Unknown card id: {id}"),
        Some(0) => Ok(0),
        Some(w) => {
            if let Some(val) = cache.get(id) {
                Ok(*val)
            } else {
                let mut counter = *w;
                for child in (id + 1)..=(id + *w) {
                    counter += get_subtree(&child, wins, cache)?;
                }
                cache.insert(*id, counter);
                Ok(counter)
            }
        }
    }
}

// Parsing functions

fn parse_card_number(input: &str) -> IResult<&str, usize> {
    map_res(
        preceded(terminated(tag("Card"), space1), digit1),
        str::parse,
    )(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, map_res(digit1, str::parse))(input)
}

fn parse_card(input: &str) -> IResult<&str, (usize, Vec<usize>, Vec<usize>)> {
    let (remaining, (id, _)) = separated_pair(parse_card_number, tag(":"), space1)(input)?;
    let (remaining, (v1, v2)) = separated_pair(
        parse_numbers,
        delimited(space1, tag("|"), space1),
        parse_numbers,
    )(remaining)?;

    Ok((remaining, (id, v1, v2)))
}

fn parse_cards(input: &str) -> IResult<&str, Cards> {
    separated_list1(newline, parse_card)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (_, cards) = parse_cards(TEST_1).unwrap();
        assert_eq!(13, part_1(&cards).unwrap());
    }

    #[test]
    fn test_input_1() {
        let (_, cards) = parse_cards(INPUT).unwrap();
        assert_eq!(21485, part_1(&cards).unwrap());
    }
    #[test]
    fn test_2() {
        let (_, cards) = parse_cards(TEST_1).unwrap();
        assert_eq!(30, part_2(&cards).unwrap());
    }

    #[test]
    fn test_input_2() {
        let (_, cards) = parse_cards(INPUT).unwrap();
        assert_eq!(11024379, part_2(&cards).unwrap());
    }
}

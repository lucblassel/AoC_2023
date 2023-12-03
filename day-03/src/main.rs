#![allow(dead_code)]
use itertools::Itertools;
use std::collections::HashMap;

use anyhow::Result;

const TEST_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

const INPUT: &str = include_str!("../../inputs/day-03.txt");

fn main() -> Result<()> {
    let (numbers, symbols) = parse_board(INPUT);
    println!("Day 03");
    println!("\t1: {}", part_1(&numbers, &symbols)?);
    println!("\t2: {}", part_2(&numbers, &symbols)?);

    Ok(())
}

fn neighbours(x: usize, y: usize, number_size: usize) -> Vec<(usize, usize)> {
    let mut ys = (y..=y + number_size).map(Some).collect_vec();
    ys.push(y.checked_sub(1));
    ys.into_iter()
        .cartesian_product(vec![x.checked_sub(1), Some(x), Some(x + 1)])
        .flat_map(|(y, x)| match (x, y) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        })
        .collect_vec()
}

type Numbers = Vec<(String, (usize, usize))>;
type Symbols = HashMap<(usize, usize), char>;

fn parse_board(input: &str) -> (Numbers, Symbols) {
    let mut numbers = vec![];
    let mut symbols = HashMap::new();
    let mut curr = String::new();
    let (mut curr_x, mut curr_y) = (0, 0);

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let finish = match c {
                '.' => true,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if curr.is_empty() {
                        curr_x = x;
                        curr_y = y;
                    }
                    curr.push(c);
                    false
                }
                _ => {
                    symbols.insert((x, y), c);
                    true
                }
            };
            if finish && !curr.is_empty() {
                numbers.push((curr.clone(), (curr_x, curr_y)));
                curr = String::new();
            }
        }
    }

    (numbers, symbols)
}

fn part_1(numbers: &Numbers, symbols: &Symbols) -> Result<usize> {
    // let (numbers, symbols) = parse_board(input);
    Ok(numbers
        .iter()
        .filter(|(num, (x, y))| {
            neighbours(*x, *y, num.len())
                .iter()
                .any(|c| symbols.get(c).is_some())
        })
        .flat_map(|(num, _)| num.parse::<usize>())
        .sum())
}

fn part_2(numbers: &Numbers, symbols: &Symbols) -> Result<usize> {
    // let (numbers, symbols) = parse_board(input);

    let mut gears = HashMap::new();
    for (num, (x, y)) in numbers.iter() {
        for coords in neighbours(*x, *y, num.len()) {
            if let Some('*') = symbols.get(&coords) {
                let n = num.parse::<usize>()?;
                gears.entry(coords).or_insert_with(Vec::new).push(n);
            }
        }
    }

    Ok(gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (n, s) = parse_board(TEST_1);
        assert_eq!(4361, part_1(&n, &s).unwrap());
    }

    #[test]
    fn test_2() {
        let (n, s) = parse_board(TEST_1);
        assert_eq!(467835, part_2(&n, &s).unwrap());
    }
}

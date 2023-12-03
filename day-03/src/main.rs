#![allow(dead_code)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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
    println!("Day 03");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn neighbours(x: usize, y: usize, number_size: usize) -> Vec<(usize, usize)> {
    let mut ys = (y..=y + number_size).map(|y| Some(y)).collect_vec();
    ys.push(y.checked_sub(1));
    ys.iter()
        .cartesian_product(vec![x.checked_sub(1), Some(x), Some(x + 1)])
        .flat_map(|(y, x)| match (x, y) {
            (Some(x), Some(y)) => Some((x.clone(), y.clone())),
            _ => None,
        })
        .collect_vec()
}

fn parse_board(input: &str) -> (Vec<(String, (usize, usize))>, HashMap<(usize, usize), char>) {
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

fn part_1(input: &str) -> Result<usize> {
    let (numbers, symbols) = parse_board(input);
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

fn part_2(input: &str) -> Result<usize> {
    let (numbers, symbols) = parse_board(input);

    let groups = numbers
        .iter()
        .flat_map(|(n, (x, y))| {
            neighbours(*x, *y, n.len())
                .into_iter()
                .flat_map(|c| match symbols.get(&c) {
                    Some('*') => Some((c, n.clone())),
                    _ => None,
                })
        })
        .group_by(|(c, _)| c.clone());

    Ok(groups
        .into_iter()
        .flat_map(|(_, v)| {
            v.enumerate()
                .map(|(i, (_, v))| (i, v.parse::<usize>().unwrap()))
                .fold(Some((0, 1)), |acc, (i, v)| {
                    if i > 1 {
                        return None;
                    }
                    if let Some((_, acc_v)) = acc {
                        Some((i, acc_v * v))
                    } else {
                        None
                    }
                })
        })
        .fold(0, |acc, (i, v)| if i == 1 { acc + v } else { acc }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4361, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_2() {
        assert_eq!(467835, part_2(TEST_1).unwrap());
    }
}

use std::collections::HashMap;

use anyhow::{Context, Result};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{alphanumeric1, multispace1, newline, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[allow(dead_code)]
const TEST_1_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

#[allow(dead_code)]
const TEST_1_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

#[allow(dead_code)]
const TEST_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

const INPUT: &str = include_str!("../../inputs/day-08.txt");

fn main() -> Result<()> {
    let (_, (path, splits)) = separated_pair(
        is_a("LR"),
        multispace1,
        separated_list1(newline, parse_split),
    )(INPUT)?;

    let (nodes, indices) = build_graph(&splits)?;

    println!("Day 08");
    println!("\t1: {}", part_1(&nodes, &indices, path)?);
    println!("\t2: {}", part_2(&nodes, &indices, path)?);

    Ok(())
}

#[derive(Debug)]
struct Node {
    name: String,
    left: usize,
    right: usize,
}

fn part_1(nodes: &[Node], indices: &HashMap<&str, usize>, path: &str) -> Result<usize> {
    let start = indices.get("AAA").context("Could not find AAA")?;
    find_cycle_length(nodes, *start, path)
}

fn part_2(nodes: &[Node], indices: &HashMap<&str, usize>, path: &str) -> Result<usize> {
    Ok(indices
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .flat_map(|(_, start)| find_cycle_length(nodes, *start, path).ok())
        .fold(1, |a, b| a * (b / gcd(a, b)))) // LCM computed with GCD
}

fn build_graph<'a>(
    splits: &[(&'a str, (&str, &str))],
) -> Result<(Vec<Node>, HashMap<&'a str, usize>)> {
    let indices = HashMap::from_iter(splits.iter().enumerate().map(|(i, (s, _))| (*s, i)));
    let nodes: Result<Vec<_>> = splits
        .iter()
        .map(|(name, (l, r))| {
            let left = indices.get(*l);
            let right = indices.get(*r);

            match (left, right) {
                (Some(left), Some(right)) => Some(Node {
                    name: (*name).into(),
                    left: *left,
                    right: *right,
                }),
                _ => None,
            }
            .context("Could not find all child indices")
        })
        .collect();

    Ok((nodes?, indices))
}

// Find length of path start ('--A') to a ('--Z') node in a given cycle with a given path
fn find_cycle_length(nodes: &[Node], start: usize, path: &str) -> Result<usize> {
    let mut path = path.chars().cycle();
    let mut curr_idx = start;
    let mut curr_val = &nodes[start].name;
    let mut path_len = 0;

    while !curr_val.ends_with('Z') {
        path_len += 1;
        let turn = path.next().context("Iterator ended unexpectedly")?;
        // eprintln!("Visiting {curr_val}, turning {turn}");

        curr_idx = match turn {
            'L' => nodes[curr_idx].left,
            'R' => nodes[curr_idx].right,
            _ => unreachable!("Unknown turn '{turn}'"),
        };
        curr_val = &nodes[curr_idx].name;
    }

    Ok(path_len)
}

fn gcd(a: usize, b: usize) -> usize {
    let max = a.max(b);
    let min = a.min(b);

    if min == 0 {
        max
    } else {
        gcd(min, max % min)
    }
}

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        tag("("),
        separated_pair(alphanumeric1, tuple((tag(","), space1)), alphanumeric1),
        tag(")"),
    )(input)
}

fn parse_split(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(alphanumeric1, tuple((space1, tag("="), space1)), parse_pair)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &'static str) -> Result<(Vec<Node>, HashMap<&str, usize>, &str)> {
        let (_, (path, splits)) = separated_pair(
            is_a("LR"),
            multispace1,
            separated_list1(newline, parse_split),
        )(input)?;

        let (nodes, indices) = build_graph(&splits)?;

        Ok((nodes, indices, path))
    }

    #[test]
    fn test_1() {
        let (nodes, indices, path) = parse(TEST_1_1).unwrap();
        assert_eq!(2, part_1(&nodes, &indices, path).unwrap());
        let (nodes, indices, path) = parse(TEST_1_2).unwrap();
        assert_eq!(6, part_1(&nodes, &indices, path).unwrap());
    }

    #[test]
    fn test_input_1() {
        let (nodes, indices, path) = parse(INPUT).unwrap();
        assert_eq!(19631, part_1(&nodes, &indices, path).unwrap());
    }

    #[test]
    fn test_2() {
        let (nodes, indices, path) = parse(TEST_2).unwrap();
        assert_eq!(6, part_2(&nodes, &indices, path).unwrap());
    }

    #[test]
    fn test_input_2() {
        let (nodes, indices, path) = parse(INPUT).unwrap();
        assert_eq!(21003205388413, part_2(&nodes, &indices, path).unwrap());
    }
}

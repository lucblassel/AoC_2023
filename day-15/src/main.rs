use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[allow(dead_code)]
const TEST_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

const INPUT: &str = include_str!("../../inputs/day-15.txt");
const ANSWER_1: usize = 519603;
const ANSWER_2: usize = 244342;

fn main() -> Result<()> {
    println!("Day 15");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input.split(',').map(hash).sum())
}

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .filter(|&v| *v != b'\n')
        .fold(0, |acc, b| (acc + *b as usize) * 17 % 256)
}

fn part_2(input: &'static str) -> Result<usize> {
    let mut boxes = vec![vec![]; 256];
    for op in input.split(',').map(|v| parse_op(v).map(|(_, o)| o)) {
        let (label, op) = op?;
        let hash = hash(label);
        let idx = boxes[hash].iter().position(|&(v, _)| v == label);
        match op {
            Op::Remove => {
                if let Some(index) = idx {
                    boxes[hash].remove(index);
                }
            }
            Op::Insert(focal_length) => {
                if let Some(index) = idx {
                    boxes[hash][index] = (label, focal_length)
                } else {
                    boxes[hash].push((label, focal_length))
                }
            }
        }
    }

    Ok(boxes
        .into_iter()
        .enumerate()
        .flat_map(|(bx, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(slot, (_, focal_length))| (bx + 1) * (slot + 1) * focal_length)
        })
        .sum())
}

#[derive(Debug)]
enum Op {
    Insert(usize),
    Remove,
}

fn parse_op(input: &str) -> IResult<&str, (&str, Op)> {
    tuple((
        is_not("-="),
        alt((
            map_res(char('-'), |_| Some(Op::Remove).context("Error parsing op")),
            map_res(tuple((char('='), digit1)), |(_, d)| {
                str::parse(d).map(Op::Insert)
            }),
        )),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1320, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(ANSWER_1, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_2() {
        let v = part_2(TEST_1);
        eprintln!("Err?: {v:?}");
        assert_eq!(145, part_2(TEST_1).unwrap());
    }

    #[test]
    fn test_input_2() {
        assert_eq!(ANSWER_2, part_2(INPUT).unwrap());
    }
}

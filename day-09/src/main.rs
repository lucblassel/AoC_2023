use anyhow::Result;
use itertools::Itertools;

#[allow(dead_code)]
const TEST_1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
const INPUT: &str = include_str!("../../inputs/day-09.txt");

fn main() -> Result<()> {
    println!("Day 09");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &'static str) -> Result<i64> {
    Ok(input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(infer_next)
        .sum())
}

fn infer_next(hist: Vec<i64>) -> i64 {
    let mut nums = hist.clone();
    let mut values = vec![nums[nums.len() - 1]];
    while !nums.iter().all(|&v| v == 0) {
        nums = nums
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        values.push(nums[nums.len() - 1]);
    }
    values.iter().sum()
}

fn part_2(input: &str) -> Result<i64> {
    Ok(input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(infer_prev)
        .sum())
}

fn infer_prev(hist: Vec<i64>) -> i64 {
    let mut nums = hist.clone();
    let mut values = vec![nums[0]];
    while !nums.iter().all(|&v| v == 0) {
        nums = nums
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        values.push(nums[0]);
    }
    let mut n = values.pop().unwrap();
    while let Some(last) = values.pop() {
        n = last - n;
    }

    n
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(114, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(1993300041, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_2() {
        assert_eq!(2, part_2(TEST_1).unwrap());
    }

    #[test]
    fn test_input_2() {
        assert_eq!(1038, part_2(INPUT).unwrap());
    }
}

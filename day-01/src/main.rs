use anyhow::Result;
use itertools::Itertools;

// Test input - part 1
#[allow(dead_code)]
const TEST_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
// Test input - part 2
#[allow(dead_code)]
const TEST_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const INPUT: &str = include_str!("../../inputs/day-01.txt");

fn main() -> Result<()> {
    println!("Day 01");
    println!(
        "\t1: {}",
        INPUT
            .split_whitespace()
            .map(|line| {
                let digits = line.chars().filter(|c| c.is_digit(10)).collect_vec();
                format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<usize>()
            })
            .flatten()
            .sum::<usize>()
    );

    println!(
        "\t2: {}",
        INPUT
            .split_whitespace()
            .map(|line| {
                let mut digits = vec![];
                for (i, c) in line.char_indices() {
                    if c.is_digit(10) {
                        digits.push(c);
                    }
                    if i < line.len() {
                        if i >= 2 {
                            match &line[i - 2..=i] {
                                "one" => digits.push('1'),
                                "two" => digits.push('2'),
                                "six" => digits.push('6'),
                                _ => {}
                            }
                        }
                        if i >= 3 {
                            match &line[i - 3..=i] {
                                "four" => digits.push('4'),
                                "five" => digits.push('5'),
                                "nine" => digits.push('9'),
                                _ => {}
                            }
                        }
                        if i >= 4 {
                            match &line[i - 4..=i] {
                                "three" => digits.push('3'),
                                "seven" => digits.push('7'),
                                "eight" => digits.push('8'),
                                _ => {}
                            }
                        }
                    }
                }
                format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<usize>()
            })
            .flatten()
            .sum::<usize>()
    );

    Ok(())
}

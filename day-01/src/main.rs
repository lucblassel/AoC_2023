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

fn check_spelled_digits(line: &str, i: usize) -> Option<char> {
    let mut digit = None;
    if i < line.len() {
        if i >= 2 {
            digit = match &line[i - 2..=i] {
                "one" => Some('1'),
                "two" => Some('2'),
                "six" => Some('6'),
                _ => None,
            }
        }
        if i >= 3 {
            digit = match &line[i - 3..=i] {
                "four" => Some('4'),
                "five" => Some('5'),
                "nine" => Some('9'),
                _ => None,
            }
        }
        if i >= 4 {
            digit = match &line[i - 4..=i] {
                "three" => Some('3'),
                "seven" => Some('7'),
                "eight" => Some('8'),
                _ => None,
            }
        }
    }

    digit
}

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
                    if let Some(digit) = check_spelled_digits(line, i) {
                        digits.push(digit)
                    }
                }
                format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<usize>()
            })
            .flatten()
            .sum::<usize>()
    );

    Ok(())
}

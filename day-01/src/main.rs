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

fn main() {
    println!("Day 01");
    println!("\t1: {}", part_1(INPUT));
    println!("\t2: {}", part_2(INPUT));
}

fn parse_spelled_digits(line: &str, i: usize, digits: &mut Vec<char>) {
    if i < line.len() {
        if i >= 2 {
            if let Some(digit) = match &line[i - 2..=i] {
                "one" => Some('1'),
                "two" => Some('2'),
                "six" => Some('6'),
                _ => None,
            } {
                digits.push(digit)
            }
        }
        if i >= 3 {
            if let Some(digit) = match &line[i - 3..=i] {
                "four" => Some('4'),
                "five" => Some('5'),
                "nine" => Some('9'),
                _ => None,
            } {
                digits.push(digit)
            }
        }
        if i >= 4 {
            if let Some(digit) = match &line[i - 4..=i] {
                "three" => Some('3'),
                "seven" => Some('7'),
                "eight" => Some('8'),
                _ => None,
            } {
                digits.push(digit)
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    input
        .split_whitespace()
        .flat_map(|line| {
            let digits = line.chars().filter(|c| c.is_ascii_digit()).collect_vec();
            format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<usize>()
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    input
        .split_whitespace()
        .flat_map(|line| {
            let digits = line
                .char_indices()
                .flat_map(|(i, c)| {
                    let mut d = vec![];

                    if c.is_ascii_digit() {
                        d.push(c)
                    }

                    parse_spelled_digits(line, i, &mut d);

                    d
                })
                .collect_vec();
            format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(142, part_1(TEST_1))
    }

    #[test]
    fn test_2() {
        assert_eq!(281, part_2(TEST_2))
    }
}

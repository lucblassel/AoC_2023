use std::collections::HashMap;

use anyhow::Result;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[allow(dead_code)]
const TEST_1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

const INPUT: &str = include_str!("../../inputs/day-12.txt");
#[allow(dead_code)]
const ANSWER_1: usize = 7407;
#[allow(dead_code)]
const ANSWER_2: usize = 30568243604962;

fn main() -> Result<()> {
    println!("Day 12");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &'static str) -> Result<usize> {
    let mut cache = HashMap::new();
    Ok(input
        .lines()
        .map(|line| {
            let (_, (springs, counts)) = parse_row_bytes(line).unwrap();
            find_valid_configurations(&springs, &counts, &mut cache)
        })
        .sum())
}

fn part_2(input: &'static str) -> Result<usize> {
    let mut cache = HashMap::new();
    Ok(input
        .lines()
        .map(|line| {
            let (_, (o_springs, o_counts)) = parse_row_bytes(line).unwrap();
            let (mut springs, mut counts) = (vec![], vec![]);

            for i in 0..5 {
                if i != 0 {
                    springs.push(b'?')
                };
                springs.extend_from_slice(&o_springs[..]);
                counts.extend_from_slice(&o_counts[..]);
            }
            find_valid_configurations(&springs, &counts, &mut cache)
        })
        .sum())
}

fn find_valid_configurations(
    springs: &[u8],
    counts: &[u8],
    cache: &mut HashMap<(String, String), usize>,
) -> usize {
    if springs.is_empty() {
        if counts.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    let seq_s = String::from_utf8(springs.to_vec()).unwrap();
    let count_s = String::from_utf8(counts.to_vec()).unwrap();
    let key = (seq_s, count_s);

    // Check cache
    if let Some(val) = cache.get(&key) {
        return *val;
    }

    let sum = counts.iter().fold(0, |acc, v| acc + *v as usize);
    let n_matches = if springs.len() < sum {
        0
    } else {
        match springs[0] {
            b'.' => find_valid_configurations(&springs[1..], counts, cache),
            b'?' => {
                let mut resolved = vec![b'#'];
                resolved.extend_from_slice(&springs[1..]);
                find_valid_configurations(&springs[1..], counts, cache)
                    + find_valid_configurations(&resolved, counts, cache)
            }
            b'#' => {
                // Remaining # but no counts left -> invalid
                if counts.is_empty() {
                    0
                } else {
                    // Do some pruning
                    let n = counts[0] as usize;
                    let first_dot = springs
                        .iter()
                        .enumerate()
                        .find(|(_, &c)| c == b'.')
                        .map(|(i, _)| i)
                        .unwrap_or(springs.len());

                    if first_dot < n {
                        // we cannot fit enough # before the first .
                        0
                    } else if springs[n..].is_empty() {
                        // Consume block of # or ?
                        find_valid_configurations(&springs[n..], &counts[1..], cache)
                    } else if springs[n] == b'#' {
                        // Fail because block is too big
                        0
                    } else {
                        find_valid_configurations(&springs[n + 1..], &counts[1..], cache)
                    }
                }
            }
            _ => unreachable!(),
        }
    };

    // Cache result
    cache.insert(key, n_matches);

    n_matches
}

fn parse_row_bytes(input: &str) -> IResult<&str, (Box<[u8]>, Box<[u8]>)> {
    let (r, (seq, counts)) = separated_pair(is_a(".#?"), space1, parse_counts)(input)?;
    Ok((r, (seq.as_bytes().into(), counts.into())))
}

fn parse_counts(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), map_res(digit1, str::parse))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(21, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(ANSWER_1, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_2() {
        assert_eq!(525152, part_2(TEST_1).unwrap());
    }

    #[test]
    fn test_input_2() {
        assert_eq!(ANSWER_2, part_2(INPUT).unwrap());
    }
}

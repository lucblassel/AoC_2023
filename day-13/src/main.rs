use anyhow::{bail, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::is_a,
    character::complete::newline,
    multi::{many1, separated_list1},
    IResult,
};

#[allow(dead_code)]
const TEST_1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

const INPUT: &str = include_str!("../../inputs/day-13.txt");
const ANSWER_1: usize = 40006;
const ANSWER_2: usize = 28627;

fn main() -> Result<()> {
    println!("Day 13");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &'static str) -> Result<usize> {
    let (_, maps) = parse_maps(input)?;

    let scores: Result<Vec<_>> = maps.iter().map(|m| m.reflection_score(false)).collect();
    Ok(scores?.into_iter().sum())
}

fn part_2(input: &'static str) -> Result<usize> {
    let (_, maps) = parse_maps(input)?;

    let scores: Result<Vec<_>> = maps.iter().map(|m| m.reflection_score(true)).collect();
    Ok(scores?.into_iter().sum())
}

struct Map {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Map {
    fn reflection_score(&self, smudged: bool) -> Result<usize> {
        if let Some(i) = Self::find_reflection(&self.rows, smudged) {
            Ok(i * 100)
        } else if let Some(j) = Self::find_reflection(&self.cols, smudged) {
            Ok(j)
        } else {
            bail!("Excpected one reflection")
        }
    }

    fn find_reflection(axes: &[u32], smudged: bool) -> Option<usize> {
        axes.iter()
            .enumerate()
            .tuple_windows()
            .find(|((i1, _), (i2, _))| {
                let mut indices = (0..=*i1).rev().zip(*i2..axes.len());
                if smudged {
                    indices
                        .map(|(il, ir)| {
                            let n = axes[il] ^ axes[ir];
                            if n == 0 {
                                // Perfect reflection
                                0
                            } else if n & (n - 1) == 0 {
                                // Exactly 1 bit off
                                1
                            } else {
                                // More than 1 bitwise difference
                                2
                            }
                        })
                        .sum::<u32>()
                        == 1
                } else {
                    indices.all(|(il, ir)| axes[il] == axes[ir])
                }
            })
            .map(|(_, (i, _))| i)
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (r, map) = separated_list1(newline, is_a(".#"))(input)?;

    let mut rows = Vec::with_capacity(map.len());
    let mut cols = vec![0; map[0].len()];
    for row in map {
        let mut row_u = 0;
        for (col, c) in row.chars().enumerate() {
            let v = match c {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            };

            row_u = (row_u << 1) | v;
            cols[col] = (cols[col] << 1) | v;
        }
        rows.push(row_u);
    }

    Ok((r, Map { rows, cols }))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(many1(newline), parse_map)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(405, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(ANSWER_1, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_2() {
        assert_eq!(400, part_2(TEST_1).unwrap());
    }

    #[test]
    fn test_input_2() {
        assert_eq!(ANSWER_2, part_2(INPUT).unwrap());
    }
}

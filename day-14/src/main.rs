use std::{collections::HashMap, ops::Range};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[allow(dead_code)]
const TEST_1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

const INPUT: &str = include_str!("../../inputs/day-14.txt");
pub const ANSWER_1: usize = 109098;
pub const ANSWER_2: usize = 100064;

fn main() -> Result<()> {
    println!("Day 14");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let (boulders, _, ver_ranges, n_rows) = parse_dish(input)?;

    let new_pos = get_new_boulder_positions(&ver_ranges, &boulders, false, true);

    let v = new_pos
        .iter()
        .map(|(row, _)| n_rows - row + 1)
        .sum::<usize>();

    Ok(v)
}

fn get_new_boulder_positions(
    ranges: &[Vec<Range<usize>>],
    boulders: &[(usize, usize)],
    horizontal: bool,
    towards_start: bool,
) -> Vec<(usize, usize)> {
    let mut new_pos = vec![];
    for (axis, ranges) in ranges.iter().enumerate() {
        let candidates = boulders
            .iter()
            .filter(|&(x, y)| {
                if horizontal {
                    *x == axis + 1
                } else {
                    *y == axis + 1
                }
            })
            .collect_vec();

        for range in ranges {
            let n = candidates
                .iter()
                .filter(|&(x, y)| {
                    if horizontal {
                        range.contains(y)
                    } else {
                        range.contains(x)
                    }
                })
                .count();

            let r = if towards_start {
                range.start..range.start + n
            } else {
                range.end - n..range.end
            };

            new_pos.extend(r.map(|coord| {
                if horizontal {
                    (axis + 1, coord)
                } else {
                    (coord, axis + 1)
                }
            }))
        }
    }

    new_pos
}

fn get_ranges(positions: &[Vec<usize>]) -> Vec<Vec<Range<usize>>> {
    positions
        .iter()
        .map(|col| {
            col.iter()
                .tuple_windows()
                .map(|(&up, &down)| up + 1..down)
                .collect()
        })
        .collect()
}

type Ranges<T> = Vec<Vec<Range<T>>>;

fn parse_dish(input: &str) -> Result<(Vec<(usize, usize)>, Ranges<usize>, Ranges<usize>, usize)> {
    let mut lines = input.lines().peekable();
    let n_cols = lines.peek().map(|l| l.len()).context("Expected input")?;

    let mut rolling = vec![vec![]; n_cols];
    let mut fixed = vec![vec![]; n_cols];

    let mut rows = vec![];
    let mut cols = vec![vec![0]; n_cols + 1];
    let mut boulders = vec![];

    let mut n_rows = 0;
    for (row, line) in lines.enumerate() {
        n_rows += 1;
        let mut curr_row = vec![0];
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    // New
                    curr_row.push(col + 1);
                    cols[col].push(row + 1);

                    fixed[col].push(row);
                }
                'O' => {
                    // New
                    boulders.push((row + 1, col + 1));

                    rolling[col].push(row);
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
        curr_row.push(n_cols + 1);
        rows.push(curr_row);
    }

    // Ensure all ranges are closed
    for row in 0..n_cols {
        cols[row].push(n_rows + 1);
    }

    let v_ranges = get_ranges(&cols);
    let h_ranges = get_ranges(&rows);

    Ok((boulders, h_ranges, v_ranges, n_rows))
}

fn part_2(input: &str) -> Result<usize> {
    let (boulders, h_ranges, v_ranges, n_rows) = parse_dish(input)?;

    let mut cache = HashMap::new();
    let mut new = boulders.clone();

    let mut tracker: Vec<Vec<(usize, usize)>> = vec![];

    for i in 0..1000 {
        new = cycle_through(&new, &h_ranges, &v_ranges);
        if let Some(iter) = cache.get(&new.iter().map(|(x, y)| (*x, *y)).sorted().collect_vec()) {
            let cycle_length: usize = i - iter;
            let ending_pos: usize = iter + (1000000000 - i - 1) % cycle_length;

            return Ok(tracker[ending_pos]
                .iter()
                .map(|(row, _)| n_rows - row + 1)
                .sum());
        } else {
            cache.insert(new.iter().map(|(x, y)| (*x, *y)).sorted().collect_vec(), i);
            tracker.push(new.clone());
        }
    }

    bail!("Could not find a cycle in a reasonnable amount of time")
}

fn cycle_through(
    boulders: &[(usize, usize)],
    h_ranges: &Ranges<usize>,
    v_ranges: &Ranges<usize>,
) -> Vec<(usize, usize)> {
    let mut new = boulders.iter().map(|(x, y)| (*x, *y)).collect_vec();
    for (horizontal, towards_start) in [(false, true), (true, true), (false, false), (true, false)]
    // N, W, S, E
    {
        let ranges = if horizontal { h_ranges } else { v_ranges };
        new = get_new_boulder_positions(ranges, &new, horizontal, towards_start);
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(136, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(ANSWER_1, part_1(INPUT).unwrap());
    }

    fn get_boulder_pos(input: &str) -> Vec<(usize, usize)> {
        input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| matches!(c, 'O'))
                    .map(|(y, _)| (x + 1, y + 1))
                    .collect_vec()
            })
            .sorted()
            .collect()
    }

    #[test]
    fn test_cycling() {
        let (boulders, h_ranges, v_ranges, _) = parse_dish(TEST_1).unwrap();

        let after_1 = get_boulder_pos(
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        );

        let new_1 = cycle_through(&boulders, &h_ranges, &v_ranges);

        assert_eq!(
            after_1,
            new_1.iter().map(|(x, y)| (*x, *y)).sorted().collect_vec(),
            "Problem in cycle 1:\nwanted{after_1:?}\ngot: {new_1:?}"
        );

        let after_2 = get_boulder_pos(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        );

        let new_2 = cycle_through(&new_1, &h_ranges, &v_ranges);

        assert_eq!(
            after_2,
            new_2.iter().map(|(x, y)| (*x, *y)).sorted().collect_vec(),
            "Problem in cycle 2:\nwanted{after_1:?}\ngot: {new_2:?}"
        );
        let after_3 = get_boulder_pos(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        );

        let new_3 = cycle_through(&new_2, &h_ranges, &v_ranges);

        assert_eq!(
            after_3,
            new_3.iter().map(|(x, y)| (*x, *y)).sorted().collect_vec(),
            "Problem in cycle 3:\nwanted{after_1:?}\ngot: {new_3:?}"
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(64, part_2(TEST_1).unwrap());
    }

    #[test]
    fn test_input_2() {
        assert_eq!(ANSWER_2, part_2(INPUT).unwrap());
    }
}

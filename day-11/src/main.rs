use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

#[allow(dead_code)]
const TEST_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

const INPUT: &str = include_str!("../../inputs/day-11.txt");

fn main() -> Result<()> {
    let (galaxies, empty_rows, empty_cols) = parse_galaxies(INPUT);

    println!("Day 11");
    println!("\t1: {}", part_1(&galaxies, &empty_rows, &empty_cols, 1)?);
    println!(
        "\t2: {}",
        part_1(&galaxies, &empty_rows, &empty_cols, 1000000 - 1)?
    );

    Ok(())
}

type Galaxy = (usize, usize);
fn parse_galaxies(input: &str) -> (Vec<Galaxy>, Vec<usize>, Vec<usize>) {
    let mut galaxies = vec![];
    let mut seen_cols = HashSet::new();
    let mut seen_rows = HashSet::new();
    let mut max_col = 0;
    let mut max_row = 0;

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                seen_rows.insert(x);
                seen_cols.insert(y);
                max_row = max_row.max(x);
                max_col = max_col.max(y);
                galaxies.push((x, y));
            }
        }
    }

    let empty_rows = (0..=max_row)
        .into_iter()
        .filter(|x| !seen_rows.contains(x))
        .collect_vec();
    let empty_cols = (0..=max_col)
        .into_iter()
        .filter(|y| !seen_cols.contains(y))
        .collect_vec();

    (galaxies, empty_rows, empty_cols)
}

fn part_1(
    galaxies: &[Galaxy],
    empty_rows: &[usize],
    empty_cols: &[usize],
    offset: usize, // Number of additional rows/cols
) -> Result<usize> {
    Ok(galaxies
        .into_iter()
        .map(|(x, y)| {
            let expanded_x = x + offset * empty_rows.iter().filter(|&&row| row < *x).count();
            let expanded_y = y + offset * empty_cols.iter().filter(|&&col| col < *y).count();

            (expanded_x, expanded_y)
        })
        .combinations(2)
        .map(|v| {
            let (x1, y1) = v[0];
            let (x2, y2) = v[1];

            (x1.max(x2) - x1.min(x2)) + (y1.max(y2) - y1.min(y2))
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (galaxies, empty_rows, empty_cols) = parse_galaxies(TEST_1);
        assert_eq!(374, part_1(&galaxies, &empty_rows, &empty_cols, 1).unwrap());
    }

    #[test]
    fn test_input_1() {
        let (galaxies, empty_rows, empty_cols) = parse_galaxies(INPUT);
        assert_eq!(
            10173804,
            part_1(&galaxies, &empty_rows, &empty_cols, 1).unwrap()
        );
    }

    #[test]
    fn test_2() {
        let (galaxies, empty_rows, empty_cols) = parse_galaxies(TEST_1);

        for (off, dist) in [(10, 1030), (100, 8410)] {
            assert_eq!(
                dist,
                part_1(&galaxies, &empty_rows, &empty_cols, off - 1).unwrap()
            );
        }
    }

    #[test]
    fn test_input_2() {
        let (galaxies, empty_rows, empty_cols) = parse_galaxies(INPUT);
        assert_eq!(
            634324905172,
            part_1(&galaxies, &empty_rows, &empty_cols, 1000000 - 1).unwrap()
        );
    }
}

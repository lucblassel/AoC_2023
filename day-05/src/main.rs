use std::ops::Range;

use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take, take_till},
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[allow(dead_code)]
const TEST_1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

const INPUT: &str = include_str!("../../inputs/day-05.txt");

// Source range -> Destination range
type MapEntry<T> = (Range<T>, Range<T>);
type Map<T> = Vec<MapEntry<T>>;

fn main() -> Result<()> {
    let (remaining, seeds) = parse_seeds(INPUT).context("Error parsing seeds")?;
    let (_, maps) =
        separated_list1(many1(newline), parse_map)(remaining).context("Erorr parsing maps")?;

    println!("Day 05");
    println!("\t1: {}", part_1(&seeds, &maps)?);
    println!("\t2: {}", part_2(&seeds, &maps)?);

    Ok(())
}

fn part_1(seeds: &[usize], maps: &[Map<usize>]) -> Result<usize> {
    let mut curr = seeds.iter().cloned().collect_vec();
    for map in maps.iter() {
        curr = curr
            .iter()
            .map(|seed| {
                match map
                    .iter()
                    .map(|(src, dest)| src.contains(seed).then(|| dest.start + seed - src.start))
                    .find(|v| v.is_some())
                {
                    None => *seed,
                    Some(Some(x)) => x,
                    _ => unreachable!(),
                }
            })
            .collect_vec();
    }

    curr.into_iter()
        .min()
        .context("Error getting minimum location")
}

fn part_2(seeds: &[usize], maps: &[Map<usize>]) -> Result<usize> {
    let mut seed_ranges = seeds.chunks(2).map(|s| (s[0]..(s[0] + s[1]))).collect_vec();

    for map in maps.iter() {
        seed_ranges = seed_ranges
            .iter()
            .flat_map(|seed_range| {
                let intersections = map
                    .iter()
                    .flat_map(move |(src, dest)| {
                        range_intersect(seed_range.clone(), src.clone()).map(|inter| {
                            let off_s = inter.start - src.start;
                            let off_e = src.end - inter.end;

                            (inter, (dest.start + off_s)..(dest.end - off_e))
                        })
                    })
                    .collect_vec();

                let mut valid_ranges = vec![];
                let mut remainder = vec![seed_range.clone()];

                for (inter, mapped) in intersections {
                    valid_ranges.push(mapped);
                    remainder = remainder
                        .iter()
                        .flat_map(|r| exclude_from_range(r.clone(), inter.clone()))
                        .collect_vec();
                }

                valid_ranges.extend_from_slice(&remainder[..]);
                valid_ranges
            })
            .collect_vec();
    }

    seed_ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .context("Error getting minimum location")
}

fn range_intersect<T: PartialOrd + Ord>(a: Range<T>, b: Range<T>) -> Option<Range<T>> {
    if b.start >= a.end || a.start >= b.end {
        return None;
    }

    let s = a.start.max(b.start);
    let e = a.end.min(b.end);

    Some(s..e)
}

fn exclude_from_range<T: PartialOrd + Ord>(
    reference: Range<T>,
    excluding: Range<T>,
) -> Vec<Range<T>> {
    let mut splits = vec![];

    if reference.start < excluding.start {
        splits.push(reference.start..excluding.start)
    }

    if excluding.end < reference.end {
        splits.push(excluding.end..reference.end)
    }

    splits
}

// Parsing functions

fn parse_ranges(input: &str) -> IResult<&str, MapEntry<usize>> {
    let (r, nums) = separated_list1(space1, map_res(digit1, str::parse))(input)?;
    let (dest, src, len) = (nums[0], nums[1], nums[2]);

    Ok((r, (src..(src + len), dest..(dest + len))))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tuple((tag("seeds:"), space1)),
        separated_list1(space1, map_res(digit1, str::parse)),
        newline,
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Map<usize>> {
    let (r, _) = take_till(|c| c == ':')(input)?;
    let (r, _) = take(1usize)(r)?; // Consume ':'
    preceded(newline, separated_list1(newline, parse_ranges))(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &'static str) -> Result<(Vec<usize>, Vec<Map<usize>>)> {
        let (remaining, seeds) = parse_seeds(input).context("Error parsing seeds")?;
        let (_, maps) =
            separated_list1(many1(newline), parse_map)(remaining).context("Erorr parsing maps")?;

        Ok((seeds, maps))
    }

    #[test]
    fn test_ranges() {
        let cases = [
            ((0..6, 3..5), Some(3..5)),
            ((0..6, 8..10), None),
            ((8..10, 0..6), None),
            ((10..100, 90..200), Some(90..100)),
            ((90..200, 10..100), Some(90..100)),
        ];

        for ((a, b), wanted) in cases {
            assert_eq!(wanted, range_intersect(a, b))
        }
    }

    #[test]
    fn test_1() {
        let (seeds, maps) = parse(TEST_1).unwrap();
        assert_eq!(35, part_1(&seeds, &maps).unwrap())
    }

    #[test]
    fn test_input_1() {
        let (seeds, maps) = parse(INPUT).unwrap();
        assert_eq!(157211394, part_1(&seeds, &maps).unwrap())
    }

    #[test]
    fn test_2() {
        let (seeds, maps) = parse(TEST_1).unwrap();
        assert_eq!(46, part_2(&seeds, &maps).unwrap())
    }

    #[test]
    fn test_input_2() {
        let (seeds, maps) = parse(INPUT).unwrap();
        assert_eq!(50855035, part_2(&seeds, &maps).unwrap())
    }
}

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

#[allow(dead_code)]
const TEST_1_1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
#[allow(dead_code)]
const TEST_1_2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
#[allow(dead_code)]
const TEST_2_1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
#[allow(dead_code)]
const TEST_2_2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
#[allow(dead_code)]
const TEST_2_3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

const INPUT: &str = include_str!("../../inputs/day-10.txt");

fn main() -> Result<()> {
    println!("Day XX");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find starting position
    let (s_x, s_y) = grid
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .find(|(_, &c)| c == 'S')
                .map(|(y, _)| (x, y))
        })
        .next()
        .unwrap();

    let connect = get_connect(&grid, s_x, s_y)[0];
    let (mut x, mut y) = match connect {
        Dir::North => (s_x - 1, s_y),
        Dir::South => (s_x + 1, s_y),
        Dir::East => (s_x, s_y + 1),
        Dir::West => (s_x, s_y - 1),
    };
    let mut coming_from = connect.from();
    let mut len_path = 1;

    eprintln!();

    while x != s_x || y != s_y {
        (x, y, coming_from) = match (grid[x][y], coming_from) {
            ('|', Dir::North) => (x + 1, y, coming_from),
            ('|', Dir::South) => (x - 1, y, coming_from),
            ('-', Dir::East) => (x, y - 1, coming_from),
            ('-', Dir::West) => (x, y + 1, coming_from),
            ('L', Dir::North) => (x, y + 1, Dir::West),
            ('L', Dir::East) => (x - 1, y, Dir::South),
            ('J', Dir::North) => (x, y - 1, Dir::East),
            ('J', Dir::West) => (x - 1, y, Dir::South),
            ('7', Dir::South) => (x, y - 1, Dir::East),
            ('7', Dir::West) => (x + 1, y, Dir::North),
            ('F', Dir::South) => (x, y + 1, Dir::West),
            ('F', Dir::East) => (x + 1, y, Dir::North),
            _ => unreachable!(),
        };
        len_path += 1;
    }

    eprintln!("{len_path} -> {}", (len_path as f64 / 2.).ceil() as usize);

    Ok(len_path / 2)
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn from(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

fn get_connect(grid: &[Vec<char>], x: usize, y: usize) -> Vec<Dir> {
    let mut dirs = vec![];
    // Check N
    if x > 0 && matches!(grid[x - 1][y], '|' | '7' | 'F') {
        dirs.push(Dir::North);
    }
    // Check S
    if matches!(grid[x + 1][y], '|' | 'L' | 'J') {
        dirs.push(Dir::South);
    }
    // Check E
    if matches!(grid[x][y + 1], '-' | '7' | 'J') {
        dirs.push(Dir::East);
    }
    // Check W
    if y > 0 && matches!(grid[x][y - 1], '-' | 'L' | 'F') {
        dirs.push(Dir::West);
    }

    dirs
}

fn show_loop(
    grid: &[Vec<char>],
    tiles: &HashMap<(usize, usize), char>,
    x: usize,
    y: usize,
    ch: char,
) {
    for (x_, line) in grid.iter().enumerate() {
        for (y_, c) in line.iter().enumerate() {
            if x_ == x && y_ == y {
                eprint!("{ch}")
            } else if tiles.get(&(x_, y_)).is_some() {
                eprint!(
                    "{}",
                    match c {
                        '|' => '║',
                        '-' => '═',
                        'L' => '╚',
                        'J' => '╝',
                        '7' => '╗',
                        'F' => '╔',
                        '.' => ' ',
                        'S' => '*',
                        _ => unreachable!(),
                    }
                )
            } else {
                eprint!(".")
            }
        }
        eprintln!()
    }
}

fn part_2(input: &str) -> Result<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find starting position
    let (s_x, s_y) = grid
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .find(|(_, &c)| c == 'S')
                .map(|(y, _)| (x, y))
        })
        .next()
        .unwrap();

    // Get loop tiles
    let connect = get_connect(&grid, s_x, s_y)[0];
    let (mut x, mut y) = match connect {
        Dir::North => (s_x - 1, s_y),
        Dir::South => (s_x + 1, s_y),
        Dir::East => (s_x, s_y + 1),
        Dir::West => (s_x, s_y - 1),
    };
    let mut coming_from = connect.from();
    let mut tiles = HashMap::new();
    tiles.insert((s_x, s_y), 'S');

    while x != s_x || y != s_y {
        tiles.insert((x, y), grid[x][y]);
        (x, y, coming_from) = match (grid[x][y], coming_from) {
            ('|', Dir::North) => (x + 1, y, coming_from),
            ('|', Dir::South) => (x - 1, y, coming_from),
            ('-', Dir::East) => (x, y - 1, coming_from),
            ('-', Dir::West) => (x, y + 1, coming_from),
            ('L', Dir::North) => (x, y + 1, Dir::West),
            ('L', Dir::East) => (x - 1, y, Dir::South),
            ('J', Dir::North) => (x, y - 1, Dir::East),
            ('J', Dir::West) => (x - 1, y, Dir::South),
            ('7', Dir::South) => (x, y - 1, Dir::East),
            ('7', Dir::West) => (x + 1, y, Dir::North),
            ('F', Dir::South) => (x, y + 1, Dir::West),
            ('F', Dir::East) => (x + 1, y, Dir::North),
            _ => unreachable!(),
        };
    }

    show_loop(&grid, &tiles, x, y, 'X');
    panic!();

    eprintln!("{tiles:?}");
    let mut n = 0;
    for x in 0..(grid.len()) {
        for y in 0..(grid[0].len()) {
            if tiles.get(&(x, y)).is_some() {
                continue;
            }
            if is_inside(&tiles, x, y) {
                show_loop(&grid, &tiles, x, y, 'I');
                n += 1;
            } else {
                show_loop(&grid, &tiles, x, y, 'O');
            }
        }
    }

    eprintln!("Inside: {n}");

    todo!()
}

fn does_connect(p1: char, p2: char, dir: Dir) -> bool {
    match dir {
        // Check if N(p1)->S(p2) works
        Dir::South => match (p1, p2) {
            ('|', '|' | 'L' | 'J') => true,
            ('7', '|' | 'L' | 'J') => true,
            ('F', '|' | 'L' | 'J') => true,
            _ => false,
        },
        // Check if W(p1)->E(p2) works
        Dir::East => match (p1, p2) {
            ('-', '-' | '7' | 'J') => true,
            ('F', '-' | '7' | 'J') => true,
            ('L', '-' | '7' | 'J') => true,
            _ => false,
        },
        _ => unreachable!("Could not connect {p1} and {p2} along {dir:?}"),
    }
}

fn is_inside(tiles: &HashMap<(usize, usize), char>, x: usize, y: usize) -> bool {
    if tiles.get(&(x, y)).is_some() {
        return false;
    }

    let h_ray = (0..=y)
        .map(|c_y| tiles.get(&(x, c_y)).unwrap_or(&'.'))
        .collect_vec();
    let v_ray = (0..=x)
        .map(|c_x| tiles.get(&(c_x, y)).unwrap_or(&'.'))
        .collect_vec();

    eprintln!("({x},{y})");
    eprintln!("\tH: {h_ray:?}");
    eprint!("\t\t");

    let mut h_chunks = 0;
    for (k, group) in &h_ray
        .into_iter()
        .tuple_windows()
        .group_by(|(&p1, &p2)| match (p1, p2) {
            ('.', _) => false,
            (_, '.') => false,
            (p1, p2) => does_connect(p1, p2, Dir::East),
        })
    {
        let v = group.collect::<Vec<_>>();
        eprint!("{k}{v:?} ");
        if k {
            if v.len() > 1 {
                h_chunks += 2;
            } else {
                h_chunks += 1;
            }
        }
    }
    eprintln!();

    // for (k, group) in &h_ray.iter().group_by(|&&&elt| elt == '.') {
    //     eprint!("");
    //     // eprint!("{:?} ", group.collect::<Vec<_>>());
    //     if !k {
    //         // An edge
    //         let group = group.collect::<Vec<_>>();
    //         if group.len() > 1 {
    //             let s = group
    //                 .into_iter()
    //                 .tuple_windows()
    //                 .all(|(&&c1, &&c2)| does_connect(c1, c2, Dir::South));
    //             eprintln!("S:{s:?} ");
    //             h_chunks += 2
    //         } else {
    //             h_chunks += 1
    //         }
    //     } else {
    //         eprint!("-- ");
    //     }
    // }
    eprintln!("");

    eprintln!("\tV: {v_ray:?}");
    eprint!("\t\t");
    let mut v_chunks = 0;
    for (k, group) in &v_ray
        .into_iter()
        .tuple_windows()
        .group_by(|(&p1, &p2)| match (p1, p2) {
            ('.', _) => false,
            (_, '.') => false,
            (p1, p2) => does_connect(p1, p2, Dir::South),
        })
    {
        let v = group.collect::<Vec<_>>();
        eprint!("{k}{v:?} ");
        // eprint!("{k}{:?} ", group.collect::<Vec<_>>());
        if k {
            if v.len() > 1 {
                v_chunks += 2;
            } else {
                v_chunks += 1;
            }
        }
    }
    eprintln!();

    //
    // let h_ray_sum = h_ray.into_iter().filter(|c| *c).count();
    // let v_ray_sum = v_ray.into_iter().filter(|c| *c).count();

    v_chunks % 2 == 1 && h_chunks % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        assert_eq!(4, part_1(TEST_1_1).unwrap());
    }

    #[test]
    fn test_1_2() {
        assert_eq!(8, part_1(TEST_1_2).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(6951, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_2_0() {
        assert_eq!(4, part_2(TEST_1_1).unwrap());
    }

    #[test]
    fn test_2_1() {
        assert_eq!(4, part_2(TEST_2_1).unwrap());
    }

    // #[test]
    // fn test_2_2() {
    //     assert_eq!(8, part_2(TEST_2_2).unwrap());
    // }

    // #[test]
    // fn test_2_3() {
    //     assert_eq!(10, part_2(TEST_2_3).unwrap());
    // }

    // #[test]
    // fn test_input_2() {
    //     assert_eq!(0, part_2(INPUT).unwrap());
    // }
}

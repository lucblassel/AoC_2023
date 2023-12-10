use std::collections::HashMap;

use anyhow::Result;

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
    println!("Day 10");
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

#[allow(dead_code)]
fn show_loop(
    grid: &[Vec<char>],
    tiles: &HashMap<(usize, usize), char>,
    x: usize,
    y: usize,
    ch: char,
) {
    for (x_, line) in grid.iter().enumerate() {
        for (y_, _) in line.iter().enumerate() {
            if x_ == x && y_ == y {
                eprint!("{ch}")
            } else if let Some(c) = tiles.get(&(x_, y_)) {
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
    let connects = get_connect(&grid, s_x, s_y);
    let (mut x, mut y) = match connects[0] {
        Dir::North => (s_x - 1, s_y),
        Dir::South => (s_x + 1, s_y),
        Dir::East => (s_x, s_y + 1),
        Dir::West => (s_x, s_y - 1),
    };
    let start_tile = match (connects[0], connects[1]) {
        (Dir::North, Dir::South) | (Dir::South, Dir::North) => '|',
        (Dir::North, Dir::East) | (Dir::East, Dir::North) => 'L',
        (Dir::North, Dir::West) | (Dir::West, Dir::North) => 'J',
        (Dir::South, Dir::East) | (Dir::East, Dir::South) => 'F',
        (Dir::South, Dir::West) | (Dir::West, Dir::South) => '7',
        (Dir::East, Dir::West) | (Dir::West, Dir::East) => '-',
        _ => unreachable!(),
    };
    let mut coming_from = connects[0].from();
    let mut tiles = HashMap::new();
    tiles.insert((s_x, s_y), start_tile);

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

    let mut n = 0;
    for x in 0..(grid.len()) {
        for y in 0..(grid[0].len()) {
            if tiles.get(&(x, y)).is_some() {
                continue;
            }
            if is_inside(&tiles, x, y) {
                n += 1;
            }
        }
    }

    Ok(n)
}

fn is_inside(tiles: &HashMap<(usize, usize), char>, x: usize, y: usize) -> bool {
    if tiles.get(&(x, y)).is_some() {
        return false;
    }

    let mut crosses = 0;
    let mut edge_start = '.';
    for c_y in 0..=y {
        if let Some(c) = tiles.get(&(x, c_y)) {
            match c {
                '|' => crosses += 1,
                'L' => {
                    edge_start = 'L';
                    crosses += 1;
                }
                'J' => {
                    if edge_start == 'L' {
                        crosses += 1;
                    }
                    edge_start = '.';
                }
                'F' => {
                    edge_start = 'F';
                    crosses += 1;
                }
                '7' => {
                    if edge_start == 'F' {
                        crosses += 1;
                    }
                    edge_start = '.';
                }
                _ => {}
            }
        }
    }

    crosses % 2 == 1
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
        assert_eq!(1, part_2(TEST_1_1).unwrap());
    }

    #[test]
    fn test_2_1() {
        assert_eq!(4, part_2(TEST_2_1).unwrap());
    }

    #[test]
    fn test_2_2() {
        assert_eq!(8, part_2(TEST_2_2).unwrap());
    }

    #[test]
    fn test_2_3() {
        assert_eq!(10, part_2(TEST_2_3).unwrap());
    }

    // #[test]
    // fn test_input_2() {
    //     assert_eq!(0, part_2(INPUT).unwrap());
    // }
}

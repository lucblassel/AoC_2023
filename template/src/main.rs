#[allow(dead_code)]
const TEST_1: &str = "";
const INPUT: &str = include_str!("../../inputs/{{project-name}}.txt");

fn main() {
    println!("Day XX:");
    println!("\t1: {}", part_1(INPUT));
    println!("\t2: {}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    todo!()
}

fn part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, part_1(TEST_1));
    }

    #[test]
    fn test_input_1() {
        assert_eq!(0, part_1(INPUT));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, part_2(TEST_1));
    }

    #[test]
    fn test_input_2() {
        assert_eq!(0, part_2(INPUT));
    }
}

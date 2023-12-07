use anyhow::{bail, Result};
use itertools::Itertools;

#[allow(dead_code)]
const TEST_1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
const INPUT: &str = include_str!("../../inputs/day-07.txt");

fn main() -> Result<()> {
    println!("Day 07");
    println!("\t1: {}", part_1(INPUT)?);
    println!("\t2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let cards: String = (0..5).flat_map(|_| chars.next()).collect();
            let bid: usize = chars
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse()
                .unwrap();

            (Draw::from_str(&cards, false).unwrap(), bid)
        })
        .sorted_by(|(cards_a, _), (cards_b, _)| cards_a.cmp(cards_b))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum())
}

fn part_2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let draw = Draw::from_str(
                &((0..5).flat_map(|_| chars.next()).collect::<String>()),
                true,
            )
            .unwrap();
            let bid: usize = chars
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse()
                .unwrap();

            (draw.resolve_joker(), bid)
        })
        .sorted_by(|(cards_a, _), (cards_b, _)| cards_a.cmp(cards_b))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Cards([u8; 5]);

impl Cards {
    fn from_str(s: &str, with_jokers: bool) -> Result<Self> {
        if s.len() != 5 {
            bail!("A Camel cards hand must have 5 cards")
        }

        let mut cards = [0; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = match c {
                'T' => 9,
                'J' => {
                    if with_jokers {
                        0
                    } else {
                        10
                    }
                }
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (c as u32 - 48) as u8 - 1,
                c => bail!("Unknown card type {c}"),
            };
        }

        Ok(Self(cards))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Draw {
    hand: Hand,
    cards: Cards,
    s: String,
}

impl Draw {
    fn from_str(s: &str, with_jokers: bool) -> Result<Self> {
        let cards = Cards::from_str(s, with_jokers)?;
        let hand = Hand::from_cards(&cards);

        Ok(Self {
            hand,
            cards,
            s: s.to_string(),
        })
    }

    fn resolve_joker(&self) -> Self {
        let n_jokers = self.cards.0.iter().filter(|&v| v == &0).count();

        let new_hand = match n_jokers {
            4 => Hand::Five,
            3 => match self.hand {
                Hand::FullHouse => Hand::Five,
                Hand::Three => Hand::Four,
                _ => unreachable!(),
            },
            2 => match self.hand {
                Hand::FullHouse => Hand::Five,
                Hand::DoublePair => Hand::Four,
                Hand::Pair => Hand::Three,
                _ => unreachable!(),
            },
            1 => match self.hand {
                Hand::Four => Hand::Five,
                Hand::Three => Hand::Four,
                Hand::Pair => Hand::Three,
                Hand::DoublePair => Hand::FullHouse,
                Hand::HighCard => Hand::Pair,
                _ => unreachable!(),
            },
            _ => self.hand,
        };

        Self {
            hand: new_hand,
            cards: self.cards,
            s: self.s.clone(),
        }
    }
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.hand.cmp(&other.hand) {
            std::cmp::Ordering::Equal => self
                .cards
                .0
                .iter()
                .zip(other.cards.0)
                .map(|(c1, c2)| c1.cmp(&c2))
                .find(|cmp| !matches!(cmp, std::cmp::Ordering::Equal))
                .unwrap_or(std::cmp::Ordering::Equal),
            ord => ord,
        })
    }
}

impl Ord for Draw {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Some(ord) = self.partial_cmp(other) else {
            unreachable!()
        };
        ord
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Hand {
    HighCard,
    Pair,
    DoublePair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl Hand {
    fn hand_order(&self) -> u8 {
        match self {
            Self::HighCard => 0,
            Self::Pair => 1,
            Self::DoublePair => 2,
            Self::Three => 3,
            Self::FullHouse => 4,
            Self::Four => 5,
            Self::Five => 6,
        }
    }

    fn from_cards(cards: &Cards) -> Self {
        let mut counter = [0; 14];
        for card in cards.0.iter() {
            counter[*card as usize] += 1;
        }

        let values = counter.iter().sorted().rev().collect_vec();
        match values[0] {
            5 => Hand::Five,
            4 => Hand::Four,
            3 => match values[1] {
                1 => Hand::Three,
                2 => Hand::FullHouse,
                _ => unreachable!(),
            },
            2 => match values[1] {
                1 => Hand::Pair,
                2 => Hand::DoublePair,
                _ => unreachable!(),
            },
            1 => Hand::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand_order().partial_cmp(&other.hand_order())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_order().cmp(&other.hand_order())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn str_to_cards() {
        let cases = [
            ("32T3K", Cards([2, 1, 9, 2, 12]), Hand::Pair),
            ("T55J5", Cards([9, 4, 4, 10, 4]), Hand::Three),
            ("KK677", Cards([12, 12, 5, 6, 6]), Hand::DoublePair),
            ("KTJJT", Cards([12, 9, 10, 10, 9]), Hand::DoublePair),
            ("QQQJA", Cards([11, 11, 11, 10, 13]), Hand::Three),
            ("QKJA2", Cards([11, 12, 10, 13, 1]), Hand::HighCard),
            ("44444", Cards([3, 3, 3, 3, 3]), Hand::Five),
            ("A2A22", Cards([13, 1, 13, 1, 1]), Hand::FullHouse),
            ("43444", Cards([3, 2, 3, 3, 3]), Hand::Four),
        ];

        for (repr, cards, hand) in cases {
            let hand_cards = Cards::from_str(repr, false).unwrap();
            let hand_type = Hand::from_cards(&hand_cards);
            assert_eq!(hand_cards, cards);
            assert_eq!(hand_type, hand);
        }
    }

    #[test]
    fn test_hands() {
        let cases = [
            (
                Draw::from_str("22345", false).unwrap(),
                Draw::from_str("23456", false).unwrap(),
                Ordering::Greater,
            ),
            (
                Draw::from_str("22345", false).unwrap(),
                Draw::from_str("22333", false).unwrap(),
                Ordering::Less,
            ),
            (
                Draw::from_str("22345", false).unwrap(),
                Draw::from_str("22346", false).unwrap(),
                Ordering::Less,
            ),
            (
                Draw::from_str("22345", false).unwrap(),
                Draw::from_str("22345", false).unwrap(),
                Ordering::Equal,
            ),
        ];

        for (h1, h2, ord) in cases {
            assert_eq!(h1.cmp(&h2), ord)
        }
    }

    #[test]
    fn test_1() {
        assert_eq!(6440, part_1(TEST_1).unwrap());
    }

    #[test]
    fn test_input_1() {
        assert_eq!(251545216, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_2() {
        assert_eq!(5905, part_2(TEST_1).unwrap());
    }

    #[test]
    fn test_input_2() {
        assert_eq!(250384185, part_2(INPUT).unwrap());
    }
}

use advent_of_code::parsers::number_parser;
use enum_iterator::Sequence;
use itertools::Itertools;
use nom::{
    character::complete::{one_of, space1},
    multi::count,
    Parser,
};
use nom_supreme::ParserExt;
use num_enum::TryFromPrimitive;
advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Sequence, TryFromPrimitive)]
#[repr(u8)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        one_of("j23456789TJQKA")
            .map(|c| match c {
                'j' => Card::Joker,
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => unreachable!(),
            })
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[Card]) -> Self {
        let mut card_counts = vec![0; enum_iterator::cardinality::<Card>()];
        enum_iterator::all::<Card>().for_each(|card| {
            card_counts[card as usize] +=
                cards
                    .iter()
                    .fold(0, |acc, c| if *c == card { acc + 1 } else { acc });
        });

        if card_counts[Card::Joker as usize] > 0 {
            return HandType::with_joker(card_counts);
        }

        let two_pair = card_counts.iter().filter(|c| *c == &2).count() == 2;

        match card_counts.iter().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if card_counts.contains(&2) => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if two_pair => HandType::TwoPair,
            2 => HandType::Pair,
            _ => HandType::HighCard,
        }
    }

    fn with_joker(card_counts: Vec<u8>) -> Self {
        let num_jokers = card_counts[Card::Joker as usize];
        let card_counts = &card_counts[Card::Two as usize..];

        let hand_without_jokers = card_counts
            .iter()
            .enumerate()
            .flat_map(|(i, c)| {
                let card = Card::try_from_primitive(i as u8 + 1).unwrap();
                vec![card; *c as usize]
            })
            .collect_vec();
        let hand_type = HandType::from_cards(&hand_without_jokers);

        match (num_jokers, hand_type) {
            (5, _) | (4, _) => HandType::FiveOfAKind,
            (3, HandType::Pair) => HandType::FiveOfAKind,
            (3, HandType::HighCard) => HandType::FourOfAKind,
            (2, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
            (2, HandType::Pair) => HandType::FourOfAKind,
            (2, HandType::HighCard) => HandType::ThreeOfAKind,
            (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
            (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
            (1, HandType::TwoPair) => HandType::FullHouse,
            (1, HandType::Pair) => HandType::ThreeOfAKind,
            (1, HandType::HighCard) => HandType::Pair,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: &[Card], bid: u64) -> Self {
        Self {
            cards: cards.try_into().unwrap(),
            bid,
            hand_type: HandType::from_cards(cards),
        }
    }

    fn parse(input: impl AsRef<str>) -> Self {
        count(Card::parse, 5)
            .terminated(space1)
            .and(number_parser)
            .map(|(cards, bid)| Self::new(&cards, bid))
            .parse(input.as_ref())
            .unwrap()
            .1
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(Hand::parse)
            .sorted()
            .enumerate()
            .map(|(i, h)| h.bid * (i as u64 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.replace('J', "j"))
            .map(Hand::parse)
            .sorted()
            .enumerate()
            .map(|(i, h)| h.bid * (i as u64 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}

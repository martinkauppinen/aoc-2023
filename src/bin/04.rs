use advent_of_code::parsers::{number_parser, space_separated_numbers_parser};
use nohash_hasher::IntMap;
use nom::{
    bytes::complete::tag,
    character::complete::space1,
    sequence::{preceded, separated_pair, terminated},
};

advent_of_code::solution!(4);

#[derive(Debug, Default, Clone)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn count_winning_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }

    pub fn score(self) -> u32 {
        if self.count_winning_numbers() == 0 {
            0
        } else {
            2u32.pow(self.count_winning_numbers() - 1)
        }
    }
}

impl<'a> TryFrom<&'a str> for Card {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut id_parser = terminated(
            preceded(terminated(tag("Card"), space1), number_parser),
            tag(": "),
        );
        let mut split_numbers_parser = separated_pair(
            space_separated_numbers_parser,
            tag(" | "),
            space_separated_numbers_parser,
        );

        let (s, id) = id_parser(s)?;
        let (_, (winning_numbers, numbers)) = split_numbers_parser(s)?;
        Ok(Card {
            id,
            winning_numbers,
            numbers,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Card::try_from)
            .map(Result::unwrap)
            .map(Card::score)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_map = IntMap::<u32, u32>::default();
    input
        .lines()
        .map(Card::try_from)
        .map(Result::unwrap)
        .for_each(|card| {
            let instances = *card_map.entry(card.id).or_insert(1);
            let start = card.id + 1;
            let end = start + card.count_winning_numbers();
            for i in start..end {
                card_map
                    .entry(i)
                    .and_modify(|v| *v += instances)
                    .or_insert(1 + instances);
            }
        });
    Some(card_map.into_values().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

use std::str::FromStr;

use nohash_hasher::IntMap;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list0,
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

impl FromStr for Card {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number_parser = || map_res(digit1::<&str, ()>, str::parse::<u32>);
        let mut id_parser = terminated(
            preceded(terminated(tag("Card"), space1), number_parser()),
            tag(": "),
        );
        let numbers_parser = || separated_list0(space1, preceded(space0, number_parser()));
        let mut split_numbers_parser =
            separated_pair(numbers_parser(), tag(" | "), numbers_parser());

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
            .map(str::parse)
            .map(Result::unwrap)
            .map(Card::score)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_map = IntMap::<u32, u32>::default();
    input
        .lines()
        .map(str::parse::<Card>)
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

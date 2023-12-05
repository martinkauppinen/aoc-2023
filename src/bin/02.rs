use advent_of_code::parsers::number_parser;
use nom::{
    bytes::complete::tag,
    character::complete::alpha0,
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
};

advent_of_code::solution!(2);

type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    pub fn maximize(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let orderings = [
            self.red.cmp(&other.red),
            self.green.cmp(&other.green),
            self.blue.cmp(&other.blue),
        ];
        Some(
            if orderings.iter().all(|o| o != &std::cmp::Ordering::Greater) {
                std::cmp::Ordering::Less
            } else if orderings.iter().all(|o| o == &std::cmp::Ordering::Equal) {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            },
        )
    }
}

fn parse_color(input: &str) -> IResult<CubeColor> {
    map_res(alpha0, |color| match color {
        "red" => Ok(CubeColor::Red),
        "green" => Ok(CubeColor::Green),
        "blue" => Ok(CubeColor::Blue),
        _ => Err(()),
    })(input)
}

fn parse_cube_set(input: &str) -> IResult<CubeSet> {
    let parse_color_count = separated_pair(number_parser::<u32>, tag(" "), parse_color);
    let (rest, color_counts) = separated_list0(tag(", "), parse_color_count)(input)?;

    let set = color_counts
        .into_iter()
        .fold(CubeSet::default(), |mut set, (count, color)| {
            match color {
                CubeColor::Red => set.red += count,
                CubeColor::Green => set.green += count,
                CubeColor::Blue => set.blue += count,
            }
            set
        });
    Ok((rest, set))
}

fn parse_game(input: &str) -> Result<Game, nom::Err<nom::error::Error<&str>>> {
    let mut parse_cube_sets = separated_list0(tag("; "), parse_cube_set);
    let mut parse_game_id = terminated(preceded(tag("Game "), number_parser), tag(": "));
    let (rest, id) = parse_game_id(input)?;
    let (_, sets) = parse_cube_sets(rest)?;
    Ok(Game { id, sets })
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    Some(
        input
            .lines()
            .map(parse_game)
            .map(Result::unwrap)
            .map(|game| {
                if game.sets.into_iter().all(|set| set <= max_set) {
                    game.id
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_game)
            .map(Result::unwrap)
            .map(|game| {
                game.sets
                    .into_iter()
                    .fold(CubeSet::default(), |acc, set| acc.maximize(&set))
                    .power()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

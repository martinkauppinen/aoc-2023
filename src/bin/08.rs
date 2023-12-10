use itertools::{FoldWhile, Itertools};
use nohash_hasher::IntSet;
use nom::{
    bytes::complete::tag, character::complete::alpha1, sequence::separated_pair, IResult, Parser,
};
use nom_supreme::ParserExt;

advent_of_code::solution!(8);

type AdjacencyList = [[Option<usize>; 2]; 26 * 26 * 26];

fn parse_letters(input: &str) -> IResult<&str, usize> {
    alpha1
        .map(|letters: &str| {
            letters
                .chars()
                .rev()
                .enumerate()
                .fold(0, |acc, (power, c)| {
                    acc + (c as usize - 'A' as usize) * 26usize.pow(power as u32)
                })
        })
        .parse(input)
}

fn parse_adjacency(input: &str) -> AdjacencyList {
    let mut adjacency = [[None; 2]; 26 * 26 * 26];

    input.lines().for_each(|line| {
        let (source, (left, right)) = parse_letters
            .and(separated_pair(parse_letters, tag(", "), parse_letters).preceded_by(tag(" = (")))
            .parse(line)
            .unwrap()
            .1;
        adjacency[source][0] = Some(left);
        adjacency[source][1] = Some(right);
    });

    adjacency
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut chunks = input.split("\n\n");
    let directions = chunks
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .cycle();
    let adj = parse_adjacency(chunks.next().unwrap());
    let steps = directions
        .enumerate()
        .fold_while(0, |source, (i, direction)| {
            let destination = adj[source][direction].unwrap();
            if destination == adj.len() - 1 {
                FoldWhile::Done(i)
            } else {
                FoldWhile::Continue(destination)
            }
        })
        .into_inner();
    Some(steps as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut chunks = input.split("\n\n");
    let directions = chunks
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .cycle();
    let adj = parse_adjacency(chunks.next().unwrap());

    let ghost_paths = adj
        .iter()
        .enumerate()
        .step_by(26)
        .filter(|(_, dir)| dir[0].is_some() && dir[1].is_some())
        .map(|(start, _)| start)
        .collect_vec();

    let steps = ghost_paths
        .iter()
        .map(|start| {
            let mut seen_zs: IntSet<usize> = IntSet::default();
            directions
                .clone()
                .enumerate()
                .fold_while(
                    (*start, vec![]),
                    |(source, mut step_vec), (i, direction)| {
                        let destination = adj[source][direction].unwrap();
                        if destination % 26 == 25 {
                            if seen_zs.contains(&destination) {
                                FoldWhile::Done((destination, step_vec))
                            } else {
                                seen_zs.insert(destination);
                                step_vec.push(i + 1);
                                FoldWhile::Continue((destination, step_vec))
                            }
                        } else {
                            FoldWhile::Continue((destination, step_vec))
                        }
                    },
                )
                .into_inner()
                .1
        })
        .collect_vec();

    let steps = steps.concat();
    let answer = steps
        .iter()
        .skip(1)
        .fold(steps[0], |acc, step| num::integer::lcm(acc, *step));
    Some(answer as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example("examples", DAY, 1));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_example("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}

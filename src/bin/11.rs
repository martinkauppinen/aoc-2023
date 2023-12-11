use itertools::Itertools;

advent_of_code::solution!(11);

type Output = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: Output,
    col: Output,
}

impl Coordinate {
    fn manhattan_distance(&self, other: &Coordinate) -> Output {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn parse_input(input: &str, multiplier: Output) -> Vec<Coordinate> {
    let mut empty_rows = Vec::new();
    let mut col_count = vec![0; input.lines().next().unwrap().len()];
    let mut galaxies = Vec::new();
    input.lines().enumerate().for_each(|(row, line)| {
        let mut row_empty = true;
        line.char_indices().for_each(|(col, c)| {
            if c == '#' {
                col_count[col] += 1;
                row_empty = false;
                galaxies.push(Coordinate {
                    row: row as Output,
                    col: col as Output,
                });
            }
        });
        if row_empty {
            empty_rows.push(row as Output);
        }
    });

    for galaxy in galaxies.iter_mut() {
        galaxy.row += (multiplier - 1).max(1)
            * empty_rows.iter().filter(|row| **row < galaxy.row).count() as Output;
        galaxy.col += (multiplier - 1).max(1)
            * col_count[0..galaxy.col as usize]
                .iter()
                .filter(|count| **count == 0)
                .count() as Output;
    }

    galaxies
}

fn solve(input: &str, multiplier: Output) -> Output {
    let galaxies = parse_input(input, multiplier);
    galaxies
        .iter()
        .tuple_combinations()
        .fold(0, |acc, (a, b)| acc + a.manhattan_distance(b))
}

pub fn part_one(input: &str) -> Option<Output> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<Output> {
    Some(solve(input, 1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        assert_eq!(solve(input, 10), 1030);
        assert_eq!(solve(input, 100), 8410);
    }
}

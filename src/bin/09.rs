use itertools::Itertools;

advent_of_code::solution!(9);

type Output = i32;

fn predict_value(values: Vec<Output>) -> Output {
    if values.iter().all(|x| *x == 0) {
        return 0;
    }

    let lower_sequence = values
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    let last = values.last().unwrap();
    predict_value(lower_sequence) + last
}

pub fn part_one(input: &str) -> Option<Output> {
    Some(
        input
            .lines()
            .map(str::split_ascii_whitespace)
            .map(|str_nums| str_nums.map(str::parse).map(Result::unwrap).collect_vec())
            .map(predict_value)
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<Output> {
    Some(
        input
            .lines()
            .map(str::split_ascii_whitespace)
            .map(|str_nums| {
                str_nums
                    .rev()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect_vec()
            })
            .map(predict_value)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(str::as_bytes)
            .map(|line| {
                let a = line.iter().find(|b| b.is_ascii_digit()).unwrap() & !b'0'; // ASCII shenanigans
                let b = line.iter().rfind(|b| b.is_ascii_digit()).unwrap() & !b'0';
                let number = 10 * a + b;
                number as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .replace("one", "o1e") // Keeping overlaps
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4") // No overlap, can replace entire thing
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9");
    part_one(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}

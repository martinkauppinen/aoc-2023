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

fn digit_string_to_digit(digit_string: &[u8]) -> u32 {
    match digit_string {
        b"1" | b"on" => 1,
        b"2" | b"tw" => 2,
        b"3" | b"thre" => 3,
        b"4" | b"four" => 4,
        b"5" | b"five" => 5,
        b"6" | b"six" => 6,
        b"7" | b"seve" => 7,
        b"8" | b"eigh" => 8,
        b"9" | b"nine" => 9,
        _ => panic!("Not a digit string: {:?}", digit_string),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // Overlapping matches, i.e. oneeight/sevenine/threeight/eightwo/twoone need lookahead
    let re = pcre2::bytes::Regex::new(
        "(on(?=e)|tw(?=o)|thre(?=e)|four|five|six|seve(?=n)|eigh(?=t)|nine|[0-9])",
    )
    .unwrap();
    Some(
        input
            .lines()
            .map(|line| {
                let mut matches = re.find_iter(line.as_bytes());
                let a = matches.next().unwrap();
                let b = matches.last().unwrap_or(a.clone());

                let a = digit_string_to_digit(a.unwrap().as_bytes());
                let b = digit_string_to_digit(b.unwrap().as_bytes());
                10 * a + b
            })
            .sum(),
    )
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

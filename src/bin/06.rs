advent_of_code::solution!(6);

// T = t_hold + t_race
// t = t_hold
// D = record distance
// v = t
// d(t) = v * t_race
//           = t * t_race
//           = t * (T - t)
//           = Tt - t^2
// Want to beat previous record, meaning that d(t) - D > 0
//      => d(t) = Tt - t^2 - D
//      => Find roots, round smaller root up, larger root down to satisfy inequality
// Quadratic: a(t^2) + bt + c = 0
//       => a = -1
//       => b = T
//       => c = -D

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = b * b - 4.0 * a * c;
    let t1 = (-b - d.sqrt()) / (2.0 * a);
    let t2 = (-b + d.sqrt()) / (2.0 * a);
    (t1.min(t2), t2.max(t1))
}

fn solve(times: impl Iterator<Item = u64>, distances: impl Iterator<Item = u64>) -> u64 {
    times
        .zip(distances)
        .map(|(time, distance)| {
            let (t1, t2) = quadratic_formula(-1.0, time as f64, -(distance as f64));
            // Correction term. When the root is an integer, the
            // previous record will be tied, not beaten, so you'd need to hold
            // the button 1 millisecond more/less, which decreases
            // the number of ways to beat the previous record.
            let mut correction_term = 0;
            if t1.fract() == 0.0 {
                correction_term += 1;
            }
            if t2.fract() == 0.0 {
                correction_term += 1;
            }

            let t1 = t1.ceil() as u64;
            let t2 = t2.floor() as u64;
            t2 - t1 + 1 - correction_term
        })
        .product()
}

pub fn part_one(input: &str) -> Option<u64> {
    let times = input
        .lines()
        .take(1)
        .map(str::split_whitespace)
        .map(|iter| iter.skip(1))
        .map(|iter| iter.map(str::parse::<u64>))
        .flat_map(|iter| iter.map(Result::unwrap));

    let distances = input
        .lines()
        .skip(1)
        .take(1)
        .map(str::split_whitespace)
        .map(|iter| iter.skip(1))
        .map(|iter| iter.map(str::parse::<u64>))
        .flat_map(|iter| iter.map(Result::unwrap));

    Some(solve(times, distances))
}

pub fn part_two(input: &str) -> Option<u64> {
    let times = input
        .lines()
        .take(1)
        .flat_map(|line| line.split(':').skip(1).take(1))
        .map(|line| line.replace(' ', ""))
        .map(|number| number.parse())
        .map(Result::unwrap);

    let distances = input
        .lines()
        .skip(1)
        .take(1)
        .flat_map(|line| line.split(':').skip(1).take(1))
        .map(|line| line.replace(' ', ""))
        .map(|number| number.parse())
        .map(Result::unwrap);

    Some(solve(times, distances))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

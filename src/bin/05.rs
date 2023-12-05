use advent_of_code::parsers::space_separated_numbers_parser;
use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn new(range: (u64, u64)) -> Self {
        Self {
            start: range.0,
            length: range.1,
        }
    }

    /// Inclusive. The last number in the range
    fn last(&self) -> u64 {
        self.start + self.length - 1
    }

    /// Exclusive. The first number not in the range
    fn end(&self) -> u64 {
        self.start + self.length
    }

    fn contains(&self, location: &u64) -> bool {
        *location >= self.start && *location < self.end()
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.contains(&other.start) || self.contains(&other.last())
    }

    /// Range of values in both ranges
    fn intersection(&self, other: &Range) -> Option<Range> {
        if self.overlaps(other) {
            let start = self.start.max(other.start);
            let len = self.end().min(other.end()) - start;
            Some(Range::new((start, len)))
        } else {
            None
        }
    }

    /// Get sub-ranges of `other` that are outside this range on either side
    fn relative_complement(&self, other: &Range) -> (Option<Self>, Option<Self>) {
        let left = if other.start >= self.start {
            None
        } else {
            let end = self.start.min(other.end());
            let length = end - other.start;
            Some(Range::new((other.start, length)))
        };

        let right = if other.end() <= self.end() {
            None
        } else {
            let start = self.end().max(other.start);
            let length = other.length - (self.end().saturating_sub(other.start));
            Some(Range::new((start, length)))
        };

        (left, right)
    }
}

#[derive(Debug, Default)]
struct MapRange {
    source_range: Range,
    destination_range: Range,
}

#[derive(Debug, Default)]
struct AlmanacMap {
    ranges: Vec<MapRange>,
}

impl AlmanacMap {
    fn new() -> Self {
        Self::default()
    }

    /// Insert a map range. It's important that the ranges are sorted by source start location,
    /// as this is assumed in `get` while scanning through the list.
    fn insert(&mut self, destination: u64, source: u64, length: u64) {
        match self
            .ranges
            .binary_search_by(|range| range.source_range.start.cmp(&source))
        {
            Ok(_) => {}
            Err(i) => {
                self.ranges.insert(
                    i,
                    MapRange {
                        source_range: Range {
                            start: source,
                            length,
                        },
                        destination_range: Range {
                            start: destination,
                            length,
                        },
                    },
                );
            }
        }
    }

    /// Get the destination ranges the given range maps to
    fn get(&self, source_range: &Range) -> Vec<Range> {
        let mut ranges = Vec::new();
        let mut ranges_to_check = vec![*source_range];
        for range in &self.ranges {
            let left_to_check = ranges_to_check.len();
            for _ in 0..left_to_check {
                let checked_range = ranges_to_check.pop().unwrap();

                if let Some(mut intersection) = range.source_range.intersection(&checked_range) {
                    let offset = intersection.start - range.source_range.start;
                    intersection.start = range.destination_range.start + offset;
                    ranges.push(intersection); // the intersection is completely mapped
                }

                let (left, right) = range.source_range.relative_complement(&checked_range);
                if let Some(left) = left {
                    ranges_to_check.push(left); // the sub-range to the left is not mapped. Check it next iteration of outer loop
                }

                if let Some(right) = right {
                    ranges_to_check.push(right); // the sub-range to the right is not mapped. Check it next iteration of outer loop
                }
            }
        }
        ranges.extend(ranges_to_check); // add the remaining ranges with identity mapping
        ranges
    }
}

fn parse_map(input: &str) -> AlmanacMap {
    input
        .lines()
        .skip(1)
        .map(space_separated_numbers_parser::<u64>)
        .map(Result::unwrap)
        .fold(AlmanacMap::new(), |mut map, (_, numbers)| {
            let dest_start = numbers[0];
            let src_start = numbers[1];
            let length = numbers[2];
            map.insert(dest_start, src_start, length);
            map
        })
}

fn solve(seed_ranges: impl Iterator<Item = Range>, maps: Vec<AlmanacMap>) -> u64 {
    let mut lowest_location = u64::MAX;
    for range in seed_ranges {
        let location = maps
            .iter()
            .fold(vec![range], |ranges, map| {
                let mut new_ranges = Vec::new();
                for range in ranges {
                    new_ranges.extend(map.get(&range));
                }
                new_ranges
            })
            .iter()
            .fold(u64::MAX, |acc, range| acc.min(range.start));

        lowest_location = lowest_location.min(location);
    }

    lowest_location
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut chunks = input.split("\n\n");
    let seeds_line = chunks.next().unwrap();
    let seed_ranges = seeds_line
        .split(": ")
        .skip(1)
        .map(space_separated_numbers_parser::<u64>)
        .map(Result::unwrap)
        .flat_map(|(_, numbers)| numbers)
        .map(|x| Range::new((x, 1)));

    let maps = chunks.map(parse_map).collect();

    let lowest_location = solve(seed_ranges, maps);
    Some(lowest_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut chunks = input.split("\n\n");
    let seeds_line = chunks.next().unwrap();
    let seed_ranges = seeds_line
        .split(": ")
        .skip(1)
        .map(space_separated_numbers_parser::<u64>)
        .map(Result::unwrap)
        .flat_map(|(_, numbers)| numbers)
        .tuples::<(u64, u64)>()
        .map(Range::new);

    let maps = chunks.map(parse_map).collect();

    let lowest_location = solve(seed_ranges, maps);
    Some(lowest_location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_contains() {
        let range1 = Range::new((5, 15)); // 5..=19
        for i in 0..=20 {
            if i >= 5 && i <= 19 {
                assert!(range1.contains(&i));
            } else {
                assert!(!range1.contains(&i));
            }
        }
    }

    #[test]
    fn test_overlaps() {
        let range1 = Range::new((5, 15)); // 5..=19
        let range2 = Range::new((0, 20)); // 0..=19
        assert!(range1.overlaps(&range2));

        let range1 = Range::new((5, 15)); // 5..=19
        let range2 = Range::new((20, 20)); // 20..=49
        assert!(!range1.overlaps(&range2));
    }

    #[test]
    fn test_intersection() {
        let range1 = Range::new((0, 10)); // 0..=9
        let range2 = Range::new((5, 15)); // 5..=19
        let intersection = range1.intersection(&range2);
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection.start, 5);
        assert_eq!(intersection.length, 5);
        assert_eq!(intersection.last(), 9);
        assert_eq!(intersection.end(), 10);

        // symmetry
        let intersection = range2.intersection(&range1);
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection.start, 5);
        assert_eq!(intersection.length, 5);
        assert_eq!(intersection.last(), 9);
        assert_eq!(intersection.end(), 10);
    }

    #[test]
    fn test_relative_complement() {
        let range1 = Range::new((5, 10)); // 5..=14
        let range2 = Range::new((0, 20)); // 0..=19
        let (left, right) = range1.relative_complement(&range2);
        assert!(left.is_some());
        assert!(right.is_some());
        let left = left.unwrap();
        let right = right.unwrap();
        assert_eq!(left.start, 0);
        assert_eq!(left.length, 5);
        assert_eq!(left.last(), 4);
        assert_eq!(left.end(), 5);
        assert_eq!(right.start, 15);
        assert_eq!(right.length, 5);
        assert_eq!(right.last(), 19);
        assert_eq!(right.end(), 20);
    }

    #[test]
    fn step_example_two() {
        // Yes, I checked all these manually. It was a pain.

        let seeds = [Range::new((79, 14)), Range::new((55, 13))];

        // seed to soil
        let mut seed_to_soil = AlmanacMap::new();
        seed_to_soil.insert(50, 98, 2);
        seed_to_soil.insert(52, 50, 48);

        let soils = seeds
            .iter()
            .flat_map(|seed| seed_to_soil.get(&seed))
            .collect::<Vec<_>>();
        assert_eq!(soils[0], Range::new((81, 14)));
        assert_eq!(soils[1], Range::new((57, 13)));

        // soil to fertilizer
        let mut soil_to_fertilizer = AlmanacMap::new();
        soil_to_fertilizer.insert(0, 15, 37);
        soil_to_fertilizer.insert(37, 52, 2);
        soil_to_fertilizer.insert(39, 0, 15);

        let fertilizers = soils
            .iter()
            .flat_map(|soil| soil_to_fertilizer.get(&soil))
            .collect::<Vec<_>>();
        assert_eq!(fertilizers[0], soils[0]);
        assert_eq!(fertilizers[1], soils[1]);

        // fertilizer to water
        let mut fertilizer_to_water = AlmanacMap::new();
        fertilizer_to_water.insert(49, 53, 8);
        fertilizer_to_water.insert(0, 11, 42);
        fertilizer_to_water.insert(42, 0, 7);
        fertilizer_to_water.insert(57, 7, 4);

        let waters = fertilizers
            .iter()
            .flat_map(|fertilizer| fertilizer_to_water.get(&fertilizer))
            .collect::<Vec<_>>();
        assert_eq!(waters[0], Range::new((81, 14)));
        assert_eq!(waters[1], Range::new((53, 4)));
        assert_eq!(waters[2], Range::new((61, 9)));
        assert_eq!(waters.len(), 3);

        // water to light
        let mut water_to_light = AlmanacMap::new();
        water_to_light.insert(88, 18, 7);
        water_to_light.insert(18, 25, 70);

        let lights = waters
            .iter()
            .flat_map(|water| water_to_light.get(&water))
            .collect::<Vec<_>>();
        assert_eq!(lights[0], Range::new((74, 14)));
        assert_eq!(lights[1], Range::new((46, 4)));
        assert_eq!(lights[2], Range::new((54, 9)));
        assert_eq!(lights.len(), 3);

        // light to temperature
        let mut light_to_temperature = AlmanacMap::new();
        light_to_temperature.insert(45, 77, 23);
        light_to_temperature.insert(81, 45, 19);
        light_to_temperature.insert(68, 64, 13);

        let temperatures = lights
            .iter()
            .flat_map(|light| light_to_temperature.get(&light))
            .collect::<Vec<_>>();
        assert_eq!(temperatures[0], Range::new((78, 3)));
        assert_eq!(temperatures[1], Range::new((45, 11)));
        assert_eq!(temperatures[2], Range::new((82, 4)));
        assert_eq!(temperatures[3], Range::new((90, 9)));
        assert_eq!(temperatures.len(), 4);

        // temperature to humidity
        let mut temperature_to_humidity = AlmanacMap::new();
        temperature_to_humidity.insert(0, 69, 1);
        temperature_to_humidity.insert(1, 0, 69);

        let humidities = temperatures
            .iter()
            .flat_map(|temperature| temperature_to_humidity.get(&temperature))
            .collect::<Vec<_>>();
        assert_eq!(humidities[0], Range::new((78, 3)));
        assert_eq!(humidities[1], Range::new((46, 11)));
        assert_eq!(humidities[2], Range::new((82, 4)));
        assert_eq!(humidities[3], Range::new((90, 9)));
        assert_eq!(humidities.len(), 4);

        // humidity to location
        let mut humidity_to_location = AlmanacMap::new();
        humidity_to_location.insert(60, 56, 37);
        humidity_to_location.insert(56, 93, 4);

        let locations = humidities
            .iter()
            .flat_map(|humidity| humidity_to_location.get(&humidity))
            .collect::<Vec<_>>();
        assert_eq!(locations[0], Range::new((82, 3)));
        assert_eq!(locations[1], Range::new((60, 1)));
        assert_eq!(locations[2], Range::new((46, 10)));
        assert_eq!(locations[3], Range::new((86, 4)));
        assert_eq!(locations[4], Range::new((94, 3)));
        assert_eq!(locations[5], Range::new((56, 4)));
        assert_eq!(locations[6], Range::new((97, 2)));
        assert_eq!(locations.len(), 7);
    }
}

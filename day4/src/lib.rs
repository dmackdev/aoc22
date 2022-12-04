use lazy_static::lazy_static;
use regex::Regex;

struct Range {
    lower_bound: u32,
    upper_bound: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.lower_bound <= other.lower_bound && self.upper_bound >= other.upper_bound
    }

    fn overlaps(&self, other: &Range) -> bool {
        let self_range = self.lower_bound..=self.upper_bound;
        let other_range = other.lower_bound..=other.upper_bound;
        self_range.contains(&other.lower_bound)
            || self_range.contains(&other.upper_bound)
            || other_range.contains(&self.lower_bound)
            || other_range.contains(&self.upper_bound)
    }
}

pub fn count_fully_contained(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (range_1, range_2) = parse_ranges_from_line(line);

            let is_one_pair_fully_contained =
                range_1.contains(&range_2) || range_2.contains(&range_1);

            is_one_pair_fully_contained.then_some((range_1, range_2))
        })
        .count()
}

pub fn count_overlaps(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (range_1, range_2) = parse_ranges_from_line(line);

            range_1.overlaps(&range_2).then_some((range_1, range_2))
        })
        .count()
}

fn parse_ranges_from_line(line: &str) -> (Range, Range) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    }

    let caps = RE.captures(line).unwrap();

    let range_1 = Range {
        lower_bound: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        upper_bound: caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
    };

    let range_2 = Range {
        lower_bound: caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        upper_bound: caps.get(4).unwrap().as_str().parse::<u32>().unwrap(),
    };

    (range_1, range_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn count_fully_contained_simple_case() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(count_fully_contained(input), 2);
    }

    #[test]
    fn count_overlaps_simple_case() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(count_overlaps(input), 4);
    }

    #[rstest]
    #[case("2-4,6-8", false)]
    #[case("2-3,4-5", false)]
    #[case("5-7,7-9", true)]
    #[case("2-8,3-7", true)]
    #[case("6-6,4-6", true)]
    #[case("2-6,4-8", true)]
    fn range_overlaps_case(#[case] line: &str, #[case] expected: bool) {
        let (range_1, range_2) = parse_ranges_from_line(line);

        assert_eq!(range_1.overlaps(&range_2), expected);
    }
}

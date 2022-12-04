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
}

pub fn parse_input(input: &str) -> usize {
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

    #[test]
    fn parse_input_simple_case() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(parse_input(input), 2);
    }
}

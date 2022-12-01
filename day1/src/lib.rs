use std::cmp::Reverse;

pub fn parse_input(input: &str) -> Vec<u64> {
    let mut totals = Vec::new();
    let mut curr_sum = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            totals.push(curr_sum);
            curr_sum = 0;
        } else {
            curr_sum += line.parse::<u64>().unwrap();
        }
    }

    totals.push(curr_sum);
    totals.sort_by_key(|w| Reverse(*w));

    totals
}

pub fn get_max_three_total(input: Vec<u64>) -> u64 {
    input.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parse_input_simple_case() {
        let result = parse_input(SAMPLE_INPUT);
        assert_eq!(result, vec![24000, 11000, 10000, 6000, 4000]);
    }

    #[test]
    fn get_max_three_total_simple_case() {
        let totals = parse_input(SAMPLE_INPUT);
        let result = get_max_three_total(totals);
        assert_eq!(result, 45000);
    }
}

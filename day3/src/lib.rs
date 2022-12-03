pub fn calculate_priority_sum_of_duplicate_items(input: &str) -> u32 {
    input
        .lines()
        .map(calculate_priority_of_duplicate_item)
        .sum()
}

fn calculate_priority_of_duplicate_item(rucksack: &str) -> u32 {
    assert!(rucksack.len() % 2 == 0);

    let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);

    for c in first_half.chars() {
        if second_half.contains(c) {
            return get_item_priority(c);
        }
    }

    panic!("Could not find common item in each part of rucksack {rucksack}");
}

fn get_item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid item: {item}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('p', 16)]
    #[case('L', 38)]
    #[case('P', 42)]
    #[case('v', 22)]
    #[case('t', 20)]
    #[case('s', 19)]
    fn get_item_priority_case(#[case] item: char, #[case] expected: u32) {
        assert_eq!(expected, get_item_priority(item));
    }

    #[rstest]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 16)]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38)]
    #[case("PmmdzqPrVvPwwTWBwg", 42)]
    #[case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 22)]
    #[case("ttgJtRGJQctTZtZT", 20)]
    #[case("CrZsJsPPZsGzwwsLwLmpwMDw", 19)]
    fn calculate_priority_of_rucksack_case(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, calculate_priority_of_duplicate_item(input))
    }

    #[test]
    fn calculate_priority_sum_of_duplicate_items_simple_case() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, calculate_priority_sum_of_duplicate_items(input));
    }
}

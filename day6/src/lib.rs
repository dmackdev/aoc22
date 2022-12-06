use std::collections::HashSet;

const WINDOW_SIZE: usize = 4;

pub fn find_marker_index(input: &str) -> usize {
    for (idx, group) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(WINDOW_SIZE)
        .enumerate()
    {
        let mut set: HashSet<char> = HashSet::new();
        for c in group.iter() {
            set.insert(*c);
        }

        if set.len() == WINDOW_SIZE {
            return idx + WINDOW_SIZE;
        }
    }
    panic!("Could not find marker index.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn find_marker_index_case(#[case] line: &str, #[case] expected: usize) {
        let idx = find_marker_index(line);
        assert_eq!(idx, expected);
    }
}

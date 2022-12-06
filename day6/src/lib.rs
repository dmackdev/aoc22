use std::collections::HashSet;

pub fn find_marker_index(input: &str, window_size: usize) -> usize {
    for (idx, group) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        let mut set: HashSet<char> = HashSet::new();
        for c in group.iter() {
            set.insert(*c);
        }

        if set.len() == window_size {
            return idx + window_size;
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
    fn find_packet_marker_index_window_size_4_case(#[case] line: &str, #[case] expected: usize) {
        let idx = find_marker_index(line, 4);
        assert_eq!(idx, expected);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn find_packet_marker_index_window_size_14_case(#[case] line: &str, #[case] expected: usize) {
        let idx = find_marker_index(line, 14);
        assert_eq!(idx, expected);
    }
}

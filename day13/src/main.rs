use std::fs;

use day13::{get_decoder_key, parse_packets, sum_indices_of_ordered_packet_pairs};

fn main() {
    let packets = parse_packets(&fs::read_to_string("input.txt").unwrap());
    let indices_sum = sum_indices_of_ordered_packet_pairs(&packets);

    println!(
        "The sum of the indices of the packets pairs in the correct order is: {}",
        indices_sum
    );

    let divider_packets = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];

    let decoder_key = get_decoder_key(packets, divider_packets);

    println!("The decoder key is: {}", decoder_key);
}

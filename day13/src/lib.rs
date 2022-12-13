use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
enum PacketData {
    List(Vec<PacketData>),
    Val(i8),
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    data: Vec<PacketData>,
}

pub fn parse_packets(input: &str) -> Vec<Packet> {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(|chunk| {
            let packet_1 = chunk[0].parse::<Packet>().unwrap();
            let packet_2 = chunk[1].parse::<Packet>().unwrap();
            [packet_1, packet_2]
        })
        .collect()
}

pub fn sum_indices_of_ordered_packet_pairs(packets: &[Packet]) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter_map(|(idx, chunk)| {
            let packet_1 = &chunk[0];
            let packet_2 = &chunk[1];

            (packet_1 < packet_2).then_some(idx + 1)
        })
        .sum()
}

pub fn get_decoder_key(mut packets: Vec<Packet>, divider_packets: Vec<Packet>) -> usize {
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    divider_packets
        .into_iter()
        .map(|divider_p| insert_packet_sorted(&mut packets, divider_p) + 1)
        .product()
}

fn insert_packet_sorted(sorted_packets: &mut Vec<Packet>, packet: Packet) -> usize {
    let insertion_idx = sorted_packets
        .iter()
        .position(|other| matches!(packet.partial_cmp(other), Some(Ordering::Less)))
        .unwrap();

    sorted_packets.insert(insertion_idx, packet);

    insertion_idx
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = split_tokens(s);

        let mut packet = Packet { data: vec![] };

        let mut lists: Vec<PacketData> = vec![];

        for token in tokens {
            match token.as_str() {
                "[" => lists.push(PacketData::List(vec![])),
                "]" => {
                    if let Some(curent_list) = lists.pop() {
                        if let Some(PacketData::List(parent_list)) = lists.last_mut() {
                            parent_list.push(curent_list)
                        } else {
                            packet.data.push(curent_list)
                        }
                    }
                }
                "" => (),
                num => {
                    let val = PacketData::Val(num.parse()?);
                    if lists.is_empty() {
                        packet.data.push(val)
                    } else if let Some(PacketData::List(current_list)) = lists.last_mut() {
                        current_list.push(val)
                    }
                }
            }
        }

        Ok(packet)
    }
}

pub fn split_tokens(line: &str) -> Vec<String> {
    line[1..line.len() - 1]
        .replace('[', "[,")
        .replace(']', ",]")
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PacketData::List(self.data.clone()).partial_cmp(&PacketData::List(other.data.clone()))
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PacketData::List(list_a), PacketData::List(list_b)) => {
                if list_a.is_empty() && !list_b.is_empty() {
                    return Some(Ordering::Less);
                }

                if !list_a.is_empty() && list_b.is_empty() {
                    return Some(Ordering::Greater);
                }

                if list_a.is_empty() && list_b.is_empty() {
                    return Some(Ordering::Equal);
                }

                match list_a[0].partial_cmp(&list_b[0]) {
                    Some(Ordering::Equal) => PacketData::List(list_a[1..].to_vec())
                        .partial_cmp(&PacketData::List(list_b[1..].to_vec())),
                    ordering => ordering,
                }
            }
            (PacketData::List(_), PacketData::Val(_)) => {
                self.partial_cmp(&PacketData::List(vec![other.clone()]))
            }
            (PacketData::Val(_), PacketData::List(_)) => {
                PacketData::List(vec![self.clone()]).partial_cmp(other)
            }
            (PacketData::Val(a), PacketData::Val(b)) => Some(a.cmp(b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn nested_empty_lists_example() {
        let packet = "[[[]]]".parse::<Packet>().unwrap();

        assert_eq!(
            packet.data,
            vec![PacketData::List(vec![PacketData::List(vec![])])]
        )
    }

    #[test]
    fn nested_lists_example() {
        let packet = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Packet>().unwrap();

        assert_eq!(
            packet.data,
            vec![
                PacketData::Val(1),
                PacketData::List(vec![
                    PacketData::Val(2),
                    PacketData::List(vec![
                        PacketData::Val(3),
                        PacketData::List(vec![
                            PacketData::Val(4),
                            PacketData::List(vec![
                                PacketData::Val(5),
                                PacketData::Val(6),
                                PacketData::Val(7)
                            ])
                        ])
                    ])
                ]),
                PacketData::Val(8),
                PacketData::Val(9)
            ]
        )
    }

    #[test]
    fn example() {
        let packets = parse_packets(&fs::read_to_string("test_input.txt").unwrap());
        let result = sum_indices_of_ordered_packet_pairs(&packets);

        assert_eq!(result, 13);

        let divider_packets = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];

        let decoder_key = get_decoder_key(packets, divider_packets);

        assert_eq!(decoder_key, 140);
    }
}

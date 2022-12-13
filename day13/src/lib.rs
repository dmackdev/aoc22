use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd)]
enum PacketData {
    List(Vec<PacketData>),
    Val(i8),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Packet {
    data: Vec<PacketData>,
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|chunk| {
            let (packet_1, packet_2) = normalise(
                chunk[0].parse::<Packet>().unwrap(),
                chunk[1].parse::<Packet>().unwrap(),
            );

            (packet_1 < packet_2).then_some((packet_1, packet_2))
        })
        .collect()
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = split_tokens(s);

        let mut packet = Packet { data: vec![] };

        let mut lists: Vec<PacketData> = vec![];

        for token in tokens {
            match token.as_str() {
                "[" => {
                    let new_list = PacketData::List(vec![]);
                    lists.push(new_list)
                }
                "]" => {
                    if let Some(n) = lists.pop() {
                        if let Some(PacketData::List(nested_list)) = lists.last_mut() {
                            nested_list.push(n)
                        } else {
                            packet.data.push(n)
                        }
                    }
                }
                "" => (),
                num => {
                    let val = PacketData::Val(num.parse().unwrap());
                    if lists.is_empty() {
                        packet.data.push(val)
                    } else if let Some(PacketData::List(nested_list)) = lists.last_mut() {
                        nested_list.push(val)
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
        .collect::<Vec<_>>()
}

// impl PartialOrd for PacketData {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         todo!()
//     }
// }

fn normalise(a: Packet, b: Packet) -> (Packet, Packet) {
    todo!()
}

#[cfg(test)]
mod tests {
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
}

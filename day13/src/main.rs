use day13::{split_tokens, Packet};

fn main() {
    let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
    println!("{:?}", input);
    println!("{:?}", split_tokens(input));
    println!("{:?}", input.parse::<Packet>());
}


use std::{
    cmp::{
        self,
        Ordering,
    },
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self,
        newline,
    },
    combinator,
    multi::separated_list0,
    sequence::{
        pair,
        separated_pair,
    },
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}
macro_rules! l {
    () => {
        Packet::List(vec![])
    };
    ($($val:expr),*) => {
        Packet::List(vec![$($val),*])
    }
}
macro_rules! i {
    ($val:expr) => {
        Packet::Int($val)
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(&b),
            (Packet::Int(a), b) => {
                Packet::List(vec![Packet::Int(*a)]).cmp(b)
            },
            (a, Packet::Int(b)) => {
                a.cmp(&Packet::List(vec![Packet::Int(*b)]))
            },
            (Packet::List(a), Packet::List(b)) => {
                for i in 0..cmp::min(a.len(), b.len()) {
                    match a[i].cmp(&b[i]) {
                        Ordering::Equal => { },
                        o => { return o; },
                    }
                }
                a.len().cmp(&b.len())
            },
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    let (s, _) = tag("[")(s)?;
    let (s, pack) = separated_list0(tag(","), alt((
            combinator::map(complete::i32, |i| Packet::Int(i)),
            parse_packet
    )))(s)?;
    let (s, _) = tag("]")(s)?;
    Ok((s, Packet::List(pack)))
}
fn parse_packet_pair(s: &str) -> IResult<&str, (Packet, Packet)> {
        separated_pair(parse_packet, newline, parse_packet)(s)
}
fn parse_packet_pair_list(s: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list0(pair(newline, newline), parse_packet_pair)(s)
}

fn part1(packets: &Vec<(Packet, Packet)>) -> usize {
    packets.iter().enumerate().fold(0, |sum, (i, (a, b))| {
        if a < b {
            sum + i + 1
        } else {
            sum
        }
    })
}

fn part2(packets: Vec<(Packet, Packet)>) -> usize {
    let decode_1: Packet = l![l![i!(2)]];
    let decode_2: Packet = l![l![i!(6)]];
    let mut packets = packets.into_iter().fold(vec![decode_1.clone(), decode_2.clone()],
    |mut vec, (pack_a, pack_b)| {
        vec.push(pack_a);
        vec.push(pack_b);
        vec
    });
    packets.sort();
    packets.iter().enumerate().fold(1, |key, (idx, pack)| {
        if pack == &decode_1 || pack == &decode_2 {
            key * (idx + 1)
        } else {
            key
        }
    })
}

fn main() {
    let input = include_str!("input.txt");
    let (_, packet_list) = parse_packet_pair_list(input).unwrap();
    println!("Part 1: {}", part1(&packet_list));
    println!("Part 2: {}", part2(packet_list));
}

#[cfg(test)]
mod tests_day_13 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");
    fn parsed_input() -> Vec<(Packet, Packet)> {
        vec![(
            l![i!(1),i!(1),i!(3),i!(1),i!(1)],
            l![i!(1),i!(1),i!(5),i!(1),i!(1)]
        ), (
            l![l![i!(1)],l![i!(2),i!(3),i!(4)]],
            l![l![i!(1)],i!(4)]
        ), (
            l![i!(9)],
            l![l![i!(8),i!(7),i!(6)]]
        ), (
            l![l![i!(4),i!(4)],i!(4),i!(4)],
            l![l![i!(4),i!(4)],i!(4),i!(4),i!(4)]
        ), (
            l![i!(7),i!(7),i!(7),i!(7)],
            l![i!(7),i!(7),i!(7)]
        ), (
            l![],
            l![i!(3)]
        ), (
            l![l![l![]]],
            l![l![]]
        ), (
            l![i!(1),l![i!(2),l![i!(3),l![i!(4),l![i!(5),i!(6),i!(7)]]]],i!(8),i!(9)],
            l![i!(1),l![i!(2),l![i!(3),l![i!(4),l![i!(5),i!(6),i!(0)]]]],i!(8),i!(9)]
        )]
    }

    #[test]
    fn full_example_part2() {
        let (_, data) = parse_packet_pair_list(INPUT).unwrap();
        assert_eq!(part2(data), 140);
    }
    #[test]
    fn example_part2() {
        assert_eq!(part2(parsed_input()), 140);
    }
    #[test]
    fn full_example_part1() {
        let (_, data) = parse_packet_pair_list(INPUT).unwrap();
        assert_eq!(part1(&data), 13);
    }
    #[test]
    fn example_part1() {
        assert_eq!(part1(&parsed_input()), 13);
    }
    #[test]
    fn parse_full_input() {
        let (_, data) = parse_packet_pair_list(INPUT).unwrap();
        assert_eq!(data, parsed_input());
    }
    #[test]
    fn parse_input_packet_pair() {
        assert_eq!(
            parse_packet_pair("[[1],[2,3,4]]\n[[1],4]").unwrap(),
            ("", (l![l![i!(1)],l![i!(2),i!(3),i!(4)]],l![l![i!(1)],i!(4)]))
        );
    }
    #[test]
    fn parse_input_packet() {
        assert_eq!(parse_packet("[]").unwrap(), ("", l![]));
        assert_eq!(parse_packet("[3]").unwrap(), ("", l![i!(3)]));
        assert_eq!(parse_packet("[[]]").unwrap(), ("", l![l![]]));
        assert_eq!(parse_packet("[1,3,1]").unwrap(),("", l![i!(1),i!(3),i!(1)]));
        assert_eq!(
            parse_packet("[[1],[2,3]]").unwrap(),
            ("", l![l![i!(1)],l![i!(2),i!(3)]])
        );
        assert_eq!(
            parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap(),
            ("", l![i!(1),l![i!(2),l![i!(3),l![i!(4),l![i!(5),i!(6),i!(7)]]]],i!(8),i!(9)])
        );
    }
}

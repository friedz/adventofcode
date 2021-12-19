
use std::{
    cmp::{
        max,
        min,
    },
    error::Error,
    fmt::{
        self,
        Display,
    },
    ops::Index,
    slice::SliceIndex,
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

#[derive(Debug, Eq, PartialEq)]
struct Bits {
    bits: Vec<bool>,
}
impl Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for bit in &self.bits {
            write!(f, "{}", if *bit { 1 } else { 0 })?;
        }
        Ok(())
    }
}
impl FromStr for Bits {
    type Err = SimpleError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Bits {
            bits: input.chars().try_fold(Vec::new(), |mut bits, hex| -> SimpleResult<Vec<bool>> {
                bits.append(&mut match hex {
                    '0' => vec![false, false, false, false], // 0000
                    '1' => vec![false, false, false,  true], // 0001
                    '2' => vec![false, false,  true, false], // 0010
                    '3' => vec![false, false,  true,  true], // 0011
                    '4' => vec![false,  true, false, false], // 0100
                    '5' => vec![false,  true, false,  true], // 0101
                    '6' => vec![false,  true,  true, false], // 0110
                    '7' => vec![false,  true,  true,  true], // 0111
                    '8' => vec![ true, false, false, false], // 1000
                    '9' => vec![ true, false, false,  true], // 1001
                    'A' => vec![ true, false,  true, false], // 1010
                    'B' => vec![ true, false,  true,  true], // 1011
                    'C' => vec![ true,  true, false, false], // 1100
                    'D' => vec![ true,  true, false,  true], // 1101
                    'E' => vec![ true,  true,  true, false], // 1110
                    'F' => vec![ true,  true,  true,  true], // 1111
                     e  => { return Err(simple_error!("{} is not a hex number!", e)); },
                });
                Ok(bits)
            })?,
        })
    }
}
impl<I: SliceIndex<[bool]>> Index<I> for Bits {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.bits[index]
    }
}
fn u8_from_slice(s: &[bool]) -> u8 {
    s.iter().fold(0, |res, &i| {
        2*res + if i { 1 } else { 0 }
    })
}
fn i64_from_slice(slice: &[bool]) -> (usize, i64) {
    let (mut start, mut end) = (0, 5);
    let mut res = 0;
    loop {
        let s = &slice[start..end];
        for bit in &s[1..5] {
            res = res * 2 + if *bit { 1 } else { 0 };
        }
        if !s[0] {
            break;
        }
        start = end;
        end += 5;
    }
    (end, res)
}
fn usize_from_slice(slice: &[bool]) -> usize {
    slice.iter().fold(0, |res, bit| {
        res * 2 + if *bit { 1 } else { 0 }
    })
}
fn print_slice(slice: &[bool]) {
    for bit in slice {
        print!("{}", if *bit { 1 } else { 0 });
    }
    println!("");
}

#[derive(Debug, Eq, PartialEq)]
struct Operator {
    type_id: u8,
    sub_packets: Vec<Packet>,
}
impl Operator {
    fn type_id(&self) -> u8 {
        self.type_id
    }
    fn version_sum(&self) -> u64 {
        self.sub_packets.iter().fold(0, |sum, packet| {
            sum + packet.version_sum()
        })
    }
    fn evaluate(&self) -> i64 {
        match self.type_id {
            0 => self.sub_packets.iter().fold(0, |sum, pack| pack.evaluate() + sum),
            1 => self.sub_packets.iter().fold(1, |product, pack| pack.evaluate() * product),
            2 => self.sub_packets[1..].iter().fold(self.sub_packets[0].evaluate(), |minimum, pack| {
                min(minimum, pack.evaluate())
            }),
            3 => self.sub_packets[1..].iter().fold(self.sub_packets[0].evaluate(), |maximum, pack| {
                max(maximum, pack.evaluate())
            }),
            5 => {
                self.sub_packets[1..].iter()
                    .fold((1, self.sub_packets[0].evaluate()), |(res, last), pack| {
                        let this = pack.evaluate();
                        if 1 == res && last > this {
                            (1, this)
                        } else {
                            (0, this)
                        }
                    })
                .0
            },
            6 => {
                self.sub_packets[1..].iter()
                    .fold((1, self.sub_packets[0].evaluate()), |(res, last), pack| {
                        let this = pack.evaluate();
                        if 1 == res && last < this {
                            (1, this)
                        } else {
                            (0, this)
                        }
                    })
                .0
            },
            7 => {
                self.sub_packets[1..].iter()
                    .fold((1, self.sub_packets[0].evaluate()), |(res, last), pack| {
                        let this = pack.evaluate();
                        if 1 == res && last == this {
                            (1, this)
                        } else {
                            (0, this)
                        }
                    })
                .0
            },
            _ => -1,
        }
    }
    fn from_slice(tid: u8, slice: &[bool]) -> (usize, Operator) {
        let mut sub_packets = Vec::new();
        let mut end = 1;
        if slice[0] {
            let num = usize_from_slice(&slice[1..12]);
            end += 11;
            for _ in 0..num {
                let (e, packet) = Packet::from_slice(&slice[end..]);
                sub_packets.push(packet);
                end += e;
            }
        } else {
            let length = usize_from_slice(&slice[1..16]);
            let mut start = 16;
            end = length + start;
            let mut s = &slice[start..end];
            while s.len() >= 11 {
                let (e, packet) = Packet::from_slice(&s);
                sub_packets.push(packet);
                start += e;
                s = &slice[start..end];
            }
        }
        (end, Operator {
            type_id: tid,
            sub_packets: sub_packets,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Type {
    Literal(i64),
    Operator(Operator),
}
impl Type {
    fn type_id(&self) -> u8 {
        match self {
            Type::Literal(_) => 4,
            Type::Operator(o) => o.type_id(),
        }
    }
    fn from_slice(slice: &[bool]) -> (usize, Type) {
        let typ = u8_from_slice(&slice[0..3]);
        match typ {
            4 => {
                let (end, val) = i64_from_slice(&slice[3..]);
                (end + 3, Type::Literal(val))
            }
            tid => {
                let (end, op) = Operator::from_slice(tid, &slice[3..]);
                (end + 3, Type::Operator(op))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    content: Type,
}
impl Packet {
    fn version(&self) -> u8 {
        self.version
    }
    fn version_sum(&self) -> u64 {
        self.version as u64 + match &self.content {
            Type::Literal(_) => 0,
            Type::Operator(o) => o.version_sum(),
        }
    }
    fn evaluate(&self) -> i64 {
        match &self.content {
            Type::Literal(v) => *v,
            Type::Operator(o) => o.evaluate(),
        }
    }
    fn from_bits(bits: &Bits) -> Packet {
        Self::from_slice(&bits[..]).1
    }
    fn from_slice(slice: &[bool]) -> (usize, Packet) {
        let version = u8_from_slice(&slice[0..3]);
        let (end, content) = Type::from_slice(&slice[3..]);
        (end + 3, Packet {
            version: version,
            content: content,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt").trim();
    let bits = Bits::from_str(input)?;
    let packet = Packet::from_bits(&bits);
    println!("{}", packet.version_sum());
    println!("{}", packet.evaluate());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        Bits,
        Operator,
        Packet,
        Type,
    };
    use std::str::FromStr;
    fn check(input: &str, packet: Packet) {
        let bits = Bits::from_str(input).unwrap();
        assert_eq!(packet, Packet::from_bits(&bits));
    }
    fn check_version_sum(input: &str, sum: u64) {
        let bits = Bits::from_str(input).unwrap();
        assert_eq!(sum, Packet::from_bits(&bits).version_sum());
    }
    fn check_evaluate(input: &str, value: i64) {
        let bits = Bits::from_str(input).unwrap();
        assert_eq!(value, Packet::from_bits(&bits).evaluate());
    }
    #[test]
    fn input_d2fe28() {
        let input = "D2FE28";
        let packet = Packet { version: 6, content: Type::Literal(2021) };
        check(input, packet);
    }
    #[test]
    fn input_38006f45291200() {
        let input = "38006F45291200";
        let packet = Packet {
            version: 1,
            content: Type::Operator(
                Operator {
                    type_id: 6,
                    sub_packets: vec![
                        Packet {
                            version: 6,
                            content: Type::Literal(10),
                        },
                        Packet {
                            version: 2,
                            content: Type::Literal(20),
                        }
                    ],
                }
            ),
        };
        check(input, packet);
    }
    #[test]
    fn input_ee00d40c823060() {
        let input = "EE00D40C823060";
        let packet = Packet {
            version: 7,
            content: Type::Operator(Operator {
                type_id: 3,
                sub_packets: vec![
                    Packet {
                        version: 2,
                        content: Type::Literal(1),
                    },
                    Packet {
                        version: 4,
                        content: Type::Literal(2),
                    },
                    Packet {
                        version: 1,
                        content: Type::Literal(3),
                    },
                ],
            })
        };
        // Pack { [ Lit, Lit, Lit] }
        check(input, packet);
    }
    #[test]
    fn input_8a004a801a8002f478() {
        let input = "8A004A801A8002F478"; // 16
        // Pack { Op { Op { Op { Lit } } } }
        check_version_sum(input, 16);
    }
    #[test]
    fn input_620080001611562c8802118e34() {
        let input = "620080001611562C8802118E34"; // 12
        // Pack { Op { Op { Lit }, Op { Lit } } }
        check_version_sum(input, 12);
    }
    #[test]
    fn input_c0015000016115a2e0802f182340() {
        let input = "C0015000016115A2E0802F182340"; // 23
        // Pack { Op { Op { Lit }, Op { Lit } } }
        check_version_sum(input, 23);
    }
    #[test]
    fn input_a0016c880162017c3686b18a3d4780() {
        let input = "A0016C880162017C3686B18A3D4780"; // 31
        // Pack { Op { Op { Op { Lit, Lit, Lit, Lit, Lit } } } }
        check_version_sum(input, 31);
    }
    #[test]
    fn input_c200b40a82() {
        let input = "C200B40A82";
        check_evaluate(input, 3);
    }
    #[test]
    fn input_04005ac33890() {
        let input = "04005AC33890";
        check_evaluate(input, 54);
    }
    #[test]
    fn input_880086c3e88112() {
        let input = "880086C3E88112";
        check_evaluate(input, 7);
    }
    #[test]
    fn input_ce00c43d881120() {
        let input = "CE00C43D881120";
        check_evaluate(input, 9);
    }
    #[test]
    fn input_d8005ac2a8f0() {
        let input = "D8005AC2A8F0";
        check_evaluate(input, 1);
    }
    #[test]
    fn input_f600bc2d8f() {
        let input = "F600BC2D8F";
        check_evaluate(input, 0);
    }
    #[test]
    fn input_9c005ac2f8f0() {
        let input = "9C005AC2F8F0";
        check_evaluate(input, 0);
    }
    #[test]
    fn input_9c0141080250320f1802104a08() {
        let input = "9C0141080250320F1802104A08";
        check_evaluate(input, 1);
    }
}

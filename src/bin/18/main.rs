
use std::{
    cmp::max,
    fmt::{
        self,
        Debug,
        Display,
    },
    ops::Add,
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};
#[derive(Debug, Clone, Eq, PartialEq)]
enum Element {
    RegularNumber(u8),
    SnailfishNumber(Box<SnailfishNumber>),
}
impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::RegularNumber(n) => write!(f, "{}", n),
            Element::SnailfishNumber(sn) => write!(f, "{}", *sn),
        }
    }
}
impl Element {
    fn magnitude(&self) -> u64 {
        match self {
            Element::RegularNumber(n) => *n as u64,
            Element::SnailfishNumber(sn) => sn.magnitude(),
        }
    }
    fn get_regular_number(&self) -> SimpleResult<u8> {
        match self {
            Element::RegularNumber(n) => Ok(*n),
            Element::SnailfishNumber(sn) => Err(simple_error!("{} is not a regular number!", sn)),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct SnailfishNumber([Element; 2]);

impl FromStr for SnailfishNumber {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = s.chars().try_fold(Vec::new(), |mut stack, c| {
            match c {
                ']' => {
                    let right = stack.pop().ok_or(simple_error!("malformed snailfish number"))?;
                    let left = stack.pop().ok_or(simple_error!("malformed snailfish number"))?;
                    stack.push(Element::SnailfishNumber(Box::new(SnailfishNumber([left, right]))));
                },
                '0' => stack.push(Element::RegularNumber(0)),
                '1' => stack.push(Element::RegularNumber(1)),
                '2' => stack.push(Element::RegularNumber(2)),
                '3' => stack.push(Element::RegularNumber(3)),
                '4' => stack.push(Element::RegularNumber(4)),
                '5' => stack.push(Element::RegularNumber(5)),
                '6' => stack.push(Element::RegularNumber(6)),
                '7' => stack.push(Element::RegularNumber(7)),
                '8' => stack.push(Element::RegularNumber(8)),
                '9' => stack.push(Element::RegularNumber(9)),
                '[' | ',' => {},
                w if w.is_whitespace() => {},
                e => return Err(simple_error!("{:?} is not a valid charakter in a snailfisch number!", e)),
            }
            Ok(stack)
        }).or_else(|e| Err(simple_error!("{:?}", e)))?;
        match stack.pop().ok_or(simple_error!("malformed snailfish number"))? {
            Element::RegularNumber(n) => Err(simple_error!("{:?} is not a complete snailfish number!", n)),
            Element::SnailfishNumber(sn) => Ok(*sn)
        }
    }
}
impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.0[0], self.0[1])
    }
}
impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl Add for SnailfishNumber {
    type Output = SnailfishNumber;
    fn add(self, rhs: SnailfishNumber) -> Self::Output {
        let mut res = SnailfishNumber([
            Element::SnailfishNumber(Box::new(self)),
            Element::SnailfishNumber(Box::new(rhs))
        ]);
        res.reduce();
        res
    }
}
impl SnailfishNumber {
    fn reduce(&mut self) -> SimpleResult<()> {
        loop {
            match self.explode(0)? {
                None => {},
                Some(_) => continue,
            }
            if self.split() {
                continue;
            } else {
                break;
            }
        }
        Ok(())
    }
    fn update_left(&mut self, left: u8) {
        match &mut self.0[1] {
            Element::RegularNumber(n) => *n += left,
            Element::SnailfishNumber(sn) => sn.update_left(left),
        }
    }
    fn update_right(&mut self, right: u8) {
        match &mut self.0[0] {
            Element::RegularNumber(n) => *n += right,
            Element::SnailfishNumber(sn) => sn.update_right(right),
        }
    }
    fn explode(&mut self, level: u8) -> SimpleResult<Option<(Option<u8>, Option<u8>)>> {
        let left = match &mut self.0[0] {
            Element::RegularNumber(_) => None,
            Element::SnailfishNumber(sn) => {
                if 3 == level {
                    let left = sn.0[0].get_regular_number()?;
                    let right = sn.0[1].get_regular_number()?;
                    self.0[0] = Element::RegularNumber(0);
                    Some((Some(left), Some(right)))
                } else {
                    sn.explode(level + 1)?
                }
            },
        };
        Ok(match left {
            Some((left, Some(right))) => {
                match &mut self.0[1] {
                    Element::RegularNumber(n) => *n += right,
                    Element::SnailfishNumber(sn) => sn.update_right(right),
                }
                Some((left, None))
            },
            Some((left, None)) => Some((left, None)),
            None => {
                let right = match &mut self.0[1] {
                    Element::RegularNumber(_) => None,
                    Element::SnailfishNumber(sn) => {
                        if 3 == level {
                            let left = sn.0[0].get_regular_number()?;
                            let right = sn.0[1].get_regular_number()?;
                            self.0[1] = Element::RegularNumber(0);
                            Some((Some(left), Some(right)))
                        } else {
                            sn.explode(level + 1)?
                        }
                    },
                };
                match right {
                    Some((Some(left), right)) => {
                        match &mut self.0[0] {
                            Element::RegularNumber(n) => *n += left,
                            Element::SnailfishNumber(sn) => sn.update_left(left),
                        }
                        Some((None, right))
                    },
                    Some((None, right)) => Some((None, right)),
                    None => {
                        None
                    },
                }
            },
        })
    }
    fn simple(left: u8, right: u8) -> Box<SnailfishNumber> {
        Box::new(SnailfishNumber([
                Element::RegularNumber(left),
                Element::RegularNumber(right)
        ]))
    }
    fn split(&mut self) -> bool {
        let left = match &mut self.0[0] {
            Element::RegularNumber(n) if *n > 9 => {
                self.0[0] = Element::SnailfishNumber(SnailfishNumber::simple(*n/2, (*n + 1)/2));
                true
            },
            Element::RegularNumber(_) => false,
            Element::SnailfishNumber(sn) => sn.split(),
        };
        if !left {
            match &mut self.0[1] {
                Element::RegularNumber(n) if *n > 9 => {
                    self.0[1] = Element::SnailfishNumber(SnailfishNumber::simple(*n/2, (*n + 1)/2));
                    true
                },
                Element::RegularNumber(_) => false,
                Element::SnailfishNumber(sn) => sn.split(),
            }
        } else {
            true
        }
    }
    fn magnitude(&self) -> u64 {
        3 * self.0[0].magnitude() + 2 * self.0[1].magnitude()
    }
}

fn main() -> SimpleResult<()> {
    let input = include_str!("input.txt");
    let numbers = input.lines().try_fold(Vec::new(), |mut numbers, line| -> SimpleResult<Vec<SnailfishNumber>> {
        let line = line.trim();
        numbers.push(SnailfishNumber::from_str(line)?);
        Ok(numbers)
    }).unwrap();
    let res = numbers[1..].into_iter().fold(numbers[0].clone(), |res, sn| {
        res + sn.clone()
    });
    println!("{} -> {}", res, res.magnitude());

    let mut part_two = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                part_two = max(part_two, (numbers[i].clone() + numbers[j].clone()).magnitude());
            }
        }
    }
    println!("{}", part_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::SnailfishNumber;
    use std::str::FromStr;
    use simple_error::SimpleResult;
    #[test]
    fn simple_addition() {
        let res = SnailfishNumber::from_str("[1,1]").unwrap()
            + SnailfishNumber::from_str("[2,2]").unwrap()
            + SnailfishNumber::from_str("[3,3]").unwrap()
            + SnailfishNumber::from_str("[4,4]").unwrap();
        assert_eq!(res, SnailfishNumber::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap());
    }
    #[test]
    fn addition_with_explosion() {
        let res = SnailfishNumber::from_str("[1,1]").unwrap()
            + SnailfishNumber::from_str("[2,2]").unwrap()
            + SnailfishNumber::from_str("[3,3]").unwrap()
            + SnailfishNumber::from_str("[4,4]").unwrap()
            + SnailfishNumber::from_str("[5,5]").unwrap();
        assert_eq!(res, SnailfishNumber::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap());

        let res = SnailfishNumber::from_str("[1,1]").unwrap()
            + SnailfishNumber::from_str("[2,2]").unwrap()
            + SnailfishNumber::from_str("[3,3]").unwrap()
            + SnailfishNumber::from_str("[4,4]").unwrap()
            + SnailfishNumber::from_str("[5,5]").unwrap()
            + SnailfishNumber::from_str("[6,6]").unwrap();
        assert_eq!(res, SnailfishNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap());
    }
    #[test]
    fn addition_with_explosion_and_split() {
        let res = SnailfishNumber::from_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").unwrap()
            + SnailfishNumber::from_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap()
            + SnailfishNumber::from_str("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]").unwrap()
            + SnailfishNumber::from_str("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]").unwrap()
            + SnailfishNumber::from_str("[7,[5,[[3,8],[1,4]]]]").unwrap()
            + SnailfishNumber::from_str("[[2,[2,2]],[8,[8,1]]]").unwrap()
            + SnailfishNumber::from_str("[2,9]").unwrap()
            + SnailfishNumber::from_str("[1,[[[9,3],9],[[9,0],[0,7]]]]").unwrap()
            + SnailfishNumber::from_str("[[[5,[7,4]],7],1]").unwrap()
            + SnailfishNumber::from_str("[[[[4,2],2],6],[8,7]]").unwrap();
        assert_eq!(res, SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap());
    }
    #[test]
    fn magnitude() {
        assert_eq!(143, SnailfishNumber::from_str("[[1,2],[[3,4],5]]").unwrap().magnitude());
        assert_eq!(1384, SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().magnitude());
        assert_eq!(445, SnailfishNumber::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap().magnitude());
        assert_eq!(791, SnailfishNumber::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().magnitude());
        assert_eq!(1137, SnailfishNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().magnitude());
        assert_eq!(3488, SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap().magnitude());
    }
    #[test]
    fn example_homework() {
        let input = include_str!("example_input.txt");
        let numbers = input.lines().try_fold(Vec::new(), |mut numbers, line| -> SimpleResult<Vec<SnailfishNumber>> {
            let line = line.trim();
            numbers.push(SnailfishNumber::from_str(line)?);
            Ok(numbers)
        }).unwrap();
        let res = numbers[1..].into_iter().fold(numbers[0].clone(), |res, sn| {
            res + sn.clone()
        });
        assert_eq!(res, SnailfishNumber::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").unwrap());
        assert_eq!(res.magnitude(), 4140);
    }
}

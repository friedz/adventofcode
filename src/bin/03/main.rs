
use std::{
    error::Error,
    ops::Add,
    str::FromStr,
};

use csv::ReaderBuilder;
use simple_error::SimpleError;

const BITS: usize = 12;

#[derive(Debug)]
struct BinaryDiagnostic {
    num: usize,
    sum: [usize; BITS],
}

impl BinaryDiagnostic {
    fn new() -> BinaryDiagnostic {
        BinaryDiagnostic {
            num: 0,
            sum: [0; BITS],
        }
    }
    fn rate<F>(&self, compare: F) -> u16
        where F: Fn(usize, usize) -> bool {
        self.sum.into_iter().fold(0, |acc, i| {
            acc * 2 + if compare(i, self.num) {
                1
            } else {
                0
            }
        })
    }
    fn gamma_rate(&self) -> u16 {
        //self.rate( |i, num| 2*i > num)
        self.rate( |i, num| i > num/2)
    }
    fn epsilon_rate(&self) -> u16 {
        //self.rate( |i, num| 2*i < num)
        self.rate( |i, num| i < num/2)
    }
    fn result(&self) -> u32 {
        self.gamma_rate() as u32 * self.epsilon_rate() as u32
    }
}

impl FromStr for BinaryDiagnostic {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bd = BinaryDiagnostic::new();
        bd.num = 1;
        if s.len() != BITS {
            return Err(SimpleError::new(format!("{} has the wrong number of bits", s)));
        }
        let bytes = s.as_bytes();
        for i in 0..BITS {
            bd.sum[i] += if b'1' == bytes[i] {
                1
            } else if b'0' == bytes[i] {
                0
            } else {
                return Err(SimpleError::new(format!("{} is not in the correct format", s)));
            }
        }
        Ok(bd)
    }
}

impl Add<&BinaryDiagnostic> for BinaryDiagnostic {
    type Output = Self;

    fn add(self, rhs: &BinaryDiagnostic) -> Self::Output {
        let mut lhs = self;
        lhs.num += rhs.num;
        for i in 0..BITS {
            lhs.sum[i] += rhs.sum[i];
        }
        lhs
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let foo = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(include_str!("input.txt").as_bytes())
        .records()
        .into_iter()
        .fold(Ok(BinaryDiagnostic::new()), |acc: Result<BinaryDiagnostic, Box<dyn Error>>, i| {
            Ok(acc? + &BinaryDiagnostic::from_str(i?.as_slice())?)
        })?;
    println!("γ: {}, ε: {}, γ × ε = {}", foo.gamma_rate(), foo.epsilon_rate(), foo.result());

    Ok(())
}

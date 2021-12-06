
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
    fn from_bits(bits: &[u8; BITS]) -> BinaryDiagnostic {
        let mut bd = BinaryDiagnostic::new();
        bd.num = 1;
        for i in 0..BITS {
            bd.sum[i] = bits[i] as usize;
        }
        bd
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
        self.rate( |i, num| 2*i >= num)
    }
    fn epsilon_rate(&self) -> u16 {
        self.rate( |i, num| 2*i < num)
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

fn bits_from_str(s: &str) -> Result<[u8; BITS], SimpleError> {
    let inpt = s.as_bytes();
    if inpt.len() != BITS {
        return Err(SimpleError::new(format!("{:?} has the wrong length", s)));
    }
    let mut arr = [0; BITS];
    for i in 0..BITS {
        arr[i] = match inpt[i] {
            b'1' => 1,
            b'0' => 0,
            e => return Err(SimpleError::new(format!("{:?} is not 1|0", e)))
        };
    }
    Ok(arr)
}

fn oxygen_co2_step(arr: &Vec<[u8; BITS]>, i: usize, cmp: fn(usize, usize) -> bool) -> Result<Vec<[u8; BITS]>, SimpleError> {
    if i >= BITS {
        return Err(SimpleError::new(format!("there are less then {} bits", i)));
    }
    let (ones, zeros) = arr.iter().fold((0, 0), |(ones, zeros), v| {
        if 1 == v[i] {
            (ones + 1, zeros)
        } else {
            (ones, zeros + 1)
        }
    });
    let mut tmp = Vec::new();
    for entry in arr.iter().filter(|v| {
        (v[i] == 1) == cmp(ones, zeros)
    }) {
        tmp.push(*entry);
    }
    Ok(tmp)
}

fn build_number(number: &[u8; BITS]) -> u32 {
    number.iter().fold(0, |num, x| num * 2 + *x as u32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (foo, arr) = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(include_str!("input.txt").as_bytes())
        .records()
        .into_iter()
        .fold(Ok((BinaryDiagnostic::new(), Vec::new())), |acc: Result<(BinaryDiagnostic, Vec<[u8; BITS]>), Box<dyn Error>>, i| {
            let (acu, mut arr) = acc?;

            let line = bits_from_str(&i?.as_slice())?;
            let bd = BinaryDiagnostic::from_bits(&line);
            arr.push(line);

            Ok((acu + &bd, arr))
        })?;
    println!("γ: {}, ε: {}, γ × ε = {}", foo.gamma_rate(), foo.epsilon_rate(), foo.result());

    let mut oxygen = arr.clone();
    for o in 0..BITS {
        oxygen = oxygen_co2_step(&oxygen, o, |ones, zeros| ones >= zeros)?;
        if oxygen.len() == 1 {
            break;
        }
    }

    let mut co2 = arr;
    for o in 0..BITS {
        co2 = oxygen_co2_step(&co2, o, |ones, zeros| ones < zeros)?;
        if co2.len() == 1 {
            break;
        }
    }
    let oxygen = build_number(&oxygen[0]);
    let co2 = build_number(&co2[0]);
    println!("oxygen: {}, co2: {}, oxygen × co2 = {}", oxygen, co2, oxygen*co2);


    Ok(())
}

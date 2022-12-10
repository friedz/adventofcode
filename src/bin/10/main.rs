
use std::{
    io::BufRead,
    iter::IntoIterator,
    str::FromStr,
};
use simple_error::SimpleError;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    AddX(i32),
    Noop,
}
impl FromStr for Instruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        let mut parts = s.split(' ');
        Ok(match parts.next() {
            Some("noop") => Instruction::Noop,
            Some("addx") => match parts.next() {
                Some(num) => match num.parse() {
                    Ok(n) => Instruction::AddX(n),
                    Err(e) => { return Err(SimpleError::from(e)); },
                }
                None => { return Err(SimpleError::new("No number!")); },
            }
            e => { return Err(SimpleError::new(format!("ERROR: {:?}", e))); },
        })
    }
}
fn instruction_stream(s: &str) -> Vec<Instruction> {
    s.as_bytes().lines().filter_map(|line| {
        Instruction::from_str(&line.ok()?).ok()
    }).collect()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct CPU<I> {
    x: i32,
    cycles_left: u8,
    change: i32,
    instructions: I,
}
impl<I> CPU<I> {
    fn new(instructions: I) -> CPU<I> {
        CPU {
            x: 1,
            cycles_left: 0,
            change: 0,
            instructions: instructions,
        }
    }
}
impl<I> Iterator for CPU<I> where I: Iterator<Item=Instruction> + Sized {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 == self.cycles_left {
            self.x += self.change;
            match self.instructions.next() {
                Some(Instruction::Noop) => {
                    self.change = 0;
                },
                Some(Instruction::AddX(n)) => {
                    self.change = n;
                    self.cycles_left = 1;
                },
                None => { return None; },
            }
        } else {
            self.cycles_left -= 1;
        }
        Some(self.x)
    }
}
fn part1<I>(data: I) -> i32 where I: IntoIterator<Item=Instruction> {
    let cpu = CPU::new(data.into_iter());
    cpu.enumerate().fold(0, |sum, (step, signal)| {
        let step = step + 1;
        if 20 <= step && (step - 20) % 40 == 0 {
            sum + signal * step as i32
        } else {
            sum
        }
    })
}
fn part2<I>(data: I) -> String where I: IntoIterator<Item=Instruction> {
    const LIT: char = if cfg!(test) { '#' } else { '█' };
    const DARK: char = if cfg!(test) { '.' } else { ' ' };
    let cpu = CPU::new(data.into_iter());
    cpu.enumerate().fold(String::new(), |mut crt, (step, signal)| {
        if 0 < step && step % 40 == 0 {
            crt.push('\n');
        }
        let step = step % 40;
        if (signal - step as i32).abs() <= 1 {
            crt.push(LIT);
        } else {
            crt.push(DARK);
        }
        crt
    })
}

fn main() {
    let input = include_str!("input.txt");
    let data = instruction_stream(input);
    println!("Part 1: {}", part1(data.clone().into_iter()));
    println!("Part 2:");
    println!("{}", part2(data.into_iter()));
}

#[cfg(test)]
mod tests_day_10 {
    use super::{
        *,
        Instruction::*,
    };

    const SMALL_INPUT: &str = "noop\naddx 3\naddx -5";
    const INPUT: &str = include_str!("example.txt");
    const OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    fn small_data() -> Vec<Instruction> {
        vec![Noop, AddX(3), AddX(-5)]
    }

    #[test]
    fn parse_instruction() {
        assert_eq!(Instruction::from_str("noop"), Ok(Noop));
        assert_eq!(Instruction::from_str("addx 3"), Ok(AddX(3)));
        assert_eq!(Instruction::from_str("addx -5"), Ok(AddX(-5)));
    }
    #[test]
    fn parse_input() {
        let instr = instruction_stream(SMALL_INPUT);
        let mut instr = instr.iter();

        assert_eq!(instr.next(), Some(&small_data()[0]));
        assert_eq!(instr.next(), Some(&small_data()[1]));
        assert_eq!(instr.next(), Some(&small_data()[2]));
        assert_eq!(instr.next(), None);
    }
    #[test]
    fn run_small_program() {
        let mut cpu = CPU::new(small_data().into_iter());
        // noon
        assert_eq!(cpu.next(), Some(1));

        // addx 3
        assert_eq!(cpu.next(), Some(1));
        assert_eq!(cpu.next(), Some(1));

        // addx -5
        assert_eq!(cpu.next(), Some(4));
        assert_eq!(cpu.next(), Some(4));

        assert_eq!(cpu.next(), None);
    }
    #[test]
    fn run_example_program() {
        let instr = instruction_stream(INPUT).into_iter();
        assert_eq!(part1(instr), 13140);
    }
    #[test]
    fn run_example_program2() {
        let instr = instruction_stream(INPUT).into_iter();
        assert_eq!(part2(instr), OUTPUT);
    }
}

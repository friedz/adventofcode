
use std::{
    io::Error,
    fs::File,
    ops::Add,
};

use csv::ReaderBuilder;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }
    fn finalize(&self) -> i32 {
        self.horizontal * self.depth
    }
}

impl Add<&Command> for Position {
    type Output = Position;

    fn add(self, other: &Command) -> Self::Output {
        let mut res = self;
        match other {
            Command::Forward(n) => res.horizontal += n,
            Command::Down(n) => res.depth += n,
            Command::Up(n) => res.depth -= n,
        };
        res
    }
}

#[derive(Debug)]
struct AimPosition {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl AimPosition {
    fn new() -> AimPosition {
        AimPosition {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }
    fn finalize(self) -> i32 {
        self.horizontal * self.depth
    }
}

impl Add<&Command> for AimPosition {
    type Output = AimPosition;

    fn add(self, other: &Command) -> Self::Output {
        let mut res = self;
        match other {
            Command::Forward(n) => {
                res.horizontal += n;
                res.depth += res.aim * n;
            },
            Command::Down(n) => res.aim += n,
            Command::Up(n) => res.aim -= n,
        };
        res
    }
}
fn main() -> Result<(), Error> {
    let (position, aim_position) =  ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(File::open("src/bin/02/input.txt")?)
        .records()
        .into_iter()
        .fold((Position::new(), AimPosition::new()), |(pos, apos), next| {
            let next = next.expect("Not a Command!");
            //pos + next.expect("Not a valid Command!")
            let n = next.get(1).unwrap().parse::<i32>().unwrap();
            let cmd = match next.get(0).unwrap() {
                "forward" => Command::Forward(n),
                "down" => Command::Down(n),
                "up" => Command::Up(n),
                _ => Command::Forward(0),
            };
            (pos + &cmd, apos + &cmd)
        });

    println!("{}", position.finalize());
    println!("{}", aim_position.finalize());

    Ok(())
}

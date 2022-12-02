
use std::{
    io::BufRead,
    str::FromStr,
};
use simple_error::SimpleError;

use RockPaperScissors::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}
impl FromStr for RockPaperScissors {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RockPaperScissors::Rock),
            "B" | "Y" => Ok(RockPaperScissors::Paper),
            "C" | "Z" => Ok(RockPaperScissors::Scissors),
            e => Err(SimpleError::new(format!("{} not a Rock Paper Scissors move", e))),
        }
    }
}
impl RockPaperScissors {
    fn shape_score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    fn round(&self, opponent: &Self) -> u32 {
        self.shape_score() + match (self, opponent) {
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Paper, Scissors) => 0,
            (Scissors, Paper) => 6,
            (Scissors, Rock) => 0,
            (_, _) => 3,
        }
    }
}


fn tournament<I>(rounds: I) -> u32
where I: IntoIterator<Item = (RockPaperScissors, RockPaperScissors)> {
    rounds.into_iter().fold(0, |score, (opponent, me)| {
        score + me.round(&opponent)
    })
}

fn read_data(data: &str) -> Result<Vec<(RockPaperScissors, RockPaperScissors)>, SimpleError> {
    data.as_bytes().lines().try_fold(Vec::new(), |mut list, round| {
        let round = round.expect("a correctly read line for a match");
        let mut moves = round.split(' ');
        list.push((moves.next().ok_or("no gesture")?.parse()?, moves.next().ok_or("no gesture")?.parse()?));
        Ok(list)
    })
}

fn main() {
    let data = read_data(include_str!("input.txt")).expect("correctly read file");
    let score = tournament(data);
    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "A Y\nB X\nC Z";
    static DATA: [(RockPaperScissors, RockPaperScissors); 3] = [
        (RockPaperScissors::Rock,     RockPaperScissors::Paper),
        (RockPaperScissors::Paper,    RockPaperScissors::Rock),
        (RockPaperScissors::Scissors, RockPaperScissors::Scissors),
    ];

    #[test]
    fn read_example_data() {
        let parsed_data = read_data(INPUT).unwrap();
        assert_eq!(parsed_data.len(), 3);
        for i in 0..parsed_data.len() {
            assert_eq!(parsed_data[i], DATA[i]);
        }
    }
    #[test]
    fn run_tournament() {
        assert_eq!(tournament(DATA), 15);
    }
}

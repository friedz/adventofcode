
use std::{
    io::BufRead,
    str::FromStr,
};
use simple_error::SimpleError;

use RockPaperScissors::*;
use Outcome::*;

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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
impl FromStr for Outcome {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            e => Err(SimpleError::new(format!("{} is not a valid game outcome", e))),
        }
    }
}
impl Outcome {
    fn round(&self, opponent: &RockPaperScissors) -> u32 {
        match (self, opponent) {
            (Win, Rock) => Paper.round(opponent),
            (Win, Paper) => Scissors.round(opponent),
            (Win, Scissors) => Rock.round(opponent),
            (Lose, Rock) => Scissors.round(opponent),
            (Lose, Paper) => Rock.round(opponent),
            (Lose, Scissors) => Paper.round(opponent),
            (Draw, rps) => rps.round(rps),
        }
    }
}

fn tournament<I>(rounds: I) -> u32
where I: IntoIterator<Item = (RockPaperScissors, RockPaperScissors)> {
    rounds.into_iter().fold(0, |score, (opponent, me)| {
        score + me.round(&opponent)
    })
}

fn fixed_tournament<I>(rounds: I) -> u32
where I: IntoIterator<Item = (RockPaperScissors, Outcome)> {
    rounds.into_iter().fold(0, |score, (opponent, result)| {
        score + result.round(&opponent)
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

fn correctly_read_data(data: &str) -> Result<Vec<(RockPaperScissors, Outcome)>, SimpleError> {
    data.as_bytes().lines().try_fold(Vec::new(), |mut list, round| {
        let round = round.expect("a correctly read line for a match");
        let mut moves = round.split(' ');
        list.push((moves.next().ok_or("no gesture")?.parse()?, moves.next().ok_or("no outcome")?.parse()?));
        Ok(list)
    })
}

fn main() {
    let input = include_str!("input.txt");

    let data = read_data(input).expect("correctly read file");
    let score = tournament(data);
    println!("Part 1: {}", score);

    let data = correctly_read_data(input).expect("correctly read file");
    let score = fixed_tournament(data);
    println!("Part 2: {}", score);
}

#[cfg(test)]
mod tests_day_02 {
    use super::*;

    static INPUT: &str = "A Y\nB X\nC Z";
    static DATA: [(RockPaperScissors, RockPaperScissors); 3] = [
        (RockPaperScissors::Rock,     RockPaperScissors::Paper),
        (RockPaperScissors::Paper,    RockPaperScissors::Rock),
        (RockPaperScissors::Scissors, RockPaperScissors::Scissors),
    ];
    static CORRECT_DATA: [(RockPaperScissors, Outcome); 3] = [
        (RockPaperScissors::Rock,     Outcome::Draw),
        (RockPaperScissors::Paper,    Outcome::Lose),
        (RockPaperScissors::Scissors, Outcome::Win),
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
    #[test]
    fn read_in_the_correct_way() {
        let parsed_data = correctly_read_data(INPUT).unwrap();
        assert_eq!(parsed_data.len(), 3);
        for i in 0..parsed_data.len() {
            assert_eq!(parsed_data[i], CORRECT_DATA[i]);
        }
    }
    #[test]
    fn run_fixed_tournament() {
        assert_eq!(fixed_tournament(CORRECT_DATA), 12);
    }
}

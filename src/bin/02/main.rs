
use std::cmp::{
    max,
    min,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    character::complete,
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}
impl CubeSet {
    fn null() -> CubeSet {
        CubeSet { red: 0, green: 0, blue: 0 }
    }
    fn max() -> CubeSet {
        CubeSet { red: i32::MAX, green: i32::MAX, blue: i32::MAX }
    }
    fn new(red: i32, green: i32, blue: i32) -> CubeSet {
        CubeSet { red, green, blue }
    }
    fn power(&self) -> i32 {
        self.red * self.blue * self.green
    }
    fn color(mut self, c: &Color) -> CubeSet {
        match c {
            Color::Red(n) => self.red = *n,
            Color::Blue(n) => self.blue = *n,
            Color::Green(n) => self.green = *n,
        }
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: i32,
    sets: Vec<CubeSet>,
    min_amounts: CubeSet,
}
impl Game {
    fn new(id: i32, csl: Vec<CubeSet>) -> Game {
        let min_amounts = csl.iter().fold(CubeSet::null(), |mi, cs| {
            CubeSet {
                red: max(mi.red, cs.red),
                green: max(mi.green, cs.green),
                blue: max(mi.blue, cs.blue)
            }
        });
        Game { id, sets: csl, min_amounts }
    }
    fn posible(&self, cs: &CubeSet) -> bool {
        self.min_amounts.red <= cs.red
            && self.min_amounts.green <= cs.green
            && self.min_amounts.blue <= cs.blue
    }
}

enum Color {
    Red(i32),
    Green(i32),
    Blue(i32),
}

fn parse_color(s: &str) -> IResult<&str, Color> {
    let (s, amount) = complete::i32(s)?;
    let (s, _) = tag(" ")(s)?;
    let (s, c) = alt((
            map(tag("blue"), |_| Color::Blue(amount)),
            map(tag("green"), |_| Color::Green(amount)),
            map(tag("red"), |_| Color::Red(amount))
            ))(s)?;
    Ok((s, c))
}

// 3 blue, 4 red
// 1 red, 2 green, 6 blue
// 2 green
fn parse_cube_set(s: &str) -> IResult<&str, CubeSet> {
       let (s, cs) = separated_list0(tag(", "), parse_color)(s)?;
       Ok((s, cs.iter().fold(CubeSet::null(), |cs, c| {
           cs.color(c)
       })))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
fn parse_game(s: &str) -> IResult<&str, Game> {
    let (s, _) = tag("Game ")(s)?;
    let (s, id) = complete::i32(s)?;
    let (s, _) = tag(": ")(s)?;
    let (s, list) = separated_list0(tag("; "), parse_cube_set)(s)?;
    let game = Game::new(id, list);
    Ok((s, game))
}

fn parse_all(s: &str) -> IResult<&str, Vec<Game>> {
    separated_list0(complete::newline, parse_game)(s)
}

fn part_1(games: &Vec<Game>, fit: &CubeSet) -> i32 {
    games.iter().fold(0, |sum, game| {
        if game.posible(fit) {
            sum + game.id
        } else {
            sum
        }
    })
}

fn part_2(games: &Vec<Game>) -> i32 {
    games.iter().fold(0, |sum, game| {
        game.min_amounts.power() + sum
    })
}

fn main() {
    let input = include_str!("input.txt");
    let (_, games) = parse_all(input).unwrap();
    println!("# Day 2");
    let fit = CubeSet::new(12, 13, 14);
    let part1 = part_1(&games, &fit);
    println!("- part 1: {}", part1);
    let part2 = part_2(&games);
    println!("- part 2: {}", part2);
}

#[cfg(test)]
mod tests_day_01 {
    use super::*;

    const INPUT: &'static str =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn check_part_1() {
        let fit = CubeSet::new(12, 13, 14);
        let (_, games) = parse_all(INPUT).unwrap();
        assert_eq!(part_1(&games, &fit), 8)
    }
    #[test]
    fn check_part_2() {
        let (_, games) = parse_all(INPUT).unwrap();
        assert_eq!(part_2(&games), 2286)
    }
}


use std::error::Error;
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

//const WIN_SCORE: usize = 21;
const WIN_SCORE: usize = 10;

#[derive(Debug, Clone, Copy)]
enum Id {
    One,
    Two,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    id: Id,
    position: usize,
    score: usize,
}
impl Player {
    fn new(pos: usize, id: Id) -> Player {
        Player {
            id: id,
            position: pos,
            score: 0
        }
    }
    fn one(pos: usize) -> Player {
        Player::new(pos, Id::One)
    }
    fn two(pos: usize) -> Player {
        Player::new(pos, Id::Two)
    }
    fn turn(&self, step: usize) -> Player {
        Player {
            id: self.id,
            position: (self.position + step) % 10,
            score: self.score + if self.position == 0 { 10 } else { self.position },
        }
    }
    fn won(&self) -> bool {
        self.score >= WIN_SCORE
    }
    fn id(&self) -> Id {
        self.id
    }
}

fn play(player: [Player; 2]) -> (usize, usize) {
    if player[0].won() {
        match player[0].id() {
            Id::One => return (1, 0),
            Id::Two => return (0, 1),
        }
    }
    if player[1].won() {
        match player[1].id() {
            Id::One => return (1, 0),
            Id::Two => return (0, 1),
        }
    }

    let (one3, two3) = play([player[1], player[0].turn(3)]);
    let (one4, two4) = play([player[1], player[0].turn(4)]);
    let (one4, two4) = (one4*3, two4*3);
    let (one5, two5) = play([player[1], player[0].turn(5)]);
    let (one5, two5) = (one5*6, two5*6);
    let (one6, two6) = play([player[1], player[0].turn(6)]);
    let (one6, two6) = (one6*7, two6*7);
    let (one7, two7) = play([player[1], player[0].turn(7)]);
    let (one7, two7) = (one7*6, two7*6);
    let (one8, two8) = play([player[1], player[0].turn(8)]);
    let (one8, two8) = (one8*3, two8*3);
    let (one9, two9) = play([player[1], player[0].turn(9)]);
    (
        one3 + one4 + one5 + one6 + one7 + one8 + one9,
        two3 + two4 + two5 + two6 + two7 + two8 + two9
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let mut inpt = input.lines();
    let one: usize = inpt.next().ok_or(simple_error!("no input lines"))?
        .strip_prefix("Player 1 starting position: ").ok_or(simple_error!("not a player position line"))?
        .trim().parse()?;
    let two: usize = inpt.next().ok_or(simple_error!("no input lines"))?
        .strip_prefix("Player 2 starting position: ").ok_or(simple_error!("not a player position line"))?
        .trim().parse()?;
    println!("Player 1 starts at {}", one);
    println!("Player 2 starts at {}", two);
    println!("");

    let one_off = [6, 4, 2, 0, 8];
    let two_off = [5, 3, 1, 9, 7];

    let mut one_score = 0;
    //let mut one_pos = 4;
    let mut one_pos = one;

    let mut two_score = 0;
    //let mut two_pos = 8;
    let mut two_pos = two;

    let mut i = 0;
    loop {
        one_pos = (one_pos + one_off[i%5])%10;
        one_score += if one_pos == 0 { 10 } else { one_pos };
        if one_score >= 1000 {
            let dice = i*6 + 3;
            println!("One wins with {} points", one_score);
            println!("Two has {} points", two_score);
            println!("dice have been rolled {} times", dice);
            println!("{} * {} = {}", two_score, dice, two_score * dice);
            break;
        }
        two_pos = (two_pos + two_off[i%5])%10;
        two_score += if two_pos == 0 { 10 } else { two_pos };
        if two_score >= 1000 {
            let dice = i*6 + 6;
            println!("Two wins with {} points", two_score);
            println!("One has {} points", one_score);
            println!("dice have been rolled {} times", dice);
            println!("{} * {} = {}", one_score, dice, one_score * dice);
            break;
        }
        i += 1;
    }
    println!("");

    //println!("{:?}", play([Player::one(one), Player::two(two)]));
    println!("{:?}", play([Player::one(4), Player::two(8)]));
    println!("{:?}", play([Player::two(8), Player::one(4)]));

    Ok(())
}

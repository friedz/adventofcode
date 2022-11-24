
use std::{
    cmp,
    error::Error,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

const WIN_SCORE: usize = 21;

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
        let new_pos = (self.position + step) % 10;
        Player {
            id: self.id,
            position: new_pos,
            score: self.score + if new_pos == 0 { 10 } else { new_pos },
        }
    }
    fn won(&self) -> bool {
        self.score >= WIN_SCORE
    }
    fn id(&self) -> Id {
        self.id
    }
}

fn play(player: [Player; 2], universes: usize) -> (usize, usize) {
    if player[1].won() {
        match player[1].id() {
            Id::One => return (universes, 0),
            Id::Two => return (0, universes),
        }
    }
    let (one3, two3) = play([player[1], player[0].turn(3)], 1);
    let (one4, two4) = play([player[1], player[0].turn(4)], 3);
    let (one5, two5) = play([player[1], player[0].turn(5)], 6);
    let (one6, two6) = play([player[1], player[0].turn(6)], 7);
    let (one7, two7) = play([player[1], player[0].turn(7)], 6);
    let (one8, two8) = play([player[1], player[0].turn(8)], 3);
    let (one9, two9) = play([player[1], player[0].turn(9)], 1);
    (
        (one3 + one4 + one5 + one6 + one7 + one8 + one9) * universes,
        (two3 + two4 + two5 + two6 + two7 + two8 + two9) * universes
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

    let res = play([Player::one(one), Player::two(two)], 1);
    println!("Player 1 won in {} universes and player 2 in {}", res.0, res.1);
    if res.0 > res.1 {
        println!("which means playler 1 wins in more universes, namly {}", res.0)
    } else if res.1 > res.0 {
        println!("which means playler 2 wins in more universes, namly {}", res.1)
    } else {
        println!("which means they both win in exactly the same number of universes universes, namly {}", res.1)
    }

    Ok(())
}

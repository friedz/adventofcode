
use std::error::Error;
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};


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


    Ok(())
}


use std::{
    io::BufRead,
    str::FromStr,
};
use simple_error::SimpleError;

#[derive(Debug)]
struct CargoStacks {
    stacks: Vec<Vec<char>>,
}

/*
impl CargoStacks {
    fn move_crates(&mut self, mv: &MoveInstruction) {
        for _ in 0..mv.ammount() {
            self.stacks[mv.to() - 1].push(self.stacks[mv.from() - 1].pop().unwrap());
        }
    }
    fn tops(&self) -> String {
        stacks.into_iter().fold(String::new(), |mut res, stack| {
            res.push(stack[stack.last().unwrap());
            res
        })
    }
}

impl FromStr for CargoStacks {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes().lines().rev()
    }
}
*/

#[derive(Debug, Eq, PartialEq)]
struct MoveInstruction {
    amount: u32,
    from: usize,
    to: usize,
}
impl MoveInstruction {
    fn new(amount: u32, from: usize, to: usize) -> MoveInstruction {
        MoveInstruction {
            amount: amount,
            from: from,
            to: to,
        }
    }
    fn ammount(&self) -> u32 {
        self.amount
    }
    fn from(&self) -> usize {
        self.from
    }
    fn to(&self) -> usize {
        self.to
    }
}
macro_rules! mv {
    (move $a:literal from $from:literal to $to:literal) => {
        MoveInstruction { amount: $a, from: $from, to: $to }
    };
}

impl FromStr for MoveInstruction {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let _ = words.next();
        let amount: u32 = words.next().unwrap().parse().unwrap();
        let _ = words.next();
        let from: usize = words.next().unwrap().parse().unwrap();
        let _ = words.next();
        let to: usize = words.next().unwrap().parse().unwrap();

        Ok(MoveInstruction::new(amount, from, to,))
    }
}
fn parse_instructions(s: &str) -> Vec<MoveInstruction> {
    s.as_bytes().lines().map(|line| MoveInstruction::from_str(&line.unwrap()).unwrap()).collect()
}


//fn parse_input(input: &str) -> {
//    todo!()
//}

#[cfg(test)]
mod tests_day_05 {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    const INPUT_STACKS: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
    const INPUT_MOVES: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    fn stacks() -> CargoStacks {
        CargoStacks {
            stacks: vec![
                vec!['Z', 'N'],
                vec!['N', 'C', 'D'],
                vec!['P'],
            ],
        }
    }
    fn moves() -> Vec<MoveInstruction> {
        vec![
            mv!(move 1 from 2 to 1),
            mv!(move 3 from 1 to 3),
            mv!(move 2 from 2 to 1),
            mv!(move 1 from 1 to 2),
        ]
    }

    #[test]
    fn move_instruction_macro() {
        assert_eq!(mv!(move 3 from 1 to 3), MoveInstruction::new(3, 1, 3));
    }
    #[test]
    fn move_instruction_from_str() {
        assert_eq!(mv!(move 3 from 1 to 3), MoveInstruction::from_str("move 3 from 1 to 3").unwrap());
    }
    #[test]
    fn parse_move_instrucion_list() {
        assert_eq!(parse_instructions(INPUT_MOVES), moves());
    }
}

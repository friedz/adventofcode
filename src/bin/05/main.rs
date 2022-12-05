
use std::{
    io::BufRead,
    str::FromStr,
};
use simple_error::SimpleError;

#[derive(Debug, Eq, PartialEq, Clone)]
struct CargoStacks {
    stacks: Vec<Vec<char>>,
}

impl CargoStacks {
    fn move_crates(&mut self, mv: &MoveInstruction) {
        for _ in 0..mv.ammount() {
            let c = self.stacks[mv.from() - 1].pop().unwrap();
            self.stacks[mv.to() - 1].push(c);
        }
    }
    fn tops(&self) -> String {
        self.stacks.iter().fold(String::new(), |mut res, stack| {
            res.push(*stack.last().unwrap());
            res
        })
    }
}
impl FromStr for CargoStacks {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.as_bytes().lines().collect::<Vec<_>>().into_iter().rev();
        let stacks = lines.next().unwrap().unwrap().chars().fold(
            Vec::new(), |mut stacks, chr| {
                match chr {
                    ' ' => stacks,
                    _ => {
                        stacks.push(Vec::<char>::new());
                        stacks
                    }
                }
            }
        );
        let stacks = lines.fold(stacks, |mut stacks, line| {
            let line = line.unwrap();
            for i in 0..stacks.len() {
                let index = i*3 + 1 + i;
                let c: char = line.as_bytes()[index] as char;
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
            stacks
        });
        Ok(CargoStacks {
            stacks: stacks,
        })
    }
}

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
#[cfg(test)]
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

fn parse_input(input: &str) -> (CargoStacks, Vec<MoveInstruction>) {
    let mut parts = input.split("\n\n");
    let cargo_stacks = CargoStacks::from_str(parts.next().unwrap()).unwrap();
    let move_instructions = parse_instructions(parts.next().unwrap());
    (cargo_stacks, move_instructions)
}

fn run_all_moves(cargo_stacks: CargoStacks, mvs: Vec<MoveInstruction>) -> CargoStacks {
    mvs.iter().fold(cargo_stacks, |mut cs, mv| {
        cs.move_crates(mv);
        cs
    })
}

fn main() {
    let input = include_str!("input.txt");
    let (cs, mvs) = parse_input(input);
    let cs = run_all_moves(cs, mvs);
    println!("Part 1: {}", cs.tops());
}

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

    fn example_stacks() -> [CargoStacks; 5] {
        [
            CargoStacks {stacks: vec![vec!['Z', 'N'],vec!['M', 'C', 'D'],vec!['P'],],},
            CargoStacks {stacks: vec![vec!['Z', 'N', 'D'],vec!['M', 'C'],vec!['P'],],},
            CargoStacks {stacks: vec![vec![],vec!['M', 'C'],vec!['P', 'D', 'N', 'Z'],],},
            CargoStacks {stacks: vec![vec!['C', 'M'],vec![],vec!['P', 'D', 'N', 'Z'],],},
            CargoStacks {stacks: vec![vec!['C'],vec!['M'],vec!['P', 'D', 'N', 'Z'],],},
        ]
    }
    fn example_moves() -> Vec<MoveInstruction> {
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
        assert_eq!(mv!(move 13 from 10 to 12), MoveInstruction::new(13, 10, 12));
    }
    #[test]
    fn move_instruction_from_str() {
        assert_eq!(mv!(move 3 from 1 to 3), MoveInstruction::from_str("move 3 from 1 to 3").unwrap());
    }
    #[test]
    fn parse_move_instrucion_list() {
        assert_eq!(parse_instructions(INPUT_MOVES), example_moves());
    }
    #[test]
    fn read_cargo_stacks() {
        assert_eq!(CargoStacks::from_str(INPUT_STACKS).unwrap(), example_stacks()[0]);
    }
    #[test]
    fn stack_tops() {
        assert_eq!(example_stacks()[0].tops(), "NDP");
    }
    #[test]
    fn move_stacks() {
        let mut cs = example_stacks()[0].clone();
        let m = example_moves();
        for i in 0..m.len() {
            cs.move_crates(&m[i]);
            assert_eq!(cs, example_stacks()[i + 1]);
        }
    }
    #[test]
    fn parse_full_example() {
        let (crate_stacks, move_instructions) = parse_input(INPUT);
        assert_eq!(crate_stacks, example_stacks()[0]);
        assert_eq!(move_instructions, example_moves());
    }
    #[test]
    fn run_all_moves_on_example() {
        let moves = example_moves();
        let stacks = example_stacks()[0].clone();

        let res_stacks = run_all_moves(stacks, moves);
        assert_eq!(res_stacks, *example_stacks().last().unwrap());
        assert_eq!(res_stacks.tops(), "CMZ");
    }
}

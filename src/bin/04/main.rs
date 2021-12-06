
use std::{
    cmp,
    error::Error,
    fmt,
    num::ParseIntError,
    ops::Deref,
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Marked {
    Unmarked(u32),
    Marked(u32),
}

impl Marked {
    fn mark(self) -> Marked {
        match self {
            Marked::Marked(n) => Marked::Marked(n),
            Marked::Unmarked(n) => Marked::Marked(n),
        }
    }
}

impl fmt::Display for Marked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Marked::Marked(n) => f.pad(&format!("({})", n)),
            Marked::Unmarked(n) => f.pad(&format!(" {} ", n)),
        }
    }
}

impl FromStr for Marked {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = u32::from_str(s)?;
        Ok(Marked::Unmarked(u))
    }
}


impl PartialEq<u32> for Marked {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Marked::Marked(n) => n == other,
            Marked::Unmarked(n) => n == other,
        }
    }
}

impl Deref for Marked {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        match self {
            Marked::Marked(n) => &n,
            Marked::Unmarked(n) => &n,
        }
    }
}

const SIZE: usize = 5;

#[derive(Debug, Clone, Copy)]
struct Board {
    finished: bool,
    last: u32,
    field: [Marked; SIZE * SIZE],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max = self.field.iter().fold(0, |max, entry| cmp::max(max, **entry));
        let len = format!("{}", max).len() + 2;
        for (i, entry) in self.field.iter().enumerate() {
            write!(f, "{:>len$} ", entry, len=len)?;
            if i % SIZE == SIZE - 1 && i < SIZE * SIZE - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl FromStr for Board {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Board::empty();
        for (i, v) in s.split_whitespace().enumerate() {
            b.field[i] = Marked::from_str(v)?;
        }
        Ok(b)
    }
}
enum Check {
    Nothing,
    Checked,
    Won,
}
impl Board {
    fn empty() -> Board {
        Board {
            finished: false,
            last: 0,
            field: [Marked::Unmarked(0); SIZE * SIZE],
        }
    }
    fn score(&self) -> u32 {
        self.field.iter().fold(0, |sum, x| {
            sum + match x {
                Marked::Marked(_) => 0,
                Marked::Unmarked(n) => *n,
            }
        })
    }
    fn last(&self) -> u32 {
        self.last
    }
    fn result(&self) -> u32 {
        self.score() * self.last()
    }
    fn done(&self) -> bool {
        self.finished
    }
    fn won(&self, (x, y): (usize, usize)) -> bool {
        if self.finished {
            return true;
        }
        let (mut row, mut col) = (true, true);
        for i in 0..SIZE {
            col = match (self.field[Self::idx((i, y))], col) {
                (_, false) => false,
                (Marked::Unmarked(_), _) => false,
                _ => true,
            };
            row = match (self.field[Self::idx((x, i))], row) {
                (_, false) => false,
                (Marked::Unmarked(_), _) => false,
                _ => true,
            };
            if !row && !col {
                return false;
            }
        }
        true
    }
    fn pos_from_idx(i: usize) -> (usize, usize) {
        (i % SIZE, i / SIZE)
    }
    fn idx((x, y): (usize, usize)) -> usize {
        y*SIZE + x
    }
    fn check_number(&mut self, next: u32) -> Check {
        for i in 0..SIZE * SIZE {
            let entry = self.field[i];
            if entry == next {
                self.field[i] = entry.mark();
                self.last = next;
                if self.won(Self::pos_from_idx(i)) {
                    self.finished = true;
                    return Check::Won;
                } else {
                    return Check::Checked;
                }
            }
        }
        Check::Nothing
    }
}

fn parse_input(s: &str) -> Result<(Vec<u32>, Vec<Board>), ParseIntError> {
    let mut boards = Vec::new();
    let mut numbers = Vec::new();
    for (i, inp) in s.split("\n\n").enumerate() {
        if i == 0 {
            for v in inp.split(",").map(|x| {
                u32::from_str(x)
            }) {
                numbers.push(v?);
            }
        } else {
            boards.push(Board::from_str(inp)?);
        }
    }
    Ok((numbers, boards))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (input, mut boards) = parse_input(&include_str!("input.txt"))?;

    let mut solutions = [None, None];
    for n in input {
        for idx in 0..boards.len() {
            if boards[idx].done() {
                continue;
            }
            //let board = boards[idx];
            //println!("{}\n", boards[idx]);
            match boards[idx].check_number(n) {
                Check::Won => {
                    match solutions[0] {
                        None => solutions[0] = Some(boards[idx]),
                        _ => {},
                    }
                    solutions[1] = Some(boards[idx]);
                },
                _ => continue,
            }
        }
    }
    match &solutions[0] {
        Some(first) => {
            println!("the winning board is:");
            println!("{}", first);
            println!("score: {}, last: {} => {}", first.score(), first.last(), first.result());
        },
        None => {},
    }
    println!("");
    match &solutions[1] {
       Some(last) => {
            println!("the last board is:");
            println!("{}", last);
            println!("score: {}, last: {} => {}", last.score(), last.last(), last.result());
        },
        None => {},
    }
    Ok(())
}

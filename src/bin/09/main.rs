
use std::{
    io::{
        Error,
        ErrorKind,
        BufRead,
    },
    ops::Deref,
    str::FromStr,
};
use im::hashset::HashSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}
impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        let mut parts = s.split(' ');
        let idnt = parts.next();
        let num: i32 = parts.next().unwrap().parse().unwrap();
        Ok(match idnt {
            Some("U") => Move::Up(num),
            Some("D") => Move::Down(num),
            Some("L") => Move::Left(num),
            Some("R") => Move::Right(num),
            _ => { return Err(Error::new(ErrorKind::Other, "thers no number")); },
        })
    }
}
impl Deref for Move {
    type Target = i32;

    fn deref(&self) -> &i32 {
        match self {
            Move::Up(num) => &num,
            Move::Down(num) => &num,
            Move::Left(num) => &num,
            Move::Right(num) => &num,
        }
    }
}
fn parse_moves(input: &str) -> Vec<Move> {
    input.as_bytes().lines().map(|line| {
        let line = line.unwrap();
        Move::from_str(line.as_str()).unwrap()
    }).collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Rope {
    tail: Vec<(i32, i32)>,
    path: HashSet<(i32, i32)>,
}
impl Rope {
    fn new(len: usize) -> Rope {
        Rope {
            tail: vec![(0, 0); len],
            path: HashSet::new(),
        }
    }
    fn from_parts(tail: Vec<(i32, i32)>, set: &[(i32, i32)]) -> Rope {
        Rope {
            tail: tail,
            path: HashSet::from(set),
        }
    }
    fn mv(&mut self, mv: &Move) {
        for _ in 0..**mv {
            match mv {
                Move::Up(_) => self.tail[0].1 += 1,
                Move::Down(_) => self.tail[0].1 -= 1,
                Move::Right(_) => self.tail[0].0 += 1,
                Move::Left(_) => self.tail[0].0 -= 1,
            }
            for i in 1..self.tail.len() {
                match (self.tail[i - 1].0 - self.tail[i].0, self.tail[i - 1].1 - self.tail[i].1) {
                    (0 | 1, 0 | 1) => {}
                    (diff, 0) => {
                        self.tail[i].0 += diff + if 0 < diff { -1 } else { 1 };
                    },
                    (0, diff) => {
                        self.tail[i].1 += diff + if 0 < diff { -1 } else { 1 };
                    },
                    (diff_x, diff_y) if 1 < diff_x.abs() || 1 < diff_y.abs() => {
                        self.tail[i].0 += diff_x + if 1 == diff_x || -1 == diff_x { 0 } else if 0 < diff_x { -1 } else { 1 };
                        self.tail[i].1 += diff_y + if 1 == diff_y || -1 == diff_y { 0 } else if 0 < diff_y { -1 } else { 1 };
                    },
                    _ => { },
                }
                self.path.insert(*self.tail.last().unwrap());
            }
        }
    }
    fn path_len(&self) -> usize {
        self.path.len()
    }
}

fn part1(moves: &Vec<Move>) -> usize {
    moves.iter().fold(Rope::new(2), |mut rope, mv| {
        rope.mv(mv);
        rope
    }).path_len()
}

fn part2(moves: &Vec<Move>) -> usize {
  moves.iter().fold(Rope::new(10), |mut rope, mv| {
        rope.mv(mv);
        rope
    }).path_len()
}

fn main() {
    let input = include_str!("input.txt");
    let moves = parse_moves(&input);
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
}

#[cfg(test)]
mod tests_day_09 {
    use super::{
        *,
        Move::*,
    };

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    fn parsed_data() -> Vec<Move> {
        vec![
            Right(4),
            Up(4),
            Left(3),
            Down(1),
            Right(4),
            Down(1),
            Left(5),
            Right(2),
        ]
    }

    #[test]
    fn read_a_move() {
        assert_eq!(Move::from_str("U 4").unwrap(), Up(4));
        assert_eq!(Move::from_str("D 1").unwrap(), Down(1));
        assert_eq!(Move::from_str("L 5").unwrap(), Left(5));
        assert_eq!(Move::from_str("R 4").unwrap(), Right(4));
    }
    #[test]
    fn read_moves() {
        assert_eq!(parse_moves(INPUT), parsed_data());
    }
    #[test]
    fn move_a_step() {
        let mut rope = Rope::new(2);
        rope.mv(&Up(4));
        assert_eq!(rope, Rope::from_parts(vec![(0, 4), (0, 3)], &[(0, 0), (0, 1), (0, 2), (0, 3)]));
        let mut rope = Rope::new(2);
        rope.mv(&Down(4));
        assert_eq!(rope, Rope::from_parts(vec![(0, -4), (0, -3)], &[(0, 0), (0, -1), (0, -2), (0, -3)]));
        let mut rope = Rope::new(2);
        rope.mv(&Left(4));
        assert_eq!(rope, Rope::from_parts(vec![(-4, 0), (-3, 0)], &[(0, 0), (-1, 0), (-2, 0), (-3, 0)]));
        let mut rope = Rope::new(2);
        rope.mv(&Right(4));
        assert_eq!(rope, Rope::from_parts(vec![(4, 0), (3, 0)], &[(0, 0), (1, 0), (2, 0), (3, 0)]));
        let mut rope = Rope::new(2);
        rope.mv(&Right(1));
        rope.mv(&Up(2));
        assert_eq!(rope, Rope::from_parts(vec![(1, 2), (1, 1)], &[(0, 0), (1, 1)]));
        let mut rope = Rope::new(2);
        rope.mv(&Up(2));
        rope.mv(&Right(1));
        assert_eq!(rope, Rope::from_parts(vec![(1, 2), (0, 1)], &[(0, 0), (0, 1)]));
        let mut rope = Rope::new(2);
        rope.mv(&Up(2));
        rope.mv(&Down(1));
        assert_eq!(rope, Rope::from_parts(vec![(0, 1), (0, 1)], &[(0, 0), (0, 1)]));
        rope.mv(&Down(1));
        assert_eq!(rope, Rope::from_parts(vec![(0, 0), (0, 1)], &[(0, 0), (0, 1)]));
    }
    #[test]
    fn exapmple_part1_manual() {
        let mut path = Vec::new();
        let mut rope = Rope::new(2);
        rope.mv(&Right(4));
        path.push((0, 0));
        path.push((1, 0));
        path.push((2, 0));
        path.push((3, 0));
        assert_eq!(rope, Rope::from_parts(vec![(4, 0), (3, 0)], &path[..]));

        rope.mv(&Up(4));
        path.push((4, 1));
        path.push((4, 2));
        path.push((4, 3));
        assert_eq!(rope, Rope::from_parts(vec![(4, 4), (4, 3)], &path[..]));

        rope.mv(&Left(3));
        path.push((3, 4));
        path.push((2, 4));
        assert_eq!(rope, Rope::from_parts(vec![(1, 4), (2, 4)], &path[..]));

        rope.mv(&Down(1));
        assert_eq!(rope, Rope::from_parts(vec![(1, 3), (2, 4)], &path[..]));

        rope.mv(&Right(4));
        path.push((3, 3));
        path.push((4, 3));
        assert_eq!(rope, Rope::from_parts(vec![(5, 3), (4, 3)], &path[..]));

        rope.mv(&Down(1));
        assert_eq!(rope, Rope::from_parts(vec![(5, 2), (4, 3)], &path[..]));

        rope.mv(&Left(5));
        path.push((3, 2));
        path.push((2, 2));
        path.push((1, 2));
        assert_eq!(rope, Rope::from_parts(vec![(0, 2), (1, 2)], &path[..]));

        rope.mv(&Right(2));
        assert_eq!(rope, Rope::from_parts(vec![(2, 2), (1, 2)], &path[..]));
    }
    #[test]
    fn example_part1() {
        let moves = parse_moves(INPUT);
        assert_eq!(moves, parsed_data());

        let rope = moves.iter().fold(Rope::new(2), |mut rope, mv| {
            rope.mv(mv);
            rope
        });

        let path = [
            (2, 4), (3, 4),
            (3, 3), (4, 3),
            (1, 2), (2, 2), (3, 2), (4, 2),
            (4, 1),
            (0, 0), (1, 0), (2, 0), (3, 0),
        ];
        assert_eq!(rope, Rope::from_parts(vec![(2, 2), (1, 2)], &path[..]));
        assert_eq!(rope.path_len(), 13);
    }
    #[test]
    fn full_part1() {
        let moves = parsed_data();
        assert_eq!(part1(&moves), 13);
    }
    #[test]
    fn full_part2() {
        let moves = parse_moves(INPUT2);
        assert_eq!(part2(&moves), 36);
    }
}

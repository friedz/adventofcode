
use std::{
    iter::Cycle,
    vec::IntoIter,
    collections::HashSet,
    cmp::max,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::many0,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}
fn parse_direction(s: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("<"), |_| Direction::Left),
        map(tag(">"), |_| Direction::Right)
    ))(s)
}
fn parse_winds(s: &str) -> IResult<&str, Vec<Direction>> {
    many0(parse_direction)(s)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Shape {
    /// ####
    Line,

    /// .#.
    /// ###
    /// .#.
    Plus,

    /// ..#
    /// ..#
    /// ###
    Corner,

    /// #
    /// #
    /// #
    /// #
    Column,

    /// ##
    /// ##
    Block,
}
impl Shape {
    fn hit_boxes(&self) -> Vec<(i64, i64)> {
        match self {
            Shape::Line => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Shape::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1),(1, 2)],
            Shape::Corner => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Shape::Column => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Shape::Block => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
}
struct Rock {
    shape: Shape,
    x: i64,
    y: i64,
}
impl Rock {
    fn new(shape: Shape, (x, y): (i64, i64)) -> Rock {
        Rock {
            shape: shape,
            x: x,
            y: y,
        }
    }
    fn hit_boxes(&self) -> Vec<(i64, i64)> {
        self.shape.hit_boxes().into_iter().map(|(x, y)| {
            (x + self.x, y + self.y)
        }).collect()
    }
    fn down(&mut self) {
        self.y -= 1;
    }
    fn up(&mut self) {
        self.y += 1;
    }
    fn move_side(&mut self, direction: Direction) {
        self.x += match direction {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
    fn move_back(&mut self, direction: Direction) {
        self.x += match direction {
            Direction::Left => 1,
            Direction::Right => -1,
        }
    }
}

struct Chamber {
    rocks: Cycle<IntoIter<Shape>>,
    winds: Cycle<IntoIter<Direction>>,
    spawn_point: (i64, i64),
    pile: HashSet<(i64, i64)>,
}
impl Chamber {
    fn new(winds: Vec<Direction>) -> Chamber {
        Chamber {
            rocks: vec![Shape::Line, Shape::Plus, Shape::Corner, Shape::Column, Shape::Block].into_iter().cycle(),
            winds: winds.into_iter().cycle(),
            spawn_point: (2, 3),
            pile: HashSet::new(),
        }
    }
    fn pile_height(&self) -> i64 {
        self.pile.iter().fold(0, |height, (_, y)| {
            max(height, *y)
        }) + 1
    }
    fn colide_rock(&self, rock: &Rock) -> bool {
        for r in rock.hit_boxes() {
            if 0 > r.1 || 0 > r.0 || 7 <= r.0
                || self.pile.contains(&r) {
                    return true;
            }
        }
        false
    }
    fn drop_rock(&mut self) {
        let rock = self.rocks.next().unwrap();
        let mut rock = Rock::new(rock, self.spawn_point);
        loop {
            // moved by air
            let wind = self.winds.next().unwrap();
            rock.move_side(wind);
            if self.colide_rock(&rock) {
                rock.move_back(wind);
            }
            // moved down
            rock.down();
            if self.colide_rock(&rock) {
                rock.up();
                for r in rock.hit_boxes() {
                    self.pile.insert(r);
                }
                self.spawn_point.1 = self.pile_height() + 3;
                return;
            }
        }
    }
    fn drop_n_rocks(&mut self, n: i64) -> i64 {
        for _ in 0..n {
            self.drop_rock();
        }
        self.pile_height()
    }
}

fn main() {
    let input = include_str!("input17.txt");
    let (_, winds) = parse_winds(input).unwrap();
    let mut c = Chamber::new(winds);
    println!("Part 1: {}", c.drop_n_rocks(2022));
}


#[cfg(test)]
mod tests_day_17 {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    fn example_data() -> Vec<Direction> {
        vec![Direction::Right, Direction::Right, Direction::Right, Direction::Left, Direction::Left, Direction::Right, Direction::Left, Direction::Right, Direction::Right, Direction::Left, Direction::Left, Direction::Left, Direction::Right, Direction::Right, Direction::Left, Direction::Right, Direction::Right, Direction::Right, Direction::Left, Direction::Left, Direction::Left, Direction::Right, Direction::Right, Direction::Right, Direction::Left, Direction::Left, Direction::Left, Direction::Right, Direction::Left, Direction::Left, Direction::Left, Direction::Right, Direction::Right, Direction::Left, Direction::Right, Direction::Right, Direction::Left, Direction::Left, Direction::Right, Direction::Right]
    }

    #[test]
    fn example_part1() {
        let (_, winds) = parse_winds(INPUT).unwrap();
        let mut c = Chamber::new(winds);
        assert_eq!(c.drop_n_rocks(2022), 3068);
    }
    #[test]
    fn read_direction_list() {
        assert_eq!(
            parse_winds(INPUT),
            Ok(("", example_data()))
        );
    }
    #[test]
    fn read_single_direction() {
        assert_eq!(parse_direction("<"), Ok(("", Direction::Left)));
        assert_eq!(parse_direction(">"), Ok(("", Direction::Right)));
        assert_eq!(parse_direction(">foo"), Ok(("foo", Direction::Right)));
    }
}

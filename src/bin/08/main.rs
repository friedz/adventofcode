
use std::{
    cmp,
    io::{
        self,
        BufRead,
    },
    str::FromStr,
};

macro_rules! check {
    ($data:expr, $tree:expr, $range:expr, $y:ident) => {
        {
            let mut visible = true;
            for ix in $range {
                if $data[$y][ix] >= $tree {
                    visible = false;
                    break;
                }
            }
            visible
        }
    };
    ($data:expr, $tree:expr, $x:ident, $range:expr) => {
        {
            let mut visible = true;
            for iy in $range {
                if $data[iy][$x] >= $tree {
                    visible = false;
                    break;
                }
            }
            visible
        }
    };
}
macro_rules! view {
    ($data:expr, $tree:expr, $x:ident, $range:expr) => {
        {
            let mut count = 0;
            for iy in $range {
                count += 1;
                if $data[iy][$x] >= $tree {
                    break;
                }
            }
            count
        }
    };
    ($data:expr, $tree:expr, $range:expr, $y:ident) => {
        {
            let mut count = 0;
            for ix in $range {
                count += 1;
                if $data[$y][ix] >= $tree {
                    break;
                }
            }
            count
        }
    };
}
#[derive(Debug, Eq, PartialEq)]
struct Grid {
    data: Vec<Vec<u8>>,
}
impl Grid {
    fn height(&self) -> usize {
        if 0 < self.data.len() {
            self.data[0].len()
        } else {
            0
        }
    }
    fn width(&self) -> usize {
        self.data.len()
    }
    fn visible_tree(&self, x: usize, y: usize) -> bool {
        if 0 == x || 0 == y || self.height() == y + 1 || self.width() == x + 1 {
            return true;
        }
        let tree = self.data[y][x];
        check!(self.data, tree, 0..x, y)
            || check!(self.data, tree, x+1..self.width(), y)
            || check!(self.data, tree, x, 0..y)
            || check!(self.data, tree, x, y+1..self.height())
    }
    fn visible(&self) -> u32 {
        (0..self.height()).fold(0, |sum, y| {
            (0..self.width()).fold(sum, |sum, x| {
                if self.visible_tree(x, y) {
                    sum + 1
                } else {
                    sum
                }
            })
        })
    }
    fn view_score(&self, x: usize, y: usize) -> u32 {
        let tree = self.data[y][x];
        view!(self.data, tree, (0..x).rev(), y)
            * view!(self.data, tree, x+1..self.width(), y)
            * view!(self.data, tree, x, (0..y).rev())
            * view!(self.data, tree, x, y+1..self.height())
    }
    fn best_view(&self) -> u32 {
        (0..self.height()).fold(0, |view, y| {
            (0..self.width()).fold(view, |view, x| {
                cmp::max(self.view_score(x, y), view)
            })
        })
    }
}
impl FromStr for Grid {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Grid, Self::Err> {
        Ok(Grid {
            data: s.as_bytes().lines().try_fold(Vec::new(), |mut data, line| {
                let line = line?;
                data.push(line.chars().fold(Vec::new(), |mut row, c| {
                    row.push(c.to_digit(10).unwrap() as u8);
                    row
                }));
                Ok::<_, Self::Err>(data)
            })?,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    let data = Grid::from_str(&input).unwrap();
    println!("Part 1: {}", data.visible());
    println!("Part 2: {}", data.best_view());
}

#[cfg(test)]
mod tests_day_08 {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";
    fn example_data() -> Grid {
        Grid {
            data: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        }
    }

    #[test]
    fn parsed_input() {
        assert_eq!(Grid::from_str(INPUT).unwrap(), example_data());
    }
    #[test]
    fn check_size() {
        assert_eq!(example_data().width(), 5);
        assert_eq!(example_data().height(), 5);
    }
    #[test]
    fn check_visible() {
        let data = example_data();
        for x in 0..data.width() {
            assert!(data.visible_tree(x, 0));
            assert!(data.visible_tree(x, data.height() - 1));
        }
        for y in 0..data.height() {
            assert!(data.visible_tree(0, y));
            assert!(data.visible_tree(data.width() - 1, y));
        }
        assert!(data.visible_tree(1, 1));
        assert!(data.visible_tree(1, 2));
        assert!(!data.visible_tree(1, 3));
        assert!(data.visible_tree(2, 1));
        assert!(!data.visible_tree(2, 2));
        assert!(data.visible_tree(2, 3));
        assert!(!data.visible_tree(3, 1));
        assert!(data.visible_tree(3, 2));
        assert!(!data.visible_tree(3, 3));
    }
    #[test]
    fn count_visible() {
        assert_eq!(example_data().visible(), 21);
    }
    #[test]
    fn scenic_tree() {
        let data = example_data();
        for x in 0..data.width() {
            assert_eq!(data.view_score(x, 0), 0);
            assert_eq!(data.view_score(x, data.height() - 1), 0);
        }
        for y in 0..data.height() {
            assert_eq!(data.view_score(0, y), 0);
            assert_eq!(data.view_score(data.width() - 1, y), 0);
        }
        assert_eq!(data.view_score(2, 1), 4);
        assert_eq!(data.view_score(2, 3), 8);

    }
    #[test]
    fn best_view_score() {
        let data = example_data();
        assert_eq!(data.best_view(), 8);
    }
}


#![feature(int_abs_diff)]

use std::{
    collections::HashMap,
    cmp::{
        max,
        min,
    },
    error::Error,
    num::ParseIntError,
    str::FromStr,
};

const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(',').into_iter().map(|x| usize::from_str(x.trim()));
        Ok(Point {
            x: vals.next().unwrap()?,
            y: vals.next().unwrap()?,
        })
    }
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split("->").into_iter().map(|x| Point::from_str(x.trim()));
        Ok(LineSegment {
            start: vals.next().unwrap()?,
            end: vals.next().unwrap()?,
        })
    }
}

impl LineSegment {
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn horizontal_or_vertical(&self) -> bool {
        self.horizontal() || self.vertical()
    }
    fn points(&self) -> Vec<Point> {
        if self.horizontal() {
            (min(self.start.x, self.end.x)..=max(self.start.x, self.end.x))
                .map(|x| Point::new(x, self.start.y)).collect()
        } else if self.vertical() {
            (min(self.start.y, self.end.y)..=max(self.start.y, self.end.y))
                .map(|y| Point::new(self.start.x, y)).collect()
        } else {
            let x_step = if self.start.x < self.end.x {
                |x| x + 1
            } else {
                |x| x - 1
            };
            let y_step = if self.start.y < self.end.y {
                |y| y + 1
            } else {
                |y| y - 1
            };
            let (mut res, _) = (0..self.start.x.abs_diff(self.end.x)).into_iter()
                .fold((Vec::new(), (self.start.x, self.start.y)), |(acc, (x, y)), _| {
                let mut acc = acc;
                acc.push(Point::new(x, y));
                (acc, (x_step(x), y_step(y)))
            });
            res.push(Point::new(self.end.x, self.end.y));
            res
        }
    }
}

fn build_map(map: HashMap<Point, usize>, line: &LineSegment) -> HashMap<Point, usize> {
    let mut map = map;
    for p in line.points() {
        match map.get_mut(&p) {
            Some(v) => {
                *v += 1;
            },
            None => {
                map.insert(p, 1);
            },
        }
    }
    map
}

fn main() -> Result<(), Box<dyn Error>> {
    //let arr = TEST_INPUT.lines()
    let arr = include_str!("input.txt").lines()
        .fold(Ok(Vec::new()), |arr: Result<Vec<LineSegment>, Box<dyn Error>>, line| {
            let mut arr = arr?;
            let ls = LineSegment::from_str(line)?;
            arr.push(ls);
            Ok(arr)
        })?;
    let cross_points = arr.iter().filter(|l| l.horizontal_or_vertical())
        .fold(HashMap::new(), |map, line| build_map(map, line));

    let num = cross_points.iter().fold(0, |sum, (_, v)| {
        if *v > 1 {
            sum + 1
        } else {
            sum
        }
    });
    println!("{}", num);

    let cross_points = arr.iter()//.filter(|l| l.horizontal_or_vertical())
        .fold(HashMap::new(), |map, line| build_map(map, line));

    let num = cross_points.iter().fold(0, |sum, (_, v)| {
        if *v > 1 {
            sum + 1
        } else {
            sum
        }
    });
    println!("{}", num);

    Ok(())
}


use std::{
    cmp::{
        Ordering,
        PartialOrd,
    },
    error::Error,
    ops::{
        Deref,
        Index,
        IndexMut,
    },
    str::FromStr,
};
use simple_error::SimpleError;


const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    depth: u8,
    lowest: Option<bool>,
    basin: Option<usize>,
}

impl Point {
    fn from_char(c: &char) -> Result<Point, SimpleError> {
        match c.to_digit(10) {
            Some(d) => Ok(Point {
                depth: d as u8,
                lowest: None,
                basin: None,
            }),
            None => Err(SimpleError::new(format!("{} is not a number!", c))),
        }
    }
    fn set_low(&mut self) {
        self.lowest = Some(true);
    }
    fn set_not_low(&mut self) {
        self.lowest = Some(false);
    }
    fn risk(&self) -> u32 {
        if Some(true) == self.lowest {
            1 + self.depth as u32
        } else {
            0
        }
    }
    fn is_lowest(&self) -> Option<bool> {
        self.lowest
    }
    fn depth(&self) -> u8 {
        self.depth
    }
}


#[derive(Debug)]
struct HeightMap {
    height: usize,
    width: usize,
    map: Vec<Point>,
}

impl FromStr for HeightMap{
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, (width, _), height) = s.chars().try_fold((Vec::new(), (0, 0), 0), |(map, (line_len, line_count), height), c| {
            if '\n' == c {
                if line_len != 0 && line_len != line_count {
                    return Err(SimpleError::new("line lenght not consitent"));
                }
                Ok((map, (line_count, 0), height + 1))
            } else {
                let mut map = map;
                map.push(Point::from_char(&c)?);
                Ok((map, (line_len, line_count + 1), height))
            }
        })?;
        Ok(HeightMap {
            height: height + if s.chars().last() == Some('\n') { 0 } else { 1 },
            width: width,
            map: map,
        })
    }
}

impl Index<(usize, usize)> for HeightMap {
    type Output = Point;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.map[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for HeightMap {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.map[x + y * self.width]
    }
}

impl HeightMap {
    fn check_low_points(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x > 0 && self[(x, y)] >= self[(x - 1, y)])
                || (y > 0 && self[(x, y)] >= self[(x, y - 1)])
                || (x < self.width - 1 && self[(x, y)] >= self[(x + 1, y)])
                || (y < self.height - 1 && self[(x, y)] >= self[(x, y + 1)]) {
                    self[(x, y)].set_not_low();
                } else {
                    self[(x, y)].set_low();
                }
            }
        }
        self
    }
    fn risk_level(&self) -> Result<u32, SimpleError> {
        self.map.clone().into_iter().try_fold(0, |risk, p| {
            Ok(risk + p.risk())
        })
    }
    fn ansi_print(&self) {
        for (i, p) in self.map.clone().into_iter().enumerate() {
            if  Some(true) == p.is_lowest() {
                print!("\x1b[30;47;1m{}\x1b[0m", p.depth());
            } else {
                print!("{}", p.depth());
            }
            if (i + 1) % self.width == 0 {
                println!("");
            }
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let input = TEST_INPUT;
    //let input = include_str!("input.txt");
    let mut map = HeightMap::from_str(input)?;
    map.check_low_points();
    //map.ansi_print();
    let risk = map.risk_level()?;
    println!("{}", risk);

    Ok(())
}

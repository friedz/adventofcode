
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Point {
    Low(u8),
    NotLow(u8),
    NotChecked(u8),
}

impl Point {
    fn from_char(c: &char) -> Result<Point, SimpleError> {
        match c.to_digit(10) {
            Some(d) => Ok(Point::NotChecked(d as u8)),
            None => Err(SimpleError::new(format!("{} is not a number!", c))),
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if **self < **other {
            Ordering::Less
        } else if **self > **other {
            Ordering::Greater
        } else if self == other {
            Ordering::Equal
        } else {
            return None
        })
    }
}

impl Deref for Point {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            Point::Low(d) => &d,
            Point::NotLow(d) => &d,
            Point::NotChecked(d) => &d,
        }
    }
}

impl Point {
    fn set_low(&mut self) {
        match self {
            Point::NotChecked(d) => *self = Point::Low(*d),
            Point::NotLow(d) => *self = Point::Low(*d),
            _ => {},
        }
    }
    fn set_not_low(&mut self) {
        match self {
            Point::NotChecked(d) => *self = Point::NotLow(*d),
            Point::Low(d) => *self = Point::NotLow(*d),
            _ => {},
        }
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
            //if c.is_whitespace() {
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
            Ok(risk + match p {
                Point::Low(d) => 1 + d as u32,
                Point::NotLow(_) => 0 as u32,
                Point::NotChecked(_) => return Err(SimpleError::new("Heights not jet evaluated")),
            })
        })
    }
    fn ansi_print(&self) {
        for (i, p) in self.map.clone().into_iter().enumerate() {
            match p {
                Point::Low(v) => print!("\x1b[30;47;1m{}\x1b[0m", v),
                Point::NotLow(v) | Point::NotChecked(v) => print!("{}", v),
            }
            if (i + 1) % self.width == 0 {
                println!("");
            }
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    //let input = TEST_INPUT;
    let input = include_str!("input.txt");
    let mut map = HeightMap::from_str(input)?;
    map.check_low_points();
    //map.ansi_print();
    let risk = map.risk_level()?;
    println!("{}", risk);

    Ok(())
}

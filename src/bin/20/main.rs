
use std::{
    ops::Index,
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

#[derive(Debug, Clone)]
struct TrenchMap {
    width: usize,
    height: usize,
    outside: bool,
    map: Vec<bool>,
}
impl FromStr for TrenchMap {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().try_fold(
            TrenchMap { width: 0, height: 0, outside: false, map: Vec::new() },
            |mut tm, c| {
                match c {
                    '.' => tm.map.push(false),
                    '#' => tm.map.push(true),
                    '\n' => tm.height += 1,
                    e => return Err(simple_error!("{:?}, is not a valid trench map charackter", e)),
                }
                if tm.height == 0 {
                    tm.width += 1;
                }
                Ok(tm)
        })
    }
}
impl Index<(i64, i64)> for TrenchMap {
    type Output = bool;
    fn index(&self, (x, y): (i64, i64)) -> &Self::Output {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            &self.outside
        } else {
            let x = x as usize;
            let y = y as usize;
            &self.map[x + y * self.width]
        }
    }
}
impl TrenchMap {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn print(&self) {
        for y in -1..(self.height as i64 + 1) {
            for x in -1..(self.width as i64 + 1) {
                if self[(x, y)] {
                    print!("█");
                } else {
                    print!("░");
                }
            }
            println!("");
        }
    }
    fn lookup_number(&self, x: i64, y: i64) -> usize {
        let mut res = if self[(x - 1, y - 1)] { 1 } else { 0 };
        res = res * 2 + if self[(x, y - 1)] { 1 } else { 0 };
        res = res * 2 + if self[(x + 1, y - 1)] { 1 } else { 0 };
        res = res * 2 + if self[(x - 1, y)] { 1 } else { 0 };
        res = res * 2 + if self[(x, y)] { 1 } else { 0 };
        res = res * 2 + if self[(x + 1, y)] { 1 } else { 0 };
        res = res * 2 + if self[(x - 1, y + 1)] { 1 } else { 0 };
        res = res * 2 + if self[(x, y + 1)] { 1 } else { 0 };
        res * 2 + if self[(x + 1, y + 1)] { 1 } else { 0 }
    }
    fn number_of_light_pixel(&self) -> usize {
        self.map.iter().fold(0, |sum, p| {
            if *p {
                sum + 1
            } else {
                sum
            }
        })
    }
    fn outside(&self) -> bool {
        self.outside
    }
}

#[derive(Debug)]
struct EnhancementAlgorithm {
    lookup: Vec<bool>,
}
impl FromStr for EnhancementAlgorithm {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EnhancementAlgorithm {
            lookup: s.chars().try_fold(Vec::new(), |mut lookup, c| {
                lookup.push(match c {
                    '.' => false,
                    '#' => true,
                    e => return Err(simple_error!("{:?} not in [#.]", e)),
                });
                Ok(lookup)
            })?,
        })
    }
}
impl EnhancementAlgorithm {
    fn enhance(&self, tm: &TrenchMap) -> TrenchMap {
        let mut new_map = Vec::new();
        for y in -1..(tm.height() as i64 + 1) {
            for x in -1..(tm.width() as i64 + 1) {
                new_map.push(self.lookup[tm.lookup_number(x, y)]);
            }
        }
        TrenchMap {
            width: tm.width() + 2,
            height: tm.height() + 2,
            outside: if tm.outside() {
                self.lookup[self.lookup.len() - 1]
            } else {
                self.lookup[0]
            },
            map: new_map,
        }
    }
}


fn main() -> SimpleResult<()> {
    //let mut input = include_str!("example_input.txt").split("\n\n");
    let mut input = include_str!("input.txt").split("\n\n");
    let ea = EnhancementAlgorithm::from_str(input.next().ok_or(simple_error!("no first line"))?)?;
    let mut tm = TrenchMap::from_str(input.next().ok_or(simple_error!("no trench map"))?)?;
    let mut tm1 = tm.clone();
    for i in 1..=2 {
        //tm1.print();
        tm1 = ea.enhance(&tm1);
        println!("{}: {}", i, tm1.number_of_light_pixel());
    }
    println!("{}", tm1.number_of_light_pixel());
    for i in 1..=50 {
        //tm.print();
        tm = ea.enhance(&tm);
        println!("{}: {}", i, tm.number_of_light_pixel());
    }
    println!("{}", tm.number_of_light_pixel());

    Ok(())
}


use std::{
    collections::BinaryHeap,
    cmp::Ordering,
    error::Error,
    ops::{
        Index,
        IndexMut,
    },
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

#[derive(Debug)]
struct RiskMap {
    height: usize,
    width: usize,
    map: Vec<usize>,
}
fn from_char(c: &char) -> SimpleResult<usize> {
    Ok(match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        e => return Err(simple_error!("{} is not a digit!", e)),
    })
}

impl FromStr for RiskMap {
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
                map.push(from_char(&c)?);
                Ok((map, (line_len, line_count + 1), height))
            }
        })?;
        Ok(RiskMap {
            height: height + if s.chars().last() == Some('\n') { 0 } else { 1 },
            width: width,
            map: map,
        })
    }
}
impl Index<(usize, usize)> for RiskMap {
    type Output = usize;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.map[x + y * self.width]
    }
}
impl IndexMut<(usize, usize)> for RiskMap {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.map[x + y * self.width]
    }
}
impl RiskMap {
    fn ansi_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self[(x, y)]);
            }
            println!("");
        }
    }
    fn neighbors(&self, x: usize, y: usize) -> Vec<((usize, usize), usize)> {
        let mut res = Vec::new();
        if x > 0 {
            res.push(((x - 1, y), self[(x - 1, y)]));
        }
        if y > 0 {
            res.push(((x , y - 1), self[(x, y - 1)]));
        }
        if x < self.width - 1 {
            res.push(((x + 1, y), self[(x + 1, y)]));
        }
        if y < self.height - 1 {
            res.push(((x, y + 1), self[(x, y + 1)]));
        }
        res
    }
    fn scale(&self, n: usize) -> Self {
        let mut scaled = RiskMap {
            height: self.height * n,
            width: self.width * n,
            map: Vec::new(),
        };
        for y in 0..scaled.height {
            let y_local = y % self.height;
            for x in 0..scaled.width {
                let x_local = x % self.width;
                let scale = x/self.width + y/self.height;
                let next = self[(x_local, y_local)] + scale;
                scaled.map.push((next-1)%9 + 1)
            }
        }
        scaled
    }
    fn max(width: usize, height: usize) -> RiskMap {
        RiskMap {
            width: width,
            height: height,
            map: vec![usize::MAX; width*height],
        }
    }
    fn score(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut dist = RiskMap::max(self.width, self.height);
        dist[(0,0)] = 0;
        heap.push(State::new(0, 0, 0));

        while let Some(state) = heap.pop() {
            if state.is_position(self.width - 1, self.height - 1) {
                return Some(state.risk());
            }
            if state.risk() > dist[state.position()] {
                continue;
            }
            for (next, cost) in self.neighbors(state.x(), state.y()) {
                let next = State::from_pos(next, state.risk() + usize::from(cost));
                if next.risk() < dist[next.position()] {
                    dist[next.position()] = next.risk();
                    heap.push(next);

                }
            }
        }
        None
    }
}
#[derive(Eq, PartialEq, Debug)]
struct State {
    pos: (usize, usize),
    //last: Option<(usize, usize)>,
    risk: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    fn is_position(&self, x: usize, y: usize) -> bool {
        (x, y) == self.pos
    }
    fn new(x: usize, y: usize, risk: usize) -> Self {
        State {
            pos: (x, y),
            risk: risk,
        }
    }
    fn from_pos((x, y): (usize, usize), risk: usize) -> Self {
        Self::new(x, y, risk)
    }
    fn risk(&self) -> usize {
        self.risk
    }
    fn position(&self) -> (usize, usize) {
        self.pos
    }
    fn x(&self) -> usize {
        self.pos.0
    }
    fn y(&self) -> usize {
        self.pos.1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = include_str!("example_input.txt");
    let input = include_str!("input.txt");
    let map = RiskMap::from_str(input)?;

    //map.ansi_print();
    //println!("{:?}", map);
    println!("\n{}", map.score().ok_or(simple_error!("couldnt find a path through the cave!"))?);

    let scaled = map.scale(5);
    //scaled.ansi_print();
    println!("{}", scaled.score().ok_or(simple_error!("couldnt find a path through the cave!"))?);

    Ok(())
}

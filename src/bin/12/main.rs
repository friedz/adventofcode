
use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    str::FromStr,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct NaviState {
    len: u32,
    pos: (usize, usize),
}
impl NaviState {
    fn new(length: u32, pos: (usize, usize)) -> NaviState {
        NaviState {
            len: length,
            pos: pos,
        }
    }
    fn next(&self, pos: (usize, usize)) -> NaviState {
        NaviState {
            len: self.len + 1,
            pos: pos,
        }
    }
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
    fn x(&self) -> usize {
        self.pos.0
    }
    fn y(&self) -> usize {
        self.pos.1
    }
    fn len(&self) -> u32 {
        self.len
    }
}
impl Ord for NaviState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.len.cmp(&self.len)
            .then_with(|| self.pos.0.cmp(&other.pos.0))
            .then_with(|| self.pos.1.cmp(&other.pos.1))
    }
}
impl PartialOrd for NaviState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct HeightMap {
    map: Vec<Vec<i32>>,
    best_signal: (usize, usize),
    start: (usize, usize),
}
impl HeightMap {
    fn get(&self, (x, y): (usize, usize)) -> Option<&i32> {
        self.map.get(y)?.get(x)
    }
    fn posible_steps(&self, nvs: &NaviState) -> Vec<NaviState> {
        let level = match self.get((nvs.x(), nvs.y())) {
            Some(h) => h,
            None => { return Vec::new(); },
        };
        let mut res = Vec::new();
        if nvs.x() > 0 {
            match self.get((nvs.x() - 1, nvs.y())) {
                Some(h) => if h - level <= 1 {
                    res.push(nvs.next((nvs.x() - 1, nvs.y())));
                },
                None => { },
            }
        }
        match self.get((nvs.x() + 1, nvs.y())) {
            Some(h) => if h - level <= 1 {
                res.push(nvs.next((nvs.x() + 1, nvs.y())));
            },
            None => { },
        }
        if nvs.y() > 0 {
            match self.get((nvs.x(), nvs.y() - 1)) {
                Some(h) => if h - level <= 1 {
                    res.push(nvs.next((nvs.x(), nvs.y() - 1)));
                },
                None => { },
            }
        }
        match self.get((nvs.x(), nvs.y() + 1)) {
            Some(h) => if h - level <= 1 {
                res.push(nvs.next((nvs.x(), nvs.y() + 1)));
            },
            None => { },
        }
        res
    }
    fn width(&self) -> usize {
        match self.map.get(0) {
            Some(line) => line.len(),
            None => 0,
        }
    }
    fn height(&self) -> usize {
        self.map.len()
    }
    fn shortest_path(&self) -> Option<u32> {
        let mut dist: Vec<Vec<u32>> = (0..self.height()).map(
            |_| (0..self.width()).map(
                |_| u32::MAX
            ).collect()
        ).collect();
        let mut heap = BinaryHeap::new();
        dist[self.start.1][self.start.0] = 0;
        heap.push(NaviState::new(0, self.start));
        while let Some(nvs) = heap.pop() {
            if nvs.pos() == self.best_signal { return Some(nvs.len); }
            if nvs.len() > dist[nvs.y()][nvs.x()] { continue; }

            for step in self.posible_steps(&nvs) {
                if  step.len() < dist[step.y()][step.x()] {
                    dist[step.y()][step.x()] = step.len();
                    heap.push(step);
                }
            }
        }
        None
    }
}
impl Default for HeightMap {
    fn default() -> HeightMap {
        HeightMap {
            map: Vec::new(),
            best_signal: (0, 0),
            start: (0, 0),
        }
    }
}
impl FromStr for HeightMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, map) = s.as_bytes().iter().fold(((0, 0), HeightMap::default()), |((x, y), mut map), c| {
            if x == 0 {
                map.map.push(Vec::new());
            }
            match c {
                0x0a /* \n */ => {
                    ((0, y + 1), map)
                },
                0x53 /* S */ => {
                    map.start = (x, y);
                    map.map[y].push(0);
                    ((x + 1, y), map)
                },
                0x45 /* E */ => {
                    map.best_signal = (x, y);
                    map.map[y].push(25);
                    ((x + 1, y), map)
                },
                h => {
                    map.map[y].push((h - 'a' as u8).into());
                    ((x + 1, y), map)
                }
            }
        });
        Ok(map)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let map: HeightMap = input.parse().unwrap();
    match map.shortest_path() {
        Some(path) => println!("Part 1: {}", path),
        None => println!("Part 1 dosn't have a solution!"),
    }
}

#[cfg(test)]
mod tests_day_12 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");
    fn parsed_map() -> HeightMap {
        HeightMap {
            map: vec![
                vec![0, 0, 1, 16, 15, 14, 13, 12],
                vec![0, 1, 2, 17, 24, 23, 23, 11],
                vec![0, 2, 2, 18, 25, 25, 23, 10],
                vec![0, 2, 2, 19, 20, 21, 22, 9],
                vec![0, 1, 3, 4, 5, 6, 7, 8],
            ],
            best_signal: (5, 2),
            start: (0, 0),
        }
    }

    #[test]
    fn least_steps_up_the_hill() {
        let hm = parsed_map();
        assert_eq!(hm.shortest_path(), Some(31));
    }
    #[test]
    fn read_map() {
        assert_eq!(INPUT.parse::<HeightMap>(), Ok(parsed_map()));
    }
    #[test]
    fn map_height() {
        assert_eq!(parsed_map().height(), 5);
    }
    #[test]
    fn map_width() {
        assert_eq!(parsed_map().width(), 8);
    }
    #[test]
    fn map_get() {
        let map = parsed_map();
        assert_eq!(map.get((0, 0)), Some(&0));
        assert_eq!(map.get((5, 2)), Some(&25));
        assert_eq!(map.get((0, 4)), Some(&0));
        assert_eq!(map.get((7, 0)), Some(&12));
        assert_eq!(map.get((7, 4)), Some(&8));
        assert_eq!(map.get((8, 2)), None);
        assert_eq!(map.get((5, 5)), None);
    }
}

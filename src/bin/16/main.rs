
use std::{
    cmp::max,
    collections::{
        HashMap,
        HashSet,
    },
};
use nom::{
    branch::alt,
    bytes::complete::{
        tag,
        take,
    },
    character::complete,
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct Valve {
    rate: i32,
    distance_map: HashMap<String, i32>,
}
impl Valve {
    fn new(rate: i32, neighbors: &[&str]) -> Valve {
        Valve {
            rate: rate,
            distance_map: neighbors.iter().fold(HashMap::new(), |mut map, s| {
                map.insert(s.to_string(), 1);
                map
            })
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
struct CaveMap {
    map: HashMap<(String, String), i32>,
    valves: HashMap<String, i32>,
}
macro_rules! t {
    ($a:expr, $b:expr) => {
        ($a.clone(), $b.clone())
    };
}
#[derive(Debug, Eq, PartialEq)]
struct Path {
    last: String,
    open: HashSet<String>,
}
impl Path {
    fn new() -> Path {
        Path {
            last: "AA".to_string(),
            open: HashSet::new(),
        }
    }
    fn add(&self, next: String) -> Path {
        let mut open = self.open.clone();
        open.insert(next.clone());
        Path {
            last: next,
            open: open,
        }
    }
}
impl CaveMap {
    fn full_routing(&mut self) {
        for k in self.valves.keys().cloned() {
            for i in self.valves.keys().cloned() {
                for j in self.valves.keys().cloned() {
                    if let Some(dist_ik) = self.map.get(&t!(i, k)).copied() {
                        if let Some(dist_kj) = self.map.get(&t!(k, j)).copied() {
                            match self.map.get_mut(&t!(i, j)) {
                                Some(dist_ij) => {
                                    if *dist_ij > dist_ik + dist_kj {
                                        *dist_ij = dist_ik + dist_kj;
                                    }
                                },
                                None => {
                                    self.map.insert((i.clone(), j.clone()), dist_ik + dist_kj);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
    fn search_max_flow(&self, path: Path, time_left: i32) -> i32 {
        let mut flow = 0;
        for (vv, f) in self.valves.iter() {
            if 0 == *f {
                continue;
            }
            if path.open.contains(vv) {
                continue;
            }
            let time_left = time_left - (self.map.get(&t!(path.last, vv)).unwrap() + 1);

            if 0 > time_left {
                continue;
            }
            let path = path.add(vv.to_string());
            flow = max(flow, f*time_left + self.search_max_flow(path, time_left));
        }
        flow
    }
}

fn parse_valve(s: &str) -> IResult<&str, (String, Valve)> {
    let (s, _) = tag("Valve ")(s)?;
    let (s, name) = take(2usize)(s)?;
    let (s, _) = tag(" has flow rate=")(s)?;
    let (s, rate) = complete::i32(s)?;
    let (s, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(s)?;
    let (s, list) = separated_list0(tag(", "), take(2usize))(s)?;
    Ok((s, (name.to_owned(), Valve {
        rate: rate,
        distance_map: list.into_iter().fold(HashMap::new(), |mut map, vv| {
            map.insert(vv.to_string(), 1);
            map
        }),
    })))
}
fn parse_cave_map(s: &str) -> IResult<&str, CaveMap> {
    let (s, list) = separated_list0(complete::newline, parse_valve)(s)?;
    Ok((s, CaveMap {
        map: list.iter().fold(HashMap::new(), |map, (name, valve)| {
            valve.distance_map.keys().fold(map, |mut map, neigh| {
                map.insert((name.clone(), neigh.clone()), 1);
                map
            })
        }),
        valves: list.into_iter().map(|(name, valve)| { (name, valve.rate) }).collect(),
    }))
}

fn main() {
    let input = include_str!("input.txt");
    //let input = include_str!("example.txt");
    let (_, mut cave) = parse_cave_map(input).unwrap();
    cave.full_routing();
    let part1 = cave.search_max_flow(Path::new(), 30);
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests_day_16 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");

    #[test]
    fn example_part1() {
        let (_, mut cave) = parse_cave_map(INPUT).unwrap();
        cave.full_routing();
        let path = Path::new();
        let part1 = cave.search_max_flow(path, 30);
        assert_eq!(part1, 1651);
    }
    #[test]
    fn read_valve() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        assert_eq!(
            parse_valve(input),
            Ok(("", ("AA".to_string(), Valve::new(0, &["DD", "II", "BB"]))))
        );
        let input = "Valve HH has flow rate=22; tunnel leads to valve GG";
        assert_eq!(
            parse_valve(input),
            Ok(("", ("HH".to_string(), Valve::new(22, &["GG"]))))
        );
    }
}

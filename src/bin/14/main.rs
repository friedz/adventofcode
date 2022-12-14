
use std::{
    cmp::{
        max,
        min,
    },
    collections::HashMap,
};
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use self::Material::{
    Sand,
    Rock,
};

fn parse_rock_path(s: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list0(
        tag(" -> "),
        separated_pair(complete::u64, tag(","), complete::u64)
    )(s)
}
fn parse_all_rocks(s: &str) -> IResult<&str, Vec<Vec<(u64, u64)>>> {
    separated_list0(complete::newline, parse_rock_path)(s)
}

#[derive(Debug, Eq, PartialEq)]
enum Material {
    Rock,
    Sand,
}
#[derive(Debug, Eq, PartialEq)]
struct CaveMap {
    sand_source: (u64, u64),
    min_pos: (u64, u64),
    max_pos: (u64, u64),
    map: HashMap<(u64, u64), Material>,
}
impl CaveMap {
    fn drop_sand(&mut self) -> Option<(u64, u64)> {
        let (mut next_x, mut next_y) = self.sand_source;
        while next_y <= self.max_pos.1 {
            if None == self.map.get(&(next_x, next_y + 1)) {
                next_y += 1;
            } else if None == self.map.get(&(next_x - 1, next_y + 1)) {
                next_x -= 1;
                next_y += 1;
            } else if None == self.map.get(&(next_x + 1, next_y + 1)) {
                next_x += 1;
                next_y += 1;
            } else {
                self.map.insert((next_x, next_y), Sand);
                return Some((next_x, next_y));
            }
        }
        None
    }
    fn fill_with_sand(&mut self) -> u64 {
        let mut count = 0;
        while let Some(_) = self.drop_sand() {
            count += 1;
        }
        count
    }
}
impl From<Vec<Vec<(u64, u64)>>> for CaveMap {
    fn from(cave_paths: Vec<Vec<(u64, u64)>>) -> CaveMap {
        let (map, min_x, max_pos) = cave_paths.into_iter().fold(
            (HashMap::new(), u64::MAX, (u64::MIN, u64::MIN)),
            |(map, min_x, max_pos), path| {
                path.windows(2).fold((map, min_x, max_pos),
                |(map, minx, (maxx, maxy)), ends_of_edge| {
                    let (start_x, start_y) = ends_of_edge[0];
                    let (end_x, end_y) = ends_of_edge[1];
                    (if start_x == end_x {
                        (min(start_y, end_y)..=max(start_y, end_y)).fold(map, |mut map, y| {
                            map.insert((start_x, y), Rock);
                            map
                        })
                    } else if start_y == end_y {
                        (min(start_x, end_x)..=max(start_x, end_x)).fold(map, |mut map, x| {
                            map.insert((x, start_y), Rock);
                            map
                        })
                    } else {
                        map
                    },
                    min(minx, min(start_x, end_x)),
                    (max(maxx, max(start_x, end_x)), max(maxy, max(start_y, end_y))))
                })
            });
        CaveMap {
            sand_source: (500, 0),
            min_pos: (min_x, 0),
            max_pos: max_pos,
            map: map,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (_, rock_paths) = parse_all_rocks(input).unwrap();
    let mut cave = CaveMap::from(rock_paths);
    println!("Part 1: {}", cave.fill_with_sand());
}

#[cfg(test)]
mod tests_day_14 {
    use super::{
        *,
        //Material::*,
        Material::Rock,
    };

    macro_rules! rock_path {
        ($($x:literal,$y:literal)->+) => {
            vec![$(($x,$y)),+]
        };
    }

    const INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    fn parsed_example() -> Vec<Vec<(u64, u64)>> {
        vec![
            rock_path![498,4 -> 498,6 -> 496,6],
            rock_path![503,4 -> 502,4 -> 502,9 -> 494,9]
        ]
    }
    fn example_cave_map() -> CaveMap {
        CaveMap {
            sand_source: (500, 0),
            min_pos: (494, 0),
            max_pos: (503, 9),
            map: HashMap::from([
                ((498,4),Rock),((498,5),Rock),((498,6),Rock),((497,6),Rock),((496,6),Rock),
                ((503,4),Rock),((502,4),Rock),((502,5),Rock),((502,6),Rock),((502,7),Rock),
                ((502,8),Rock),((502,9),Rock),((501,9),Rock),((500,9),Rock),((499,9),Rock),
                ((498,9),Rock),((497,9),Rock),((496,9),Rock),((495,9),Rock),((494,9),Rock)
            ]),
        }
    }

    #[test]
    fn example_part1() {
        assert_eq!(example_cave_map().fill_with_sand(), 24);
    }
    #[test]
    fn drop_one_grain_of_sand() {
        let mut cave = example_cave_map();
        cave.map.insert((500,8), Sand);
        let mut new_map = example_cave_map();
        assert_eq!(new_map.drop_sand(), Some((500, 8)));
        assert_eq!(new_map, cave);
    }
    #[test]
    fn get_cave_map_from_rock_paths() {
        assert_eq!(
            CaveMap::from(parsed_example()),
            example_cave_map()
        );
    }
    #[test]
    fn read_one_rock_path() {
        assert_eq!(
            parse_rock_path("498,4 -> 498,6 -> 496,6"),
            Ok(("", vec![(498, 4), (498,6), (496, 6)]))
        );
        assert_eq!(
            parse_rock_path("503,4 -> 502,4 -> 502,9 -> 494,9"),
            Ok(("", vec![(503, 4), (502, 4), (502, 9), (494, 9)]))
        );
    }
    #[test]
    fn read_all_rock_paths() {
        assert_eq!(
            parse_all_rocks(INPUT),
            Ok(("", parsed_example()))
        );
    }
}

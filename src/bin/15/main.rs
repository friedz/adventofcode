
use std::cmp::{
    max,
    min,
};
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    position: (i64, i64),
    closest_beacon: (i64, i64),
}
impl Sensor {
    fn new(pos: (i64, i64), beacon: (i64, i64)) -> Sensor {
        Sensor {
            position: pos,
            closest_beacon: beacon,
        }
    }
    fn position(&self) -> (i64, i64) {
        self.position
    }
    fn beacon(&self) -> (i64, i64) {
        self.closest_beacon
    }
    fn distance(&self, point: (i64, i64)) -> i64 {
        (self.position.0 - point.0).abs()
        + (self.position.1 - point.1).abs()
    }
    fn beacon_distance(&self) -> i64 {
        self.distance(self.closest_beacon)
    }
    /// gets the first position to the riht in the same line
    /// wich is farther away from the sensor than its closest beacon
    fn next_posible_beacon_in_line(&self, (x, y): (i64, i64)) -> (i64, i64) {
        if self.could_be_beacon((x, y)) {
            (x, y)
        } else {
            let y_to_sensor = (self.position().1 - y).abs();
            (self.position().0 + self.beacon_distance() - y_to_sensor + 1, y)
        }
    }
    fn could_be_beacon(&self, pos: (i64, i64)) -> bool {
        self.distance(pos) > self.beacon_distance()
    }
}
fn sensor_coverage(sensors: &Vec<Sensor>) -> ((i64, i64), (i64, i64)) {
    sensors.iter().fold(((i64::MAX, i64::MAX), (i64::MIN, i64::MIN)), |((xmin,ymin),(xmax,ymax)), sensor| {
        (
            (
                min(xmin, sensor.position().0 - sensor.beacon_distance()),
                min(ymin, sensor.position().1 - sensor.beacon_distance()),
            ), (
                max(xmax, sensor.position().0 + sensor.beacon_distance()),
                max(ymax, sensor.position().1 + sensor.beacon_distance()),
            )
        )
    })
}
fn count_distress_beacon_free_pos_in_line(sensors: &Vec<Sensor>, line: i64) -> u32 {
    let ((xmin, _), (xmax, _)) = sensor_coverage(sensors);
    (xmin..=xmax).fold(0, |count, x| {
        let mut plus = false;
        for s in sensors {
            if (x, line) == s.position() || (x, line) == s.beacon() {
                return count;
            } else if !s.could_be_beacon((x, line)) {
                plus = true;
            }
        }
        if plus {
            count + 1
        } else {
            count
        }
    })
}
fn find_distress_beacon(sensors: &Vec<Sensor>, (xmax, ymax): (i64, i64)) -> Option<(i64, i64)> {
    let (mut x, mut y) = (0, 0);
    'coords: while y <= ymax {
        for s in sensors {
            if !s.could_be_beacon((x, y)) {
                (x, y) = s.next_posible_beacon_in_line((x, y));
                if x > xmax {
                    (x, y) = (0, y + 1);
                }
                continue 'coords;
            }
        }
        return Some((x, y))
    }
    None
}

fn parse_sensor(s: &str) -> IResult<&str, Sensor> {
    let (s, _) = tag("Sensor at x=")(s)?;
    let (s, sens_x) = complete::i64(s)?;
    let (s, _) = tag(", y=")(s)?;
    let (s, sens_y) = complete::i64(s)?;
    let (s, _) = tag(": closest beacon is at x=")(s)?;
    let (s, beac_x) = complete::i64(s)?;
    let (s, _) = tag(", y=")(s)?;
    let (s, beac_y) = complete::i64(s)?;
    Ok((s, Sensor::new((sens_x, sens_y), (beac_x, beac_y))))
}
fn parse_input(s: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list0(complete::newline, parse_sensor)(s)
}

fn main() {
    let input = include_str!("input.txt");
    let (_, data) = parse_input(input).unwrap();
    let part1 = count_distress_beacon_free_pos_in_line(&data, 2_000_000);
    println!("Part 1: {}", part1);
    let (x, y) = find_distress_beacon(&data, (4_000_000, 4_000_000)).unwrap();
    println!("Part 2: {}", x * 4_000_000 + y);
}

#[cfg(test)]
mod tests_day_15 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");
    fn example_data() -> Vec<Sensor> {
        vec![
            Sensor::new(( 2, 18), (-2, 15)),
            Sensor::new(( 9, 16), (10, 16)),
            Sensor::new((13,  2), (15,  3)),
            Sensor::new((12, 14), (10, 16)),
            Sensor::new((10, 20), (10, 16)),
            Sensor::new((14, 17), (10, 16)),
            Sensor::new(( 8,  7), ( 2, 10)),
            Sensor::new(( 2,  0), ( 2, 10)),
            Sensor::new(( 0, 11), ( 2, 10)),
            Sensor::new((20, 14), (25, 17)),
            Sensor::new((17, 20), (21, 22)),
            Sensor::new((16,  7), (15,  3)),
            Sensor::new((14,  3), (15,  3)),
            Sensor::new((20,  1), (15,  3)),
        ]
    }

    #[test]
    fn full_example_part2() {
        let (_, data) = parse_input(INPUT).unwrap();
        let res = find_distress_beacon(&data, (20, 20));
        assert_eq!(res, Some((14, 11)));
        let (x, y) = res.unwrap();
        assert_eq!(x * 4_000_000 + y, 56_000_011);
    }
    #[test]
    fn example_part2() {
        let res = find_distress_beacon(&example_data(), (20, 20));
        assert_eq!(res, Some((14, 11)));
        let (x, y) = res.unwrap();
        assert_eq!(x * 4_000_000 + y, 56_000_011);
    }
    #[test]
    fn full_example_part1() {
        let (_, data) = parse_input(INPUT).unwrap();
        let part1 = count_distress_beacon_free_pos_in_line(&data, 10);
        assert_eq!(part1, 26);
    }
    #[test]
    fn example_part1() {
        assert_eq!(
            count_distress_beacon_free_pos_in_line(&example_data(), 10),
            26
        );
    }
    #[test]
    fn test_next_posible_beacon_in_line() {
            let sense = Sensor::new((0, 0), (4, 4));
            assert_eq!(sense.next_posible_beacon_in_line((-4, 4)), (5, 4));
    }
    #[test]
    fn test_could_be_beacon() {
        assert!(!Sensor::new((13,  2), (15,  3)).could_be_beacon((15, 3)));
        assert!(Sensor::new((13,  2), (15,  3)).could_be_beacon((2, 10)));
        assert!(Sensor::new((13,  2), (15,  3)).could_be_beacon((-2, 15)));
    }
    #[test]
    fn min_max_cave_limit() {
        assert_eq!(
            sensor_coverage(&example_data()),
            ((-8, -10), (28, 26))
        );
    }
    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input(INPUT).unwrap();
        assert_eq!(data, example_data());
    }
    #[test]
    fn test_parse_sensor() {
        assert_eq!(
            parse_sensor("Sensor at x=2, y=0: closest beacon is at x=2, y=10"),
            Ok(("", Sensor::new((2, 0), (2, 10))))
        );
        assert_eq!(
            parse_sensor("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Ok(("", Sensor::new((2, 18), (-2, 15))))
        );
        assert_eq!(
            parse_sensor("Sensor at x=9, y=16: closest beacon is at x=10, y=16"),
            Ok(("", Sensor::new((9, 16), (10, 16))))
        );
    }
    #[test]
    fn test_beacon_distances() {
        assert_eq!(Sensor::new((0,0),(2,0)).beacon_distance(), 2);
        assert_eq!(Sensor::new((0,0),(-2,0)).beacon_distance(), 2);
        assert_eq!(Sensor::new((0,0),(0,2)).beacon_distance(), 2);
        assert_eq!(Sensor::new((0,0),(0,-2)).beacon_distance(), 2);
        assert_eq!(Sensor::new((2,0),(0,0)).beacon_distance(), 2);
        assert_eq!(Sensor::new((-2,0),(0,0)).beacon_distance(), 2);
        assert_eq!(Sensor::new((0,2),(0,0)).beacon_distance(), 2);
        assert_eq!(Sensor::new((0,-2),(0,0)).beacon_distance(), 2);
        assert_eq!(Sensor::new((0,0),(2,2)).beacon_distance(), 4);
        assert_eq!(Sensor::new((0,0),(2,-2)).beacon_distance(), 4);
        assert_eq!(Sensor::new((0,0),(-2,2)).beacon_distance(), 4);
        assert_eq!(Sensor::new((0,0),(-2,-2)).beacon_distance(), 4);
    }
}

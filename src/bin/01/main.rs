fn calibration_simple_value(line: &str) -> Option<i32> {
    let left = line.find(|c: char| c.is_ascii_digit());
    let right = line.rfind(|c: char| c.is_ascii_digit());
    match (left, right) {
        (None, _) | (_, None) => None,
        (Some(l), Some(r)) => {
            Some((line.as_bytes()[l] - 48) as i32 * 10 + (line.as_bytes()[r] - 48) as i32)
        }
    }
}
static NUMS: [(&'static str, i32); 19] = [
    ("0", 0),
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5),
    ("five", 5),
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
];

fn left_digit(line: &str) -> Option<i32> {
    let (res, _) = NUMS
        .into_iter()
        .fold((None, None), |(res, pos), (pat, val)| {
            match line.find(pat) {
                Some(idx) if idx < pos.unwrap_or(usize::MAX) => (Some(val), Some(idx)),
                _ => (res, pos),
            }
        });
    res
}

fn right_digit(line: &str) -> Option<i32> {
    let (res, _) = NUMS
        .into_iter()
        .fold((None, None), |(res, pos), (pat, val)| {
            match line.rfind(pat) {
                Some(idx) => match pos {
                    Some(p) if idx < p => (res, pos),
                    _ => (Some(val), Some(idx)),
                },
                None => (res, pos),
            }
        });
    res
}

fn calibration_value(line: &str) -> Option<i32> {
    let left = left_digit(line);
    let right = right_digit(line);
    match (left, right) {
        (Some(l), Some(r)) => Some(l * 10 + r),
        _ => None,
    }
}

fn full_part_1(input: &str) -> Option<i32> {
    input.lines().fold(Some(0), |sum, line| match sum {
        Some(sum) => match calibration_simple_value(line) {
            Some(val) => Some(val + sum),
            None => None,
        },
        None => None,
    })
}

fn full_part_2(input: &str) -> Option<i32> {
    input.lines().fold(Some(0), |sum, line| match sum {
        Some(sum) => match calibration_value(line) {
            Some(val) => Some(val + sum),
            None => None,
        },
        None => None,
    })
}

fn main() {
    let input = include_str!("input.txt");
    println!("# Advent of Code 2023 Day 1:");
    println!("Part 1: {}", full_part_1(input).unwrap());
    println!("Part 2: {}", full_part_2(input).unwrap());
}

#[cfg(test)]
mod tests_day_01 {
    use crate::{
        calibration_simple_value,
        calibration_value,
        full_part_1,
        full_part_2,
        left_digit,
        right_digit,
    };

    static EXAMPLE1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    static EXAMPLE1_ARR: [(&'static str, Option<i32>); 4] = [
        ("1abc2", Some(12)),
        ("pqr3stu8vwx", Some(38)),
        ("a1b2c3d4e5f", Some(15)),
        ("treb7uchet", Some(77)),
    ];
    static EXAMPLE2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    static EXAMPLE2_ARR: [(&'static str, Option<i32>); 7] = [
        ("two1nine", Some(29)),
        ("eightwothree", Some(83)),
        ("abcone2threexyz", Some(13)),
        ("xtwone3four", Some(24)),
        ("4nineeightseven2", Some(42)),
        ("zoneight234", Some(14)),
        ("7pqrstsixteen", Some(76)),
    ];

    #[test]
    fn check_left_digit() {
        for (line, res) in EXAMPLE2_ARR {
            let digit = res.unwrap() / 10;
            assert_eq!(left_digit(line), Some(digit));
        }
    }

    #[test]
    fn check_right_digit() {
        for (line, res) in EXAMPLE2_ARR {
            let digit = res.unwrap() % 10;
            assert_eq!(right_digit(line), Some(digit));
        }
    }

    #[test]
    fn check_calibration_simple_value() {
        for (line, res) in EXAMPLE1_ARR {
            assert_eq!(calibration_simple_value(line), res);
        }
    }

    #[test]
    fn check_full_part_1() {
        assert_eq!(full_part_1(EXAMPLE1), Some(142));
    }

    #[test]
    fn check_calibration_value() {
        for (line, res) in EXAMPLE2_ARR {
            assert_eq!(calibration_value(line), res);
        }
    }

    #[test]
    fn check_full_part2() {
        assert_eq!(full_part_2(EXAMPLE2), Some(281));
    }
}

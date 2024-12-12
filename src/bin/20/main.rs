
use nom::{
    character::complete,
    multi::separated_list0,
    IResult,
};

fn parse_input(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(complete::newline, complete::i32)(s)
}
fn shift_value(data: &mut Vec<(i32, bool)>, idx: usize) -> usize {
    let ix = idx as i32;
    let ln = data.len() as i32;
    let mut i = (ix + data[idx].0).rem_euclid(ln) as usize;
    if data[idx].0 < 0 {
        i = if i == 0 { data.len() - 1 } else { i - 1 };
    } else if i < idx /*data[idx].0 + ix >= ln*/ {
        i = if i == data.len() - 1 { 1 } else { i + 1 };
    }
    if data[idx].1 || data[idx].0 == 0 || i == idx {
        data[idx].1 = true;
        return idx + 1;
    }
    if i > idx {
        data[idx].1 = true;
        data[idx..=i].rotate_left(1);
    } else {
        data[idx].1 = true;
        data[i..=idx].rotate_right(1);
    }
    println!("i: {}, idx: {}, data: {:?}", i, idx, data.iter().map(|(i, _)| i).collect::<Vec<_>>());
    idx
}
fn shift_data(data: Vec<i32>) -> Vec<i32> {
    let mut data: Vec<(i32, bool)> = data.into_iter().map(|i| (i, false)).collect();
    let mut idx = 0;
    println!("              data: {:?}", data.iter().map(|(i, _)| i).collect::<Vec<_>>());
    while idx < data.len() {
        idx = shift_value(&mut data, idx);
    }
    data.into_iter().map(|(i, _)| i).collect()
}
fn part1(data: Vec<i32>) -> i32 {
    let data = shift_data(data);
    let idx_0 = data.iter().position(|x| *x == 0).unwrap();
    // 1000th, 2000th, and 3000th
    //data[(idx_0 + 1_000) % data.len()] + data[(idx_0 + 2_000) % data.len()] + data[(idx_0 + 3_000) % data.len()]
    let mut idx_1_000 = (idx_0 + 1_000) % data.len();
    //if idx_1_000 < idx_0 {
    //    idx_1_000 = if idx_1_000 == data.len() - 1 { 1 } else { idx_1_000 + 1 };
    //}
    let mut idx_2_000 = (idx_0 + 2_000) % data.len();
    //if idx_2_000 < idx_0 {
    //    idx_2_000 = if idx_2_000 == data.len() - 1 { 1 } else { idx_2_000 + 1 };
    //}
    let mut idx_3_000 = (idx_0 + 3_000) % data.len();
    //if idx_3_000 < idx_0 {
    //    idx_3_000 = if idx_3_000 == data.len() - 1 { 1 } else { idx_3_000 + 1 };
    //}
    data[idx_1_000] + data[idx_2_000] + data[idx_3_000]
}

fn main() {
    let input = include_str!("input.txt");
    let (_, data) = parse_input(input).unwrap();
    let part1 = part1(data);
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests_day_20 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");
    fn example_data() -> Vec<i32> {
        vec![1, 2, -3, 3, -2, 0, 4]
    }

    #[test]
    fn read_input() {
        assert_eq!(parse_input(INPUT), Ok(("\n", example_data())));
    }
    #[test]
    fn check_shifted_data() {
        assert_eq!(
            shift_data(example_data()),
            vec![1, 2, -3, 4, 0, 3, -2]
        );
    }
    #[test]
    fn check_part1() {
        assert_eq!(part1(example_data()), 3);
    }
}

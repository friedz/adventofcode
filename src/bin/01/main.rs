#![feature(is_sorted)]

use std::{
    io::BufRead,
    mem,
    cmp
};

fn get_input(input: &str) -> Vec<u32> {
    let buf = input.as_bytes();
    let (mut elves, last) = buf.lines().fold((Vec::new(), 0), |(mut arr, food), line| {
        match line.expect("string").parse::<u32>() {
            Ok(kcal) => (arr, food + kcal),
            Err(_) => {
                arr.push(food);
                (arr, 0)
            }
        }
    });
    elves.push(last);
    elves
}

fn max_n(n: usize, arr: &Vec<u32>) -> Vec<u32> {
    arr.iter().fold(vec![0; n], |mut res, tmp| {
        let mut tmp = *tmp;
        for el in res.iter_mut() {
            if tmp > *el {
                mem::swap(&mut tmp, el);
            }
        }
        res
    })
}

fn main() {
    let input = include_str!("input.txt");
    let calorie_list = get_input(&input);

    let maximum_3 = max_n(3, &calorie_list);
    println!("maximumt number of calories caried by a single elve is: {} kcal", maximum_3[0]);
    println!("the 3 most carrying elves carry: {} kcal, {} kcal & {} kcal", maximum_3[0], maximum_3[1], maximum_3[2]);
    println!("wich is together: {} kcal", maximum_3.iter().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input() {
        let data = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        let input = get_input(&data);

        assert_eq!(input.len(), 5);

        assert_eq!(input[0], 6000);
        assert_eq!(input[1], 4000);
        assert_eq!(input[2], 11000);
        assert_eq!(input[3], 24000);
        assert_eq!(input[4], 10000);
    }
    #[test]
    fn max_n_is_sorted() {
        let input = vec![6000, 4000, 11000, 24000, 10000];

        assert!(max_n(4, &input).as_slice().is_sorted_by(|a,b| Some(b.cmp(&a))));
    }
    #[test]
    fn find_maximum() {
        let input = vec![6000, 4000, 11000, 24000, 10000];

        assert_eq!(max_n(1, &input)[0], 24000);
    }
    #[test]
    fn find_max_3() {
        let input = vec![6000, 4000, 11000, 24000, 10000];

        let max_3 = max_n(3, &input);
        assert_eq!(max_3[0], 24000);
        assert_eq!(max_3[1], 11000);
        assert_eq!(max_3[2], 10000);
        assert_eq!(max_3.iter().sum::<u32>(), 45000);
    }
}

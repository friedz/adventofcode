
use std::{
    collections::HashSet,
    io::BufRead,
};

fn type_score(c: u8) -> u32 {
    if 65 <= c && c <= 90 {
        c - 38
    } else if 97 <= c && c <= 122 {
        c - 96
    } else {
        0
    }.into()
}

fn read_rucksacks(data: &str) -> u32{
    data.as_bytes().lines().fold(0, |res, line| {
        let line = line.unwrap();
        let line = line.as_bytes();
        let compartment1: HashSet<_> = HashSet::from_iter(&line[..line.len()/2]);
        let compartment2 = HashSet::from_iter(&line[line.len()/2..]);
        let shared_type = compartment1.intersection(&compartment2).next().unwrap();
        type_score(**shared_type) + res
    })
}

fn main() {
    let input = include_str!("input.txt");
    let output = read_rucksacks(input);
    println!("Part 1: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn whats_in_the_rucksacks() {
        assert_eq!(read_rucksacks(INPUT), 157);
    }
    #[test]
    fn check_type_score() {
        assert_eq!(type_score("a".as_bytes()[0]), 1);
        assert_eq!(type_score("z".as_bytes()[0]), 26);
        assert_eq!(type_score("A".as_bytes()[0]), 27);
        assert_eq!(type_score("Z".as_bytes()[0]), 52);
    }
}

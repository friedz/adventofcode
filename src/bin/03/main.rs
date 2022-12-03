
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

fn read_rucksacks(data: &str) -> (u32, u32) {
    let (part1, part2, _) = data.as_bytes().lines().fold((0, 0, (HashSet::new(), 0)), |(mut part1, mut part2, (mut set, mut count)),  line| {
        let line = line.unwrap();
        let line = line.as_bytes();

        let compartment1 = line[..line.len()/2].into_iter().fold(HashSet::new(), |mut hs, c| {
            hs.insert(c.clone());
            hs
        });
        let compartment2 = line[line.len()/2..].into_iter().fold(HashSet::new(), |mut hs, c| {
            hs.insert(c.clone());
            hs
        });
        let shared_type: u8 = *compartment1.intersection(&compartment2).next().unwrap();
        part1 = type_score(shared_type) + part1;

        if count == 0 {
            set = line.iter().fold(HashSet::new(), |mut s, t| {
                s.insert(t.clone());
                s
            });
        } else {
            set = set.intersection(&line.iter().fold(HashSet::new(), |mut s, t| {
                s.insert(t.clone());
                s
            })).fold(HashSet::new(), |mut s, t| {
                s.insert(t.clone());
                s
            });
        }
        if count == 2 {
            part2 = part2 + type_score(set.into_iter().next().unwrap());
            set = HashSet::new();
        }
        count = (count + 1) % 3;

        (part1, part2, (set, count))
    });
    (part1, part2)
}

fn main() {
    let input = include_str!("input.txt");
    let output = read_rucksacks(input);
    println!("Part 1: {}", output.0);
    println!("Part 2: {}", output.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn whats_in_the_rucksacks() {
        assert_eq!(read_rucksacks(INPUT).0, 157);
        assert_eq!(read_rucksacks(INPUT).1, 70);
    }
    #[test]
    fn check_type_score() {
        assert_eq!(type_score("a".as_bytes()[0]), 1);
        assert_eq!(type_score("z".as_bytes()[0]), 26);
        assert_eq!(type_score("A".as_bytes()[0]), 27);
        assert_eq!(type_score("Z".as_bytes()[0]), 52);
    }
}

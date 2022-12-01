
use std::io::BufRead;

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

fn main() {
    let input = include_str!("input.txt");
    let mut calorie_list = get_input(&input);
    calorie_list.sort();
    let maximum = calorie_list.last().unwrap();
    println!("maximumt number of calories caried by a single elve is: {}", maximum);
    let second = calorie_list[calorie_list.len() - 2];
    let third = calorie_list[calorie_list.len() - 3];
    println!("the 3 most carrying elves carry: {}, {} & {}", maximum, second, third);
    println!("wich is together: {}", maximum + second + third);
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
    fn find_maximum() {
        let mut input = vec![6000, 4000, 11000, 24000, 10000];
        input.sort();
        assert_eq!(*input.last().unwrap(), 24000);
    }
    #[test]
    fn find_max_3() {
        let mut input = vec![6000, 4000, 11000, 24000, 10000];
        input.sort();
        let maximum = *input.last().unwrap();
        let second = input[input.len() - 2];
        let third = input[input.len() - 3];

        assert_eq!(maximum, 24000);
        assert_eq!(second, 11000);
        assert_eq!(third, 10000);

        assert_eq!(maximum + second + third, 45000);
    }
}

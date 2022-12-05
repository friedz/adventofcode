
use std::io::BufRead;

fn read_input(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input.as_bytes().lines().fold(Vec::new(), |mut res, line| {
        let line = line.unwrap();
        let numbers: Vec<u32> = line.split(&[',', '-']).map(|x| x.parse::<u32>().unwrap()).collect();
        res.push(((numbers[0], numbers[1]), (numbers[2], numbers[3])));
        res
    })
}

fn count_total_overlaps(data: &[((u32, u32), (u32, u32))]) -> u32 {
    data.iter().fold(0, |overlaps, ((min_a, max_a), (min_b, max_b))| {
        overlaps + if (min_a <= min_b && max_a >= max_b)
            || (min_b <= min_a && max_b >= max_a) {
                1
            } else {
                0
        }
    })
}

fn count_overlaps(data: &[((u32, u32), (u32, u32))]) -> u32 {
    data.iter().fold(0, |overlaps, ((min_a, max_a), (min_b, max_b))| {
        overlaps + if !(max_a < min_b || min_a > max_b) {
                1
            } else {
                0
        }
    })
}

fn main() {
    let input = include_str!("input.txt");
    let data = read_input(input);

    let overlaps = count_total_overlaps(&data);
    println!("Part 1: {}", overlaps);

    let overlaps = count_overlaps(&data);
    println!("Part 2: {}", overlaps);
}

#[cfg(test)]
mod tests_day_04 {
    use super::*;

    static INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    static DATA: [((u32, u32), (u32, u32)); 6] = [
        ((2, 4), (6, 8)),
        ((2, 3), (4, 5)),
        ((5, 7), (7, 9)),
        ((2, 8), (3, 7)),
        ((6, 6), (4, 6)),
        ((2, 6), (4, 8)),
    ];

    #[test]
    fn parse_input() {
        let data = read_input(INPUT);

        assert_eq!(data.as_slice(), DATA.as_slice());
    }
    #[test]
    fn count_total_number_overlaps() {
        assert_eq!(count_total_overlaps(&DATA), 2);
    }
    #[test]
    fn count_number_overlaps() {
        assert_eq!(count_overlaps(&DATA), 4);
        // second section completely overlaps the first section
        assert_eq!(count_overlaps(&[((2, 3), (1, 4))]), 1);
    }
}

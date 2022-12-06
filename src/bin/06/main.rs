
fn check_marker(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    for i in 0..c.len() - 1 {
        for j in i+1..c.len() {
            if c[i] == c[j] {
                return false;
            }
        }
    }
    true
}
fn find_marker(input: &str, len: usize) -> Option<usize> {
    for i in len..input.len() + 1 {
        if check_marker(&input[i-len..i]) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let input = include_str!("input.txt");
    match find_marker(input, 4) {
        Some(pos) => println!("Part 1: {}", pos),
        None => println!("Part 1 has no solution, weird"),
    }
    match find_marker(input, 14) {
        Some(pos) => println!("Part 2: {}", pos),
        None => println!("Part 2 has no solution, weird"),
    }
}

#[cfg(test)]
mod tests_day_06 {
    use super::*;

    const INPUT: [(&'static str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn check_a_marker() {
        assert!(!check_marker("mjqj"));
        assert!(check_marker("jpqm"));
        assert!(check_marker("qmgbljsphdztnv"));
    }
    #[test]
    fn find_packet_markers() {
        for (input, marker_pos, _) in INPUT {
            assert_eq!(find_marker(input, 4), Some(marker_pos));
        }
    }
    #[test]
    fn find_message_markers() {
        for (input, _, marker_pos) in INPUT {
            assert_eq!(find_marker(input, 14), Some(marker_pos));
        }
    }
}

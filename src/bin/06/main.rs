
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
fn find_packet_marker(input: &str) -> Option<usize> {
    for i in 4..input.len() + 1 {
        if check_marker(&input[i-4..i]) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let input = include_str!("input.txt");
    match find_packet_marker(input) {
        Some(pos) => println!("Part 1: {}", pos),
        None => println!("Part 1 has no solution, weird"),
    }
}

#[cfg(test)]
mod tests_day_06 {
    use super::*;

    const INPUT: [(&'static str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn check_a_marker() {
        assert!(!check_marker("mjqj"));
        assert!(check_marker("jpqm"));
    }
    #[test]
    fn find_the_markers() {
        for (input, marker_pos) in INPUT {
            assert_eq!(find_packet_marker(input), Some(marker_pos));
        }
    }
}


use std::{
    collections::VecDeque,
    cmp::PartialEq,
    fmt::{
        self,
        Debug,
    },
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator,
    character::complete::{
        self,
        newline,
        space0,
        space1,
    },
    sequence::pair,
    multi::separated_list0,
    IResult,
};

struct Monkey {
    items: VecDeque<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    inspection_count: u64,
    test: i64,
    pass_to: (usize, usize),
    safe_multiple: Option<i64>,
}
impl Monkey {
    #[cfg(test)]
    fn new(items: Vec<i64>, op: Box<dyn Fn(i64) -> i64>, test: i64, pass: (usize, usize)) -> Monkey {
        Monkey {
            items: VecDeque::from(items),
            operation: op,
            inspection_count: 0,
            test: test,
            pass_to: pass,
            safe_multiple: None,
        }
    }
    fn turn(&mut self, managing: bool) -> Option<(usize, i64)> {
        match self.items.pop_front() {
            Some(worry) => {
                let worry = (self.operation)(worry);
                let worry = if managing {
                    worry / 3
                } else {
                    worry
                };
                let worry = match self.safe_multiple {
                    Some(n) => worry % n,
                    None => worry,
                };
                self.inspection_count += 1;
                if 0 == worry % self.test {
                    Some((self.pass_to.0, worry))
                } else {
                    Some((self.pass_to.1, worry))
                }
            },
            None => None,
        }
    }
    fn add_item(&mut self, item: i64) {
        self.items.push_back(item);
    }
    fn safe_multiple(&mut self, safe: i64) {
        self.safe_multiple = Some(safe);
    }
}
fn primfactors(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut res = Vec::new();
    let mut i = 2;
    loop {
        if n % i == 0 {
            n = n / i;
            res.push(i);
            if n == 1 {
                return res;
            }
        } else {
            i += 1;
        }
    }
}
impl Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("items", &self.items)
            .field("operation", &"|u64| -> i64")
            .field("inspection_count", &self.inspection_count)
            .field("test", &self.test)
            .field("pass_to", &self.pass_to)
            .field("safe_multiple", &self.safe_multiple)
            .finish()
    }
}
impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: VecDeque::default(),
            operation: Box::new(move |old| old),
            inspection_count: u64::default(),
            test: i64::default(),
            pass_to: (usize::default(), usize::default()),
            safe_multiple: None,
        }
    }
}
impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
            && self.inspection_count == other.inspection_count
            && self.test == other.test
            && self.pass_to == other.pass_to
            && self.safe_multiple == other.safe_multiple
    }
}
fn monkey_number(s: &str) -> IResult<&str, usize> {
    // Monkey 0:
    let (s, _) = tag("Monkey")(s)?;
    let (s, _) = space1(s)?;
    let (s, i) = complete::u64(s)?;
    let (s, _) = tag(":")(s)?;
    Ok((s, i as usize))
}
fn starting_items(s: &str) -> IResult<&str, VecDeque<i64>> {
    //   Starting items: 79, 98
    let (s, _) = space0(s)?;
    let (s, _) = tag("Starting items:")(s)?;
    let (s, _) = space0(s)?;
    let (s, v) = separated_list0(tag(", "), complete::i64)(s)?;
    Ok((s, VecDeque::from(v)))
}
#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mult,
}
#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Num(i64),
}
fn operation(s: &str) -> IResult<&str, Box<dyn Fn(i64) -> i64>> {
    //   Operation: new = old * 19
    let (s, _) = space0(s)?;
    let (s, _) = tag("Operation: new = old")(s)?;
    let (s, _) = space0(s)?;
    let (s, op) = alt((
            combinator::value(Op::Add, tag("+")),
            combinator::value(Op::Mult, tag("*")),
    ))(s)?;
    let (s, _) = space0(s)?;
    let (s, right) = alt((
            combinator::value(Operand::Old, tag("old")),
            combinator::map(complete::i64, |i| Operand::Num(i)),
    ))(s)?;
    Ok((s, match (op, right) {
        (Op::Add, Operand::Old) => Box::new(move |old| old + old),
        (Op::Mult, Operand::Old) => Box::new(move |old| old * old),
        (Op::Add, Operand::Num(i)) => Box::new(move |old| old + i),
        (Op::Mult, Operand::Num(i)) => Box::new(move |old| old * i),
    }))
}
fn test(s: &str) -> IResult<&str, i64> {
    //   Test: divisible by 23
    let (s, _) = space0(s)?;
    let (s, _) = tag("Test: divisible by")(s)?;
    let (s, _) = space0(s)?;
    let (s, i) = complete::i64(s)?;
    Ok((s, i))
}
fn pass_to(s: &str) -> IResult<&str, (usize, usize)> {
    //     If true: throw to monkey 2
    let (s, _) = space0(s)?;
    let (s, _) = tag("If true: throw to monkey")(s)?;
    let (s, _) = space0(s)?;
    let (s, if_true) = complete::i64(s)?;
    let (s, _) = newline(s)?;
    //     If false: throw to monkey 3;"
    let (s, _) = space0(s)?;
    let (s, _) = tag("If false: throw to monkey")(s)?;
    let (s, _) = space0(s)?;
    let (s, if_false) = complete::i64(s)?;
    Ok((s, (if_true as usize, if_false as usize)))
}
fn monkey(s: &str) -> IResult<&str, (usize, Monkey)> {
    // Monkey 0:
    let (s, idx) = monkey_number(s)?;
    let (s, _) = newline(s)?;
    //   Starting items: 79, 98
    let (s, items) = starting_items(s)?;
    let (s, _) = newline(s)?;
    //   Operation: new = old * 19
    let (s, op) = operation(s)?;
    let (s, _) = newline(s)?;
    //   Test: divisible by 23
    let (s, test) = test(s)?;
    let (s, _) = newline(s)?;
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3;"
    let (s, pass) = pass_to(s)?;
    Ok((s, (idx, Monkey {
        items: items,
        operation: op,
        inspection_count: u64::default(),
        test: test,
        pass_to: pass,
        safe_multiple: None,
    })))
}
fn monkey_list(s: &str) -> IResult<&str, Vec<Monkey>> {
     let (s, v) = separated_list0(pair(newline, newline), monkey)(s)?;
     Ok((s, v.into_iter().map(|(_, monk)| monk).collect()))
}
fn merge_factors(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
    let mut res = Vec::new();
    let (mut ai, mut bi) = (0, 0);
    while ai < a.len() && bi < b.len() {
        if a[ai] == b[bi] {
            res.push(a[ai]);
            ai += 1;
            bi += 1;
        } else if a[ai] < b[bi] {
            res.push(a[ai]);
            ai += 1;
        } else {
            res.push(b[bi]);
            bi += 1;
        }
    }
    if ai < a.len() {
        res.extend_from_slice(&a[ai..]);
    }
    if bi < b.len() {
        res.extend_from_slice(&b[bi..]);
    }
    res
}
fn find_safe_multiple(ml: &Vec<Monkey>) -> i64 {
    let mut factors = Vec::new();
    for m in ml.iter().map(|m| primfactors(m.test)) {
        factors = merge_factors(&factors, &m);
    }
    factors.iter().fold(1, |res, a| res * a)
}

fn part1(monkeys: &mut Vec<Monkey>) -> u64 {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((to, item)) = monkeys[i].turn(true) {
                monkeys[to].add_item(item);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}
fn part2(monkeys: &mut Vec<Monkey>) -> u64 {
    let safe = find_safe_multiple(monkeys);
    for m in monkeys.iter_mut() {
        m.safe_multiple(safe);
    }
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some((to, item)) = monkeys[i].turn(false) {
                monkeys[to].add_item(item);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}


fn main() {
    let input = include_str!("input.txt");
    let (_, mut monkeys) = monkey_list(input).unwrap();
    println!("Part 1: {}", part1(&mut monkeys));
    let (_, mut monkeys) = monkey_list(input).unwrap();
    println!("Part 2: {}", part2(&mut monkeys));
}

#[cfg(test)]
mod tests_day_11 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");
    const SINGLE_MONKEY: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
    fn parsed_monkeys() -> Vec<Monkey> {
        vec![
            Monkey::new(
                vec![79, 98],
                Box::new(move |old| old * 19),
                23,
                (2, 3)
            ),
            Monkey::new(
                vec![54, 65, 75, 74],
                Box::new(move |old| old + 6),
                19,
                (2, 0)
            ),
            Monkey::new(
                vec![79, 60, 97],
                Box::new(move |old| old * old),
                13,
                (1, 3)
            ),
            Monkey::new(
                vec![74],
                Box::new(move |old| old + 3),
                17,
                (0, 1)
            ),
        ]
    }

    #[test]
    fn part1_example() {
        let (_, mut monkeys) = monkey_list(INPUT).unwrap();
        assert_eq!(part1(&mut monkeys), 10605);
    }
    #[test]
    fn part2_example() {
        let (_, mut monkeys) = monkey_list(INPUT).unwrap();
        assert_eq!(part2(&mut monkeys), 2713310158);
    }
    #[test]
    fn parse_all_monkeys() {
        let monkeys = parsed_monkeys();
        let (_, read_monkeys) = monkey_list(INPUT).unwrap();
        assert_eq!(monkeys, read_monkeys);
    }
    #[test]
    fn parse_monkey() {
        let (left, (idx, monk)) = monkey(SINGLE_MONKEY).unwrap();
        let correct_monkey = Monkey::new(
            vec![79, 98],
            Box::new(move |worry| worry),
            23,
            (2, 3),
        );
        assert_eq!(idx, 0);
        assert_eq!(monk, correct_monkey);
        for i in 0..20 {
            assert_eq!((monk.operation)(i), i * 19);
        }
        assert_eq!(left, "");
    }
    #[test]
    fn index_for_monkeys() {
        let (s, idx) = monkey_number("Monkey 0:").unwrap();
        assert_eq!(s, "");
        assert_eq!(idx, 0);
        let (s, idx) = monkey_number("Monkey 1:").unwrap();
        assert_eq!(s, "");
        assert_eq!(idx, 1);
        let (s, idx) = monkey_number("Monkey 2:").unwrap();
        assert_eq!(s, "");
        assert_eq!(idx, 2);
        let (s, idx) = monkey_number("Monkey 3:").unwrap();
        assert_eq!(s, "");
        assert_eq!(idx, 3);
    }
    #[test]
    fn parse_items() {
        let (s, items) = starting_items("  Starting items: 79, 98").unwrap();
        assert_eq!(s, "");
        assert_eq!(items, VecDeque::from([79, 98]));
        let (s, items) = starting_items("Starting items: 54, 65, 75, 74").unwrap();
        assert_eq!(s, "");
        assert_eq!(items, VecDeque::from([54, 65, 75, 74]));
        let (s, items) = starting_items("    Starting items: 79, 60, 97").unwrap();
        assert_eq!(s, "");
        assert_eq!(items, VecDeque::from([79, 60, 97]));
        let (s, items) = starting_items("   Starting items: 74").unwrap();
        assert_eq!(s, "");
        assert_eq!(items, VecDeque::from([74]));
    }
    #[test]
    fn read_test() {
        //   Test: divisible by 23
        let (s, t) = test("  Test: divisible by 23").unwrap();
        assert_eq!(s, "");
        assert_eq!(t, 23);
        let (s, t) = test("Test: divisible by 19").unwrap();
        assert_eq!(s, "");
        assert_eq!(t, 19);
        let (s, t) = test(" Test: divisible by 13").unwrap();
        assert_eq!(s, "");
        assert_eq!(t, 13);
        let (s, t) = test("   Test: divisible by 17").unwrap();
        assert_eq!(s, "");
        assert_eq!(t, 17);
    }
    #[test]
    fn pass() {
        let (s, pass) = pass_to("    If true: throw to monkey 2
    If false: throw to monkey 3").unwrap();
        assert_eq!(s, "");
        assert_eq!(pass, (2, 3));
        let (s, pass) = pass_to("If true: throw to monkey 2\nIf false: throw to monkey 0").unwrap();
        assert_eq!(s, "");
        assert_eq!(pass, (2, 0));
        let (s, pass) = pass_to("   If true: throw to monkey 1
      If false: throw to monkey 3").unwrap();
        assert_eq!(s, "");
        assert_eq!(pass, (1, 3));
        let (s, pass) = pass_to("  If true: throw to monkey 0
  If false: throw to monkey 1").unwrap();
        assert_eq!(s, "");
        assert_eq!(pass, (0, 1));
            }
    #[test]
    fn parse_operations() {
        let (s, op) = operation("  Operation: new = old * 19").unwrap();
        assert_eq!(s, "");
        for i in 0..100 {
            assert_eq!(op(i), i * 19);
        }
        let (s, op) = operation("Operation: new = old + 6").unwrap();
        assert_eq!(s, "");
        for i in 0..100 {
            assert_eq!(op(i), i + 6);
        }
        let (s, op) = operation("    Operation: new = old * old").unwrap();
        assert_eq!(s, "");
        for i in 0..100 {
            assert_eq!(op(i), i * i);
        }
        let (s, op) = operation("  Operation: new = old + 3").unwrap();
        assert_eq!(s, "");
        for i in 0..100 {
            assert_eq!(op(i), i + 3);
        }
    }
    #[test]
    fn prims() {
        assert_eq!(primfactors(12), [2, 2, 3]);
        assert_eq!(primfactors(147), [3, 7, 7]);
        assert_eq!(primfactors(84), [2, 2, 3, 7]);
    }
}

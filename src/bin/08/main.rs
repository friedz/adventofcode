
use std::{
    collections::{
        HashSet,
        HashMap,
    },
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Symbol {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Symbol {
    fn from_char(c: &char) -> SimpleResult<Self> {
        Ok(match c {
            'a' => Symbol::A,
            'b' => Symbol::B,
            'c' => Symbol::C,
            'd' => Symbol::D,
            'e' => Symbol::E,
            'f' => Symbol::F,
            'g' => Symbol::G,
            s => return Err(simple_error!("{:?} is not a correct wire", s)),
        })
    }
}

#[derive(Debug)]
struct MangledWires(HashSet<Symbol>);

impl FromStr for MangledWires {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars()
            .try_fold(MangledWires(HashSet::new()), |mut line, c| -> SimpleResult<MangledWires> {
                line.0.insert(Symbol::from_char(&c)?);
                Ok(line)
            })?)
    }
}
impl MangledWires {
    fn as_set(self) -> HashSet<Symbol> {
        self.0
    }
}

#[derive(Debug)]
struct LookupBuilder {
    two: Vec<HashSet<Symbol>>,
    three: Vec<HashSet<Symbol>>,
    four: Vec<HashSet<Symbol>>,
    five: Vec<HashSet<Symbol>>,
    six: Vec<HashSet<Symbol>>,
}
impl Default for LookupBuilder {
    fn default() -> Self {
        LookupBuilder {
            two: Vec::new(),
            three: Vec::new(),
            four: Vec::new(),
            five: Vec::new(),
            six: Vec::new(),
        }
    }
}
impl FromStr for LookupBuilder {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .try_fold(LookupBuilder::default(), |mut builder, digit| -> SimpleResult<LookupBuilder> {
                let mw = MangledWires::from_str(digit.trim())?.as_set();
                match mw.len() {
                    2 => builder.two.push(mw),
                    3 => builder.three.push(mw),
                    4 => builder.four.push(mw),
                    5 => builder.five.push(mw),
                    6 => builder.six.push(mw),
                    7 => {},
                    _ => return Err(simple_error!("{:?} is not a posible 7 segment digit", mw)),
                }
                Ok(builder)
            })
    }
}
fn count_sets_with_symbol(v: &Vec<HashSet<Symbol>>, s: &Symbol) -> usize {
    v.into_iter().fold(0, |sum, digit| {
        if digit.contains(s) {
            sum + 1
        } else {
            sum
        }
    })
}
impl LookupBuilder {
    fn symbol_filter(&self, s: &Symbol) -> SimpleResult<Symbol> {
        let two = count_sets_with_symbol(&self.two, s);
        let three = count_sets_with_symbol(&self.three, s);
        let four = count_sets_with_symbol(&self.four, s);
        let five = count_sets_with_symbol(&self.five, s);
        let six = count_sets_with_symbol(&self.six, s);
        Ok(match (two, three, four, five, six) {
            (0, 1, 0, 3, 3) => Symbol::A,
            (0, 0, 1, 1, 3) => Symbol::B,
            (1, 1, 1, 2, 2) => Symbol::C,
            (0, 0, 1, 3, 2) => Symbol::D,
            (0, 0, 0, 1, 2) => Symbol::E,
            (1, 1, 1, 2, 3) => Symbol::F,
            (0, 0, 0, 3, 3) => Symbol::G,
            _ => return Err(simple_error!("impossible comnbination")),
        })
    }
    fn digit_builder(&self) -> SimpleResult<DigitBuilder> {
        let mut res = HashMap::new();
        res.insert(Symbol::A, self.symbol_filter(&Symbol::A)?);
        res.insert(Symbol::B, self.symbol_filter(&Symbol::B)?);
        res.insert(Symbol::C, self.symbol_filter(&Symbol::C)?);
        res.insert(Symbol::D, self.symbol_filter(&Symbol::D)?);
        res.insert(Symbol::E, self.symbol_filter(&Symbol::E)?);
        res.insert(Symbol::F, self.symbol_filter(&Symbol::F)?);
        res.insert(Symbol::G, self.symbol_filter(&Symbol::G)?);
        Ok(DigitBuilder {
            table: res,
        })
    }
}
#[derive(Debug)]
struct DigitBuilder {
    table: HashMap<Symbol, Symbol>,
}
impl DigitBuilder {
    fn from_str(&self, s: &str) -> SimpleResult<Digit> {
        Ok(self.from_mangled_wires(MangledWires::from_str(s)?))
    }
    fn from_mangled_wires(&self, mw: MangledWires) -> Digit {
        let set = mw.as_set().drain().fold(HashSet::new(), |mut set, s| {
            set.insert(self.table[&s]);
            set
        });
        Digit {
            a: set.contains(&Symbol::A),
            b: set.contains(&Symbol::B),
            c: set.contains(&Symbol::C),
            d: set.contains(&Symbol::D),
            e: set.contains(&Symbol::E),
            f: set.contains(&Symbol::F),
            g: set.contains(&Symbol::G),
        }
    }
}
#[derive(Debug)]
struct Digit {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}
impl Digit {
    fn as_u8(&self) -> SimpleResult<u8> {
        Ok(match (self.a, self.b, self.c, self.d, self.e, self.f, self.g) {
            (true, true, true, false, true, true, true) => 0,
            (false, false, true, false, false, true, false) => 1,
            (true, false, true, true, true, false, true) => 2,
            (true, false, true, true, false, true, true) => 3,
            (false, true, true, true, false, true, false) => 4,
            (true, true, false, true, false, true, true) => 5,
            (true, true, false, true, true, true, true) => 6,
            (true, false, true, false, false, true, false) => 7,
            (true, true, true, true, true, true, true) => 8,
            (true, true, true, true, false, true, true) => 9,
            w => return Err(simple_error!("{:?} is not a valid number!", w))
        })
    }
}

fn main() -> SimpleResult<()> {
    //let input = include_str!("test_input.txt");
    let input = include_str!("input.txt");
    let (part1, part2) = input.lines().filter(|x| 0 != x.len())
        .try_fold((0,0), |(p1, p2), line| -> SimpleResult<(usize, usize)> {
            let mut line = line.split("|");
            let digit_builder = LookupBuilder::from_str(
                line.next().ok_or(simple_error!("malformed line"))?
            )?.digit_builder()?;
            let (p1, pl2) = line.next().ok_or(simple_error!("malformed line"))?
                .split(" ")
                .map(|x| x.trim())
                .filter(|x| 0 != x.len())
                .try_fold((p1, 0), |(p1, p2), d| -> SimpleResult<(usize, usize)> {
                    let d = digit_builder.from_str(d)?.as_u8()?;
                    let p1 = match d {
                        1 | 4 | 7 | 8 => p1 + 1,
                        _ => p1,
                    };
                    Ok((p1, p2 * 10 + d as usize))
                })?;
            Ok((p1, p2 + pl2))
        })?;
    println!("{:?}", part1);
    println!("{:?}", part2);
    Ok(())
}

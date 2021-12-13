
use std::{
    cmp::{
        max,
        min,
    },
    collections::HashSet,
    fmt::{
        self,
        Display,
    },
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

#[derive(Debug)]
enum FoldLine {
    X(i64),
    Y(i64),
}
impl FromStr for FoldLine {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s.strip_prefix("fold along ") {
            Some(s) => s,
            None => return Err(simple_error!("{:?} is not a valid fold instruction!", s)),
        };
        let mut it = s.split("=");
        let axis = match it.next() {
            Some(z) => z,
            None => return Err(simple_error!("{:?} dosn't have a valid fold axis!", s)),
        };
        let coord = match it.next() {
            Some(i) => match i64::from_str(i) {
                Ok(i) => i,
                Err(e) => return Err(simple_error!("{:?}", e)),
            },
            None => return Err(simple_error!("{:?} dosn't have a valid fold coordinate!", s)),
        };

        Ok(match axis {
            "x" => FoldLine::X(coord),
            "y" => FoldLine::Y(coord),
            e => return Err(simple_error!("{} is not a valid dimension to fold!", e)),
        })
    }
}
impl Display for FoldLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FoldLine::X(x) => write!(f, "fold at x={}", x),
            FoldLine::Y(y) => write!(f, "fold at y={}", y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dot {
    x: i64,
    y: i64,
}
impl FromStr for Dot {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(",");
        let x = it.next().ok_or(simple_error!("{:?} is not a valid Dot!", s))?;
        let x = i64::from_str(x).or_else(|e| Err(simple_error!("{:?}", e)))?;
        let y = it.next().ok_or(simple_error!("{:?} is not a valid Dot!", s))?;
        let y = i64::from_str(y).or_else(|e| Err(simple_error!("{:?}", e)))?;

        Ok(Dot {
            x: x,
            y: y,
        })
    }
}
impl Dot {
    fn new(x: i64, y: i64) -> Dot {
        Dot {
            x: x,
            y: y,
        }
    }
    fn fold(&self, fl: &FoldLine) -> Self {
        match fl {
            FoldLine::X(x) => if self.x > *x {
                Dot::new(x - (self.x - x), self.y)
            } else {
                *self
            }
            FoldLine::Y(y) => if self.y > *y {
                Dot::new(self.x, y - (self.y - y))
            } else {
                *self
            }
        }
    }
}

fn print_dots(dots: &HashSet<Dot>) {
    let (max_x, max_y, min_x, min_y) = dots.into_iter().fold((0, 0, 0, 0), |(max_x, max_y, min_x, min_y), dot| {
        (max(max_x, dot.x), max(max_y, dot.y), min(min_x, dot.x), min(min_y, dot.y))
    });
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&Dot::new(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}


fn main() -> SimpleResult<()> {
    //let input = include_str!("example_input.txt");
    let input = include_str!("input.txt");

    let mut input = input.split("\n\n");
    let dots = input.next().ok_or(simple_error!("No dots!"))?;
    let mut dots = dots.lines().try_fold(HashSet::new(), |mut dots, line| -> SimpleResult<HashSet<Dot>> {
        dots.insert(Dot::from_str(line)?);
        Ok(dots)
    })?;
    //println!("{:?} |{}", dots, dots.len());
    let folds = input.next().ok_or(simple_error!("no folds"))?;
    let folds = folds.lines().try_fold(Vec::new(), |mut folds, line| -> SimpleResult<Vec<FoldLine>> {
        folds.push(FoldLine::from_str(line)?);
        Ok(folds)
    })?;
    //println!("{:?}", folds);

    println!("Befor folding there are {} dots", dots.len());
    for fl in folds {
        dots = dots.into_iter().fold(HashSet::new(), |mut dots, dot| {
            dots.insert(dot.fold(&fl));
            dots
        });
        //println!("{:?} |{}", dots, dots.len());
        println!("After {} there are {} dots", fl, dots.len());
    }
    print_dots(&dots);

    Ok(())
}

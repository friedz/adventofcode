
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    error::Error,
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}
impl FromStr for Cave {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            s => {
                match s.chars().next() {
                    Some(big) if big.is_uppercase() => Cave::Big(String::from(s)),
                    Some(small) if small.is_lowercase() => Cave::Small(String::from(s)),
                    Some(e) => return Err(simple_error!("{} is neighter upercase nor lowercase!", e)),
                    None => return Err(simple_error!("empty str is not a valid cave name!")),
                }
            },
        })
    }
}
impl Cave {
    fn is_small(&self) -> bool {
        match self {
            Cave::Small(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: HashMap<Cave, HashSet<Cave>>
}
impl FromStr for CaveSystem {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines().map(|line| line.trim()).filter(|line| 0 != line.len())
            .try_fold(CaveSystem::default(), |mut caves, line| {
                let mut c = line.split("-");
                let a = match c.next() {
                    Some(cave) => Cave::from_str(cave)?,
                    None => return Err(simple_error!("Nothing to parse in this line")),
                };
                let b = match c.next() {
                    Some(cave) => Cave::from_str(cave)?,
                    None => return Err(simple_error!("No cave to connect to")),
                };
                caves.add_path(a, b);
                Ok(caves)
            })
    }
}
impl Default for CaveSystem {
    fn default() -> Self {
        CaveSystem {
            caves: HashMap::new(),
        }
    }
}
impl CaveSystem {
    fn neighbors(&self, c: &Cave) -> &HashSet<Cave> {
        &self.caves[c]
    }
    fn add_path(&mut self, a: Cave, b: Cave) {
        if b != Cave::Start {
            match self.caves.get_mut(&a) {
                Some(neighbors) => { (*neighbors).insert(b.clone()); }
                None => { self.caves.insert(a.clone(), [b.clone()].into()); }
            }
        }
        if a != Cave::Start {
            match self.caves.get_mut(&b) {
                Some(neighbors) => { (*neighbors).insert(a); }
                None => { self.caves.insert(b, [a].into()); }
            }
        }
    }
    fn find_paths(&self) -> Vec<Vec<Cave>> {
        let mut paths = Vec::new();
        let mut stack = vec![vec![Cave::Start]];
        while let Some(path) = stack.pop() {
            'neighbors: for n in self.neighbors(&path[path.len() - 1]) {
                for i in (&path).into_iter().filter(|c| c.is_small()) {
                    if i == n {
                        continue 'neighbors;
                    }
                }
                let mut p = path.clone();
                p.push(n.clone());
                match *n {
                    Cave::End => paths.push(p),
                    _ => stack.push(p),
                }
            }
        }

        paths
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    //let input = include_str!("small_example_input.txt");
    //let input = include_str!("larger_example_input.txt");
    let input = include_str!("input.txt");

    let caves = CaveSystem::from_str(input)?;
    let paths = caves.find_paths();
    //for path in &paths {
    //    println!("{:?}", path);
    //}
    println!("{}", paths.len());

    Ok(())
}

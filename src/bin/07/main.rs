
use std::{
    collections::HashMap,
    io::{
        self,
        BufRead,
    },
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq)]
enum DirEntry {
    Dir(Dir),
    File(u32),
}
impl DirEntry {
    fn size(&self) -> u32 {
        match self {
            DirEntry::Dir(d) => d.size(),
            DirEntry::File(f) => *f,
        }
    }
    fn dir(&self) -> Option<&Dir> {
        match self {
            DirEntry::Dir(ref dir) => Some(dir),
            _ => None,
        }
    }
    fn dir_mut(&mut self) -> Option<&mut Dir> {
        match self {
            DirEntry::Dir(ref mut dir) => Some(dir),
            //_ => panic!("not a dir"),
            _ => None,
        }
    }
}
macro_rules! file {
    ($size:expr) => {
        DirEntry::File($size)
    };
}
macro_rules! dir {
    ($($name:literal: $entry:expr),*) => {
        DirEntry::Dir(Dir { entries: dir_entries!($($name: $entry),*) })
    };
}
macro_rules! dir_entries {
    ($($name:literal: $entry:expr),*) => {
        {
            let mut tmp_entries = HashMap::<String, DirEntry>::new();
            $(
                tmp_entries.insert(format!("{}", $name), $entry);
            )*
            tmp_entries
        }
    };
}
/// generates a `Vec<String>` for simple path index generation
macro_rules! path {
    () => {
        Vec::<String>::new()
    };
    ($s:literal) => {
        vec![$s.to_owned()]
    };
    ($($s:literal),*) => {
        [$($s,)+].into_iter().map(|c| c.to_owned()).collect::<Vec<_>>()
    };
}

#[derive(Debug, Eq, PartialEq)]
struct Dir {
    entries: HashMap<String, DirEntry>
}
impl Dir {
    fn new() -> Dir {
        Dir {
            entries: HashMap::new(),
        }
    }
    fn size(&self) -> u32 {
        self.entries.iter().fold(0, |size, entry| {
            size + entry.1.size()
        })
    }
    fn get_mut(&mut self, idx: &Vec<String>) -> Option<&mut DirEntry> {
        let mut idx = idx.iter();
        let first = idx.next()?;
        idx.fold(self.entries.get_mut(first), |dir, i| {
            match dir {
                Some(DirEntry::Dir(d)) => d.entries.get_mut(i),
                _ => None,
            }
        })
    }
    fn get(&self, idx: &Vec<String>) -> Option<&DirEntry> {
        let mut idx = idx.iter();
        let first = match idx.next() {
            Some(i) => i,
            None => return None,
        };
        idx.fold(self.entries.get(first), |dir, i| {
            match dir {
                Some(DirEntry::Dir(d)) => d.entries.get(i),
                _ => None,
            }
        })
    }
    fn all_dirs(&self) -> Vec<Vec<String>> {
        let mut stack: Vec<Vec<String>> = self.dirs().into_iter().map(|e| {
            let mut v = Vec::new();
            v.push(e);
            v
        }).collect();
        let mut res = Vec::new();
        while 0 < stack.len() {
            let tmp = stack.pop().unwrap();
            let dirs = self.get(&tmp).unwrap().dir().unwrap().dirs();
            stack = dirs.into_iter().fold(stack, |mut stack, dir| {
                let mut tmpclone = tmp.clone();
                tmpclone.push(dir);
                stack.push(tmpclone);
                stack
            });
            res.push(tmp);
        }
        res
    }
    fn dirs(&self) -> Vec<String> {
        self.entries.iter().fold(Vec::new(), |mut vec, (name, entry)| {
            match entry {
                DirEntry::Dir(_) => vec.push(name.clone()),
                _ => { },
            }
            vec
        })
    }
    fn mkdir(&mut self, name: String) {
        self.entries.insert(name, dir!());
    }
    fn touch(&mut self, name: String, size: u32) {
        self.entries.insert(name, file!(size));
    }
}
impl FromStr for Dir {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, _) = s.as_bytes().lines()
            .try_fold((Dir::new(), path![]), |(mut dir, mut path), line| {
                let line = line?;
                let mut parts = line.split(' ');
                let work_dir = if 0 == path.len() {
                    &mut dir
                } else {
                    dir.get_mut(&path).unwrap().dir_mut().unwrap()
                };
                match parts.next().unwrap() {
                    "$" => match parts.next().unwrap() {
                        "cd" => match parts.next().unwrap() {
                            "/" => path = path![],
                            ".." => { path.pop(); },
                            p => { path.push(p.to_owned()); },
                        }
                        _ => {},
                    },
                    "dir" => {
                        work_dir.mkdir(parts.next().unwrap().to_owned());
                    },
                    size => match size.parse::<u32>() {
                        Ok(size) => work_dir.touch(parts.next().unwrap().to_owned(), size),
                        _ => {},
                    }
                }
                Ok::<_, Self::Err>((dir, path))
            })?;
        Ok(dir)
    }
}

fn part1(dir: &Dir) -> u32 {
    let res = if dir.size() <= 100_000 {
        dir.size()
    } else {
        0
    };
    dir.all_dirs().into_iter().fold(res, |res, path| {
        let size = dir.get(&path).unwrap().size();
        if 100_000 >= size {
            size + res
        } else {
            res
        }
    })
}

fn main() {
    let input = include_str!("input.txt");
    let data = Dir::from_str(&input).unwrap();
    println!("Part 1: {}", part1(&data));
}

#[cfg(test)]
mod tests_day_07 {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    fn test_data() -> Dir {
        Dir {
            entries: dir_entries![
                "a": dir![
                    "e": dir![
                        "i": file!(584)
                    ],
                    "f": file!(29116),
                    "g": file!(2557),
                    "h.lst": file!(62596)
                ],
                "b.txt": file!(14848514),
                "c.dat": file!(8504156),
                "d": dir![
                    "j": file!(4060174),
                    "d.log": file!(8033020),
                    "d.ext": file!(5626152),
                    "k": file!(7214296)
                ]
            ],
        }
    }

    #[test]
    fn small_dirs_sum() {
        assert_eq!(part1(&test_data()), 95437);
    }
    #[test]
    fn parse_dirs() {
        assert_eq!(Dir::from_str(INPUT).unwrap(), test_data());
    }
    #[test]
    fn dir_index() {
        let dir = test_data();
        //let idx = path![];
        //assert_eq!(dir.get(&idx), Some(&DirEntry::Dir(test_data())));
        let idx = path!["a"];
        assert_eq!(dir.get(&idx), Some(&dir![
            "e": dir![
                "i": file!(584)
            ],
            "f": file!(29116),
            "g": file!(2557),
            "h.lst": file!(62596)
        ]));

        let idx = path!["a", "e"];
        assert_eq!(dir.get(&idx), Some(&dir!["i": file!(584)]));
        let idx = path!["a", "e", "i"];
        assert_eq!(dir.get(&idx), Some(&file!(584)));
        let idx = path!["a", "f"];
        assert_eq!(dir.get(&idx), Some(&file!(29116)));
        let idx = path!["a", "g"];
        assert_eq!(dir.get(&idx), Some(&file!(2557)));
        let idx = path!["a", "h.lst"];
        assert_eq!(dir.get(&idx), Some(&file!(62596)));
        let idx = path!["b.txt"];
        assert_eq!(dir.get(&idx), Some(&file!(14848514)));
        let idx = path!["c.dat"];
        assert_eq!(dir.get(&idx), Some(&file!(8504156)));
        let idx = path!["d"];
        assert_eq!(dir.get(&idx), Some(&dir![
            "j": file!(4060174),
            "d.log": file!(8033020),
            "d.ext": file!(5626152),
            "k": file!(7214296)
        ]));
        let idx = path!["d", "j"];
        assert_eq!(dir.get(&idx), Some(&file!(4060174)));
        let idx = path!["d", "d.log"];
        assert_eq!(dir.get(&idx), Some(&file!(8033020)));
        let idx = path!["d", "d.ext"];
        assert_eq!(dir.get(&idx), Some(&file!(5626152)));
        let idx = path!["d", "k"];
        assert_eq!(dir.get(&idx), Some(&file!(7214296)));
    }
}

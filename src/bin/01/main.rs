
use std::{
    iter::Iterator,
    io::Error,
};
use csv::ReaderBuilder;
use std::fs::File;

fn main() -> Result<(), Error> {
    let input: Vec<i32> =  ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open("src/bin/01/input.txt")?)
        .deserialize()
        .into_iter()
        .map(|i| i.expect("Not a number!")).collect();

    let res = input[..input.len()].iter()
        .zip(input[1..].iter())
        .fold(0, |acc, (a, b)| {
            acc + if a < b {
                1
            } else {
                0
            }
        });
    println!("{}", res);

    let res = input[..input.len()].windows(3)
        .zip(input[1..].windows(3))
        .fold(0, |acc, (a, b)| {
            acc + if a.iter().sum::<i32>() < b.iter().sum::<i32>() {
                1
            } else {
                0
            }
        });
    println!("{}", res);

    Ok(())
}


use std::{
    error::Error,
    iter::Iterator,
};
use csv::ReaderBuilder;
use std::fs::File;


fn main() -> Result<(), Box<dyn Error>> {
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
    println!("{:?}", res);

    Ok(())
}

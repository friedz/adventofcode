
#![feature(int_abs_diff)]

use std::{
    error::Error,
    num::ParseIntError,
    str::FromStr,
};

use simple_error::{
    SimpleResult,
    simple_error,
};

const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

fn main() -> Result<(), Box<dyn Error>> {
    //let input = TEST_INPUT;
    let input = include_str!("input.txt");
    let mut data = input.split(',')
        .try_fold(Vec::new(), |mut data, n| -> Result<Vec<usize>, ParseIntError> {
        data.push(usize::from_str(n.trim_end())?);
        Ok(data)
    })?;

    data.sort();
    let mead = data[data.len()/2];
    let res = data.into_iter().fold(0, |cost, i| {
        cost + mead.abs_diff(i)
    });
    println!("{}", res);

    Ok(())
}

use std::{
    error::Error,
    str::FromStr,
};

use csv::{
    ReaderBuilder,
    Terminator,
};
use simple_error::SimpleError;

const TEST_INPUT: &str = "3,4,3,1,2";

fn main() -> Result<(), Box<dyn Error>> {
    //let input = TEST_INPUT;
    let input = include_str!("input.txt");
    let population = input.split(',').try_fold([0; 9], |pop, x| {
        let mut pop = pop;
        //let x = usize::from_str(x)?;
        let x = match usize::from_str(x.trim_end()) {
            Ok(x) => x,
            Err(e) => return Err(SimpleError::new(format!("{:?}", e))),
        };
        pop[x] += 1;
        Ok(pop)
    })?;

    let res: u64 = (0..80).into_iter().fold(population, |p, _| {
        let tmp = p[0];
        [p[1], p[2], p[3], p[4], p[5], p[6], p[7] + tmp, p[8], tmp]

    }).into_iter().sum();
    println!("{}", res);

    let res: u64 = (0..256).into_iter().fold(population, |p, _| {
        let tmp = p[0];
        [p[1], p[2], p[3], p[4], p[5], p[6], p[7] + tmp, p[8], tmp]

    }).into_iter().sum();

    println!("{}", res);


    Ok(())
}

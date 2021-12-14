
use std::collections::HashMap;
use simple_error::{
    SimpleError,
    simple_error,
    SimpleResult,
};

fn max_and_min(polymer: &str) -> SimpleResult<((char, usize), (char, usize))> {
    let count_map = polymer.chars().fold(HashMap::new(), |mut counts, element| {
        if let Some(count) = counts.get_mut(&element) {
            *count += 1;
        } else {
            counts.insert(element, 1);
        }
        counts
    });
    println!("{:?}", count_map);
    let max = count_map.iter().max_by(|a, b| a.1.cmp(&b.1))
        .ok_or(simple_error!("No maximum found?!?"))?;
    let min = count_map.iter().min_by(|a, b| a.1.cmp(&b.1))
        .ok_or(simple_error!("No minimum found?!?"))?;

    Ok(((*max.0, *max.1), (*min.0, *min.1)))
}

fn main() -> SimpleResult<()> {
    //let input = include_str!("example_input.txt");
    let input = include_str!("input.txt");
    let mut input = input.split("\n\n");
    let polymer = input.next().ok_or(simple_error!("No polymer providet!"))?;
    let lookup = input.next().ok_or(simple_error!("No pair insetion rules providet!"))?
        .lines().try_fold(HashMap::new(), |mut lookup, line| -> SimpleResult<HashMap<(char, char), char>> {
            let mut elm = line.split(" -> ");

            let mut key = elm.next().ok_or(simple_error!("No pair to transform providet!"))?.trim().chars();
            let a = key.next().ok_or(simple_error!("No elements in pair!"))?;
            let b = key.next().ok_or(simple_error!("Only one element in pair!"))?;

            let val = elm.next().ok_or(simple_error!("Nothing to insert providet!"))?.trim()
                .chars().next().ok_or(simple_error!("Nothing to insert providet!"))?;

            lookup.insert((a, b), val);
            Ok(lookup)
        })?;
    //println!("{}", polymer);
    //println!("{:?}", lookup);

    let mut polymer = polymer.to_string();
    //let mut polymer_1 = String::new();
    for i in 1..=40 {
        polymer = polymer.chars().fold((None, String::new()), |(last, mut polymer), c| {
            if let Some(last) = last {
                if let Some(insert) = lookup.get(&(last, c)) {
                    polymer.push(*insert);
                }
            }
            polymer.push(c);
            (Some(c), polymer)
        }).1;
        //println!("{}: {}", i, polymer);
        if 10 == i {
            let (max, min) = max_and_min(&polymer)?;
            //polymer_1 = polymer.clone();
            println!("Max: {:?}, Min: {:?} -> {}", max, min, max.1 - min.1);
        }
    }
    //let (max, min) = max_and_min(&polymer_1)?;
    //println!("Max: {:?}, Min: {:?} -> {}", max, min, max.1 - min.1);
    let (max, min) = max_and_min(&polymer)?;
    println!("Max: {:?}, Min: {:?} -> {}", max, min, max.1 - min.1);

    Ok(())
}

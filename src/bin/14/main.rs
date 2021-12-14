
use std::{
    collections::HashMap,
    hash::Hash,
    ops::AddAssign,
};
use simple_error::{
    simple_error,
    SimpleResult,
};

type PolyMap = HashMap<(char, char), usize>;
type LookUp = HashMap<(char, char), char>;

fn max_and_min(poly_map: &PolyMap) -> SimpleResult<((char, usize), (char, usize))> {
    let count_map = poly_map.iter().fold(HashMap::new(), |mut count_map, ((_, k), v)| {
        insert_or_add(&mut count_map, k, v);
        count_map
    });
    let max = count_map.iter().max_by(|a, b| a.1.cmp(&b.1))
        .ok_or(simple_error!("No maximum found?!?"))?;
    let min = count_map.iter().min_by(|a, b| a.1.cmp(&b.1))
        .ok_or(simple_error!("No minimum found?!?"))?;

    Ok(((*max.0, *max.1), (*min.0, *min.1)))
}

fn genererate_pair_map(polymer: &str) -> PolyMap {
    polymer.chars().fold((None, HashMap::new()), |(last, mut polymer_map), c| {
        if let Some(last) = last {
            if let Some(val) = polymer_map.get_mut(&(last, c)) {
                *val += 1;
            } else {
                polymer_map.insert((last, c), 1);
            }
        }
        (Some(c), polymer_map)
    }).1
}
fn insert_or_add<K: Eq + Hash + Copy, V: AddAssign + Copy>(map: &mut HashMap<K, V>, key: &K, val: &V) {
    if let Some(v) = map.get_mut(key) {
        *v += *val;
    } else {
        map.insert(*key, *val);
    }
}
fn transform_map(polymer_map: PolyMap, lookup: &LookUp) -> PolyMap {
    polymer_map.into_iter().fold(PolyMap::new(), |mut poly_map, ((k_left, k_right), v)| {
        if let Some(insert) = lookup.get(&(k_left, k_right)) {
            insert_or_add(&mut poly_map, &(k_left, *insert), &v);
            insert_or_add(&mut poly_map, &(*insert, k_right), &v);
        } else {
            insert_or_add(&mut poly_map, &(k_left, k_right), &v);
        }
        poly_map
    })
}

fn main() -> SimpleResult<()> {
    //let input = include_str!("example_input.txt");
    let input = include_str!("input.txt");
    let mut input = input.split("\n\n");
    let polymer = input.next().ok_or(simple_error!("No polymer providet!"))?;
    let lookup = input.next().ok_or(simple_error!("No pair insetion rules providet!"))?
        .lines().try_fold(LookUp::new(), |mut lookup, line| -> SimpleResult<LookUp> {
            let mut elm = line.split(" -> ");

            let mut key = elm.next().ok_or(simple_error!("No pair to transform providet!"))?.trim().chars();
            let a = key.next().ok_or(simple_error!("No elements in pair!"))?;
            let b = key.next().ok_or(simple_error!("Only one element in pair!"))?;

            let val = elm.next().ok_or(simple_error!("Nothing to insert providet!"))?.trim()
                .chars().next().ok_or(simple_error!("Nothing to insert providet!"))?;

            lookup.insert((a, b), val);
            Ok(lookup)
        })?;
    println!("{}", polymer);
    //println!("{:?}", lookup);

    let mut poly_map = genererate_pair_map(polymer);
    //println!("{:?}", poly_map);
    for i in 1..=40 {
        poly_map = transform_map(poly_map, &lookup);
        if 10 == i || 40 == i {
            let (max, min) = max_and_min(&poly_map)?;
            println!("Max: {:?}, Min: {:?} -> {}", max, min, max.1 - min.1);
        }
    }

    Ok(())
}

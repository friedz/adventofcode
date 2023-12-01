#![feature(is_some_and)]

use std::{
    cmp::max,
    collections::HashMap,
    ops::{
        Index,
        IndexMut,
        AddAssign,
        SubAssign,
    },
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
fn parse_material(s: &str) -> IResult<&str, Material> {
    alt((
            map(tag("ore"), |_| Material::Ore),
            map(tag("clay"), |_| Material::Clay),
            map(tag("obsidian"), |_| Material::Obsidian),
            map(tag("geode"), |_| Material::Geode),
    ))(s)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct MaterialPile {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}
impl MaterialPile {
    fn empty() -> Self {
        MaterialPile {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
    fn new(ore: i32, clay: i32, obsidian: i32, geode: i32) -> Self {
        MaterialPile {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }
}
impl Index<&Material> for MaterialPile {
    type Output = i32;
    fn index(&self, index: &Material) -> &Self::Output {
        match index {
            Material::Ore => &self.ore,
            Material::Clay => &self.clay,
            Material::Obsidian => &self.obsidian,
            Material::Geode => &self.geode,
        }
    }
}
impl IndexMut<&Material> for MaterialPile {
    fn index_mut(&mut self, index: &Material) -> &mut Self::Output {
        match index {
            Material::Ore => &mut self.ore,
            Material::Clay => &mut self.clay,
            Material::Obsidian => &mut self.obsidian,
            Material::Geode => &mut self.geode,
        }
    }
}
impl AddAssign for MaterialPile {
    fn add_assign(&mut self, other: Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}
impl SubAssign for MaterialPile {
    fn sub_assign(&mut self, other: Self) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    number: i32,
    robots: HashMap<Material, MaterialPile>,
}
impl Blueprint {
    fn largest_number_of_geodes(
        &self,
        mut materials: MaterialPile,
        robots: MaterialPile,
        time_left: i32
    ) -> i32 {
        if time_left <= 0 {
            return materials[&Material::Geode];
        }
        let geodes = self.robots.iter().fold(0, |most_geodes, (robot, costs)| {
            if costs.ore <= materials.ore
            && costs.clay <= materials.clay
            && costs.obsidian <= materials.obsidian
            {
                let mut new_robots = robots.clone();
                new_robots[robot] += 1;
                let mut new_materials = materials.clone();
                dbg!(new_materials, new_robots);
                // the material needet for the new robot is consumed
                new_materials -= *costs;
                // every robot mines its material
                new_materials += robots;
                let geodes = self.largest_number_of_geodes(new_materials, new_robots, time_left - 1);
                max(geodes, most_geodes)
            } else {
                most_geodes
            }
        });
        materials += robots;
        max(geodes, self.largest_number_of_geodes(materials, robots, time_left - 1))
    }
}
fn parse_material_amount(s: &str) -> IResult<&str, (Material, i32)> {
    let (s, amount) = complete::i32(s)?;
    let (s, _) = complete::space1(s)?;
    let (s, material) = parse_material(s)?;
    Ok((s, (material, amount)))
}
fn parse_robot_blueprint(s: &str) -> IResult<&str, (Material, MaterialPile)> {
    let (s, _) = tag("Each ")(s)?;
    let (s, material) = parse_material(s)?;
    let (s, _) = tag(" robot costs ")(s)?;
    let (s, cost_list) = separated_list0(tag(" and "), parse_material_amount)(s)?;
    let mut costs = MaterialPile::empty();
    for (m, c) in cost_list {
        costs[&m] = c;
    }
    Ok((s, (material, costs)))
}
fn parse_blueprint(s: &str) -> IResult<&str, Blueprint> {
    let (s, _) = tag("Blueprint ")(s)?;
    let (s, number) = complete::i32(s)?;
    let (s, _) = tag(": ")(s)?;
    let (s, cost_list) = separated_list0(tag(". "), parse_robot_blueprint)(s)?;
    let (s, _) = tag(".")(s)?;
    Ok((s, Blueprint {
        number: number,
        robots: cost_list.into_iter().collect(),
    }))
}
fn parse_blueprint_list(s: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list0(complete::newline, parse_blueprint)(s)
}

fn main() {
    let input = include_str!("input.txt");
}

#[cfg(test)]
mod tests_day_19 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");
    fn example_blueprints() -> Vec<Blueprint> {
        vec![
            Blueprint {
                number: 1,
                robots: HashMap::from([
                    (Material::Ore, MaterialPile::new(4, 0, 0, 0)),
                    (Material::Clay, MaterialPile::new(2, 0, 0, 0)),
                    (Material::Obsidian, MaterialPile::new(3, 14, 0, 0)),
                    (Material::Geode, MaterialPile::new(2, 0, 7, 0)),
                ]),
            },
            Blueprint {
                number: 2,
                robots: HashMap::from([
                    (Material::Ore, MaterialPile::new(2, 0, 0, 0)),
                    (Material::Clay, MaterialPile::new(3, 0, 0, 0)),
                    (Material::Obsidian, MaterialPile::new(3, 8, 0, 0)),
                    (Material::Geode, MaterialPile::new(3, 0, 12, 0)),
                ]),
            }
        ]
    }

    #[test]
    fn read_material() {
        assert_eq!(parse_material("ore"), Ok(("", Material::Ore)));
        assert_eq!(parse_material("clay"), Ok(("", Material::Clay)));
        assert_eq!(parse_material("obsidian"), Ok(("", Material::Obsidian)));
        assert_eq!(parse_material("geode"), Ok(("", Material::Geode)));
    }
    #[test]
    fn read_material_amount() {
        assert_eq!(parse_material_amount("20 ore"), Ok(("", (Material::Ore, 20))));
        assert_eq!(parse_material_amount("12 clay"), Ok(("", (Material::Clay, 12))));
        assert_eq!(parse_material_amount("9 obsidian"), Ok(("", (Material::Obsidian, 9))));
        assert_eq!(parse_material_amount("1 geode"), Ok(("", (Material::Geode, 1))));
    }
    #[test]
    fn read_robot_blueprint() {
        assert_eq!(
            parse_robot_blueprint("Each ore robot costs 4 ore"),
            Ok(("", (Material::Ore, MaterialPile::new(4, 0, 0, 0))))
        );
        assert_eq!(
            parse_robot_blueprint("Each clay robot costs 2 ore."),
            Ok((".", (Material::Clay, MaterialPile::new(2, 0, 0, 0))))
        );
        assert_eq!(
            parse_robot_blueprint("Each obsidian robot costs 3 ore and 14 clay"),
            Ok(("", (Material::Obsidian, MaterialPile::new(3, 14, 0, 0))))
        );
        assert_eq!(
            parse_robot_blueprint("Each geode robot costs 2 ore and 7 obsidian."),
            Ok((".", (Material::Geode, MaterialPile::new(2, 0, 7, 0))))
        );
    }
    #[test]
    fn read_blueprint() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        assert_eq!(
            parse_blueprint(input),
            Ok(("", example_blueprints()[0].clone()))
        );
        let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(
            parse_blueprint(input),
            Ok(("", example_blueprints()[1].clone()))
        );
    }
    #[test]
    fn read_all_blueprints() {
        assert_eq!(parse_blueprint_list(INPUT), Ok(("\n", example_blueprints())));
    }
    #[ignore]
    #[test]
    fn get_the_most_geodes() {
        //assert_eq!(example_blueprints()[0]
        //    //.largest_number_of_geodes(MaterialPile::empty(), MaterialPile::new(1, 0, 0, 0), 24), 9);
        //    .largest_number_of_geodes(MaterialPile::empty(), MaterialPile::new(1, 0, 0, 0), 18), 9);
        assert_eq!(example_blueprints()[1]
        //    .largest_number_of_geodes(MaterialPile::empty(), MaterialPile::new(1, 0, 0, 0), 24), 12);
            .largest_number_of_geodes(MaterialPile::empty(), MaterialPile::new(1, 0, 0, 0), 18), 9);
    }
}


use std::{
    cmp::{
        max,
        min,
    },
    collections::HashSet,
    time::SystemTime,
};
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list0,
    IResult,
};

fn parse_coordinate_triple(s: &str) -> IResult<&str, (i32, i32, i32)> {
    let (s, x) = complete::i32(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, y) = complete::i32(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, z) = complete::i32(s)?;
    Ok((s, (x, y, z)))
}
//fn parse_droplet(s: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
fn parse_droplet(s: &str) -> IResult<&str, Droplet> {
    let (s, list) = separated_list0(complete::newline, parse_coordinate_triple)(s)?;
    Ok((s, Droplet::new(&list)))
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct Droplet {
    voxel: HashSet<(i32, i32, i32)>,
}
impl Droplet {
    fn new(voxel: &[(i32, i32, i32)]) -> Droplet {
        let voxel: HashSet<(i32, i32, i32)> = voxel.into_iter().copied().collect();
        Droplet {
            voxel: voxel,
        }
    }
    fn bounding_box(&self) -> ((i32, i32, i32), (i32, i32, i32)) {
        self.voxel.iter().fold(
            ((i32::MAX,i32::MAX,i32::MAX), (i32::MIN,i32::MIN,i32::MIN)),
            |((xmin, ymin, zmin), (xmax, ymax, zmax)), (x, y, z)| {
                (
                    (min(xmin, *x), min(ymin, *y), min(zmin, *z)),
                    (max(xmax, *x + 1), max(ymax, *y + 1), max(zmax, *z + 1))
                )
            })
    }
    fn surface(&self) -> i32 {
        self.voxel.iter().copied().fold(0, |surface, (x, y, z)| {
            surface
            + if self.voxel.contains(&(x + 1, y, z)) { 0 } else { 1 }
            + if self.voxel.contains(&(x - 1, y, z)) { 0 } else { 1 }
            + if self.voxel.contains(&(x , y + 1, z)) { 0 } else { 1 }
            + if self.voxel.contains(&(x , y - 1, z)) { 0 } else { 1 }
            + if self.voxel.contains(&(x , y, z + 1)) { 0 } else { 1 }
            + if self.voxel.contains(&(x , y, z - 1)) { 0 } else { 1 }
        })
    }
    fn is_outside(&self, (x, y, z): (i32, i32, i32)) -> bool {
        if self.voxel.contains(&(x, y, z)) { return false; }
        let ((xmin, ymin, zmin), (xmax, ymax, zmax)) = self.bounding_box();
        let mut checked = HashSet::from([(x,y,z)]);
        let mut to_check =  vec![(x, y, z)];
        loop {
            match to_check.pop() {
                Some((x, y, z)) => {
                    if x < xmin || y < ymin || z < zmin || z > zmax || y > ymax || x > xmax {
                        return true;
                    }
                    checked.insert((x, y, z));
                    let neighbors =  [
                        (x + 1, y, z), (x, y + 1, z), (x, y, z + 1),
                        (x - 1, y, z), (x, y - 1, z), (x, y, z - 1)
                    ];
                    for vox in neighbors {
                       if !(self.voxel.contains(&vox) || checked.contains(&vox)) {
                           to_check.push(vox);
                           checked.insert(vox);
                       }
                    }
                },
                None => { return false; },
            }
        }
    }
    fn exterior_surface(&self) -> i32 {
        self.voxel.iter().fold(0, |surface, (x, y, z)| {
            let neighbors =  [
                (x + 1, *y, *z), (*x, *y + 1, *z), (*x, *y, *z + 1),
                (x - 1, *y, *z), (*x, *y - 1, *z), (*x, *y, *z - 1)
            ];
            neighbors.iter().fold(surface, |sur, pos| {
                sur + if self.is_outside(*pos) { 1 } else { 0 }
            })
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (_, droplet) = parse_droplet(input).unwrap();

    let now = SystemTime::now();
    let part1 = droplet.surface();
    let elapsed = now.elapsed().unwrap();
    println!("Part 1: {}", part1);
    println!("  took: {:?}", elapsed);

    let now = SystemTime::now();
    let part2 = droplet.exterior_surface();
    let elapsed = now.elapsed().unwrap();
    println!("Part 2: {}", part2);
    println!("  took: {:?}", elapsed);
}

#[cfg(test)]
mod tests_day_18 {
    use super::*;

    const INPUT: &str = include_str!("example.txt");
    fn example_droplet() -> Droplet {
        Droplet::new(&[
            (2,2,2),(1,2,2),(3,2,2),(2,1,2),(2,3,2),(2,2,1),(2,2,3),
            (2,2,4),(2,2,6),(1,2,5),(3,2,5),(2,1,5),(2,3,5)
        ])
    }

    #[test]
    fn example_part1() {
        let (_, drop) = parse_droplet(INPUT).unwrap();
        assert_eq!(drop.surface(), 64);
    }
    #[test]
    fn check_if_voxel_is_outside() {
        assert!(!example_droplet().is_outside((2,2,5)));
        assert!(!example_droplet().is_outside((2,2,2)));
        assert!(example_droplet().is_outside((2,2,7)));
        assert!(example_droplet().is_outside((2,1,3)));
    }
    #[test]
    fn droplet_exterior_surface() {
        assert_eq!(example_droplet().exterior_surface(), 58);
    }
    #[test]
    fn droplet_surface() {
        assert_eq!(example_droplet().surface(), 64);
    }
    #[test]
    fn read_droplet() {
        let (_, drop) = parse_droplet(INPUT).unwrap();
        assert_eq!(drop, example_droplet());
    }
    #[test]
    fn read_point() {
        assert_eq!(parse_coordinate_triple("2,2,2"), Ok(("", (2, 2, 2))));
        assert_eq!(parse_coordinate_triple("0,0,0"), Ok(("", (0, 0, 0))));
        assert_eq!(parse_coordinate_triple("2,3,5"), Ok(("", (2, 3, 5))));
    }
}

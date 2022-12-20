use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use std::collections::HashSet;
use regex::Regex;
use std::cmp::{max, min};


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(Clone,Copy,Hash,PartialEq,Eq,Debug)]
struct Point { 
    x: i32,
    y: i32,
    z: i32
}

impl Point{

    fn adjacent(&self, drugi: Point ) -> bool {
        let mut adjacent: bool = false;

        let disx = (self.x-drugi.x).abs();
        let disy = (self.y-drugi.y).abs();
        let disz = (self.z-drugi.z).abs();

        if (disx + disy + disz) == 1 {
            adjacent = true;
        }
        adjacent
    }

    fn neighbours(&self) -> HashSet<Point> {
        return HashSet::from([
            Point { x: self.x - 1, ..*self }, Point { x: self.x + 1, ..*self },
            Point { y: self.y - 1, ..*self }, Point { y: self.y + 1, ..*self },
            Point { z: self.z - 1, ..*self }, Point { z: self.z + 1, ..*self }
        ]);
    }

    fn sides_touching(&self, other: &HashSet<Point>) -> i32 {
        self.neighbours().iter().filter(|p| other.contains(&p)).count() as i32
    }

}

#[derive(Debug)]
struct BoundingBox { maxx: i32, minx: i32, maxy: i32, miny: i32, maxz: i32, minz: i32 }

impl BoundingBox {
    fn new(points: &HashSet<Point>) -> BoundingBox {
        return BoundingBox {
            maxx: points.iter().fold(i32::MIN, |acc, a| max(acc, a.x)) + 1,
            minx: points.iter().fold(i32::MAX, |acc, a| min(acc, a.x)) - 1,
            maxy: points.iter().fold(i32::MIN, |acc, a| max(acc, a.y)) + 1,
            miny: points.iter().fold(i32::MAX, |acc, a| min(acc, a.y)) - 1,
            maxz: points.iter().fold(i32::MIN, |acc, a| max(acc, a.z)) + 1,
            minz: points.iter().fold(i32::MAX, |acc, a| min(acc, a.z)) - 1
        }
    }

    fn inside(&self, p: Point) -> bool {
        return p.x >= self.minx && p.x <= self.maxx &&
            p.y >= self.miny && p.y <= self.maxy &&
            p.z >= self.minz && p.z <= self.maxz;            
    }
    
    fn outside_points(&self, solid: &HashSet<Point>) -> HashSet<Point> {
        let mut res = HashSet::new();
        let mut work = Vec::from([Point { x: self.minx, y: self.miny, z: self.minz }]);
        while let Some(p) = work.pop() {
            if !solid.contains(&p) && !res.contains(&p) && self.inside(p) {
                res.insert(p);
                p.neighbours().iter().for_each(|n| work.push(*n));
            }            
        }
        return res;
    }
}

fn read_points(filename: &str) -> HashSet<Point> {
    let re = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
    let mut result = HashSet::new();
    let lines = lines_from_file("18.txt").expect("Could not load lines");
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            result.insert(Point{ x: cap[1].parse().unwrap(),
                                 y: cap[2].parse().unwrap(),
                                 z: cap[3].parse().unwrap()});
        }
    }
    return result;
}

fn main() {

    let points = read_points("18.txt");
    let res: i32 = points.iter().map(|p| 6 - p.sides_touching(&points)).sum();
    println!("part 1: {}", res);

    let outside = BoundingBox::new(&points).outside_points(&points);
    let res: i32 = points.iter().map(|p| p.sides_touching(&outside)).sum();
    println!("Star 2: {}", res);

    //my_ugly_part1();

}

fn my_ugly_part1(){
    let lines = lines_from_file("18.txt").expect("Could not load lines");
    
    let mut blocks: Vec<Point> = vec![];

    for line in lines {
        let coords: Vec<&str>=line.split(",").collect();
        blocks.push(
            Point{
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap(),
                z: coords[2].parse().unwrap()
            });   
    }

    //println!("{:?}", blocks);
    let mut walls_sum = 0;
    
    for x in 0..blocks.len() {
        let mut touching = 0;
        for y in 0..blocks.len(){
            if x != y {
                if blocks[x].adjacent(blocks[y].clone()) {
                    touching += 1;
                }
            }
            
        }
        walls_sum += (6 - touching);
    }
    println!("sum of part1: {}", walls_sum);
}


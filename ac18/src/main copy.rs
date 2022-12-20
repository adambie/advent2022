use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use multiarray::*;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point{
    fn new(x: i32, y:i32, z:i32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

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
}

fn main() {
    let lines = lines_from_file("18.txt").expect("Could not load lines");
    
    let mut blocks: Vec<Point> = vec![];

    for line in lines {
        let coords: Vec<&str>=line.split(",").collect();
        blocks.push(
            Point::new(
                coords[0].parse().unwrap(),
                coords[1].parse().unwrap(),
                coords[2].parse().unwrap()
            ));
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


    //---------------------------------------------------------------- PART 2
    #[derive(Debug, Copy, Clone)]
    enum Fill {
        Empty,
        Rock,
        Water
    }

    let mut space = Array3D::new([100,100,100], Fill::Empty);
    //move elements towards middle of space by 10 units an add
    for elem in blocks {
        space[[ (elem.x +10) as usize,
                (elem.y +10) as usize,
                (elem.z +10) as usize]] = Fill::Rock;
    }

    let mut to_check: Vec<Point> = vec![];
    to_check.push(Point::new(0,0,0));
    let mut outer_surface = 0;
    loop {
        if to_check.len() == 0{
            break;
        }

        let worktable = to_check.clone();
        to_check = vec![];
        for elem in worktable {
            //if_touching rock
            outer_surface += getTouching(&elem, &space);
            //add_possible_neighbours
            let neighbours = getNeighbours(&elem, &space);
        }
    }

    fn getNeighbours(elem: &Point, space: &MultiArray<T, Dim3> ) -> Vec<Point> {

        vec![Point::new(1,1,1)]
    }
    
    fn getTouching(elem: &Point, space: &MultiArray<T, Dim3>  ) -> i32{
    
        0
    }

}


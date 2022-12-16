use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point{
    fn new(x: i32, y:i32) -> Self {
        Self {
            x,
            y
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
fn parse() -> (Vec<Vec<char>> , usize) {
    let lines = lines_from_file("14.txt").expect("Could not load lines");
    
    let mut minx =9999;
    let mut maxx =0;
    let mut maxy =0;
    let mut allrocks: Vec<Vec<Point>> = vec![];
    let drop_point:usize;

    for line in lines {
        let mut rocks: Vec<Point> = vec![];
        let coords = line.split(" -> ");
        for coord in coords {
            let pair = coord.split(",").collect::<Vec<&str>>();
            if pair.len() == 2 {
                let x:i32 = pair[0].parse().unwrap();
                let y:i32 = pair[1].parse().unwrap();

                if x <= minx {
                    minx = x;
                }

                if x >= maxx {
                    maxx = x;
                }

                if y >= maxy {
                    maxy = y;
                }
                rocks.push(Point::new(x,y));

            }        
        }
        allrocks.push(rocks);
    }
    let mut map: Vec<Vec<char>> = vec![vec!['.'; (maxx-minx+1) as usize]; (maxy+1) as usize];
    drop_point = 500-(minx as usize)-1;

    //normalize rocks coords to begin at zero
    for x in 0..allrocks.len(){
        for y in 0..allrocks[x].len() {
            allrocks[x][y].x -= minx;
        }
    }
    println!("{:?}",allrocks);
    //update map
    for rock in 0..allrocks.len() {
        for rock_line in 0..allrocks[rock].len() {
            println!("# in pos {} {}",allrocks[rock][rock_line].x, allrocks[rock][rock_line].y );
            map[allrocks[rock][rock_line].y as usize][allrocks[rock][rock_line].x as usize] = '#';
            if rock_line > 0 {
                let ydist = allrocks[rock][rock_line-1].y - allrocks[rock][rock_line].y;
                for i in 0..ydist.abs() {
                    map[(allrocks[rock][rock_line].y+i*ydist.signum()) as usize][allrocks[rock][rock_line].x as usize] = '#';
                }

                let xdist = allrocks[rock][rock_line-1].x - allrocks[rock][rock_line].x;
                for i in 0..xdist.abs() {
                    map[allrocks[rock][rock_line].y as usize][(allrocks[rock][rock_line].x+i*xdist.signum()) as usize] = '#';
                }
            }
        }
    }
    (map, drop_point)
}



pub fn part_1() -> usize {
    let mut map: Vec<Vec<char>>; 
    let drop_point: usize;
    (map, drop_point) = parse();
    
    for x in 0..map[0].len(){
        for y in 0..map.len() {
            print!("{}", map[x][y]);
        }
        println!("");
    }
    println!("drop: {}",drop_point);
    0
}


fn main() {
    println!("p1 result: {}", part_1());
}
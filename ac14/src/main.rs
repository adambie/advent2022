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

#[derive(Debug, Copy, Clone)]
struct Sand {
    x: usize,
    y: usize,
    falling: bool,
    fallout: bool
}

impl Sand{
    fn new(x: usize, y:usize, falling:bool, fallout: bool) -> Self {
        Self {
            x,
            y,
            falling,
            fallout
        }
    }

    fn fall(&mut self, mut map: &mut Vec<Vec<char>>) {
        let x: usize = self.x as usize;
        let y: usize = self.y as usize;

        //println!("falling from {}, {}", x,y);

        if map[x][(y+1)] == '.' {
            self.y += 1;
        } else if map[(x-1)][(y+1)] == '.' {
            self.x -= 1;
            self.y += 1;

        } else if map[(x+1)][(y+1)] == '.' {
            self.x +=1;
            self.y +=1;

        } else {
            self.falling = false;
        }

        if self.falling {
            map[x][y]='.';
            map[self.x][self.y]='O';
        } else {
            map[self.x][self.y]='B';
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
fn parse() -> (Vec<Vec<char>> , usize) {
    let lines = lines_from_file("14.txt").expect("Could not load lines");
    
    let mut minx =99999;
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
    println!("maxx-maxy {} {} {}",maxx, minx,maxy);
    drop_point = 500-(minx as usize)-1;

    //normalize rocks coords to begin at zero
    for x in 0..allrocks.len(){
        for y in 0..allrocks[x].len() {
            allrocks[x][y].x -= minx;
        }
    }
    //println!("{:?}",allrocks);
    //update map
    for rock in 0..allrocks.len() {
        for rock_line in 0..allrocks[rock].len() {
            //println!("# in pos {} {}",allrocks[rock][rock_line].x, allrocks[rock][rock_line].y );
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

fn parse2() -> (Vec<Vec<char>> , usize) {
    let lines = lines_from_file("14.txt").expect("Could not load lines");
    
    let mut minx =99999;
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
    let mut map: Vec<Vec<char>> = vec![vec!['.';(maxy+3) as usize] ; (1000) as usize];
    println!("maxx-maxy {} {} {}",maxx, minx,maxy);
    drop_point = 499;

    //add floor
    for x in 0..1000 {
        map[x][(maxy+2) as usize]='#';
    }

    //normalize rocks coords to begin at zero
    // for x in 0..allrocks.len(){
    //     for y in 0..allrocks[x].len() {
    //         allrocks[x][y].x -= minx;
    //     }
    // }
    //println!("{:?}",allrocks);
    //update map
    for rock in 0..allrocks.len() {
        for rock_line in 0..allrocks[rock].len() {
            // println!("# in pos {} {}",allrocks[rock][rock_line].x, allrocks[rock][rock_line].y );
            map[allrocks[rock][rock_line].x as usize][allrocks[rock][rock_line].y as usize] = '#';
            if rock_line > 0 {
                let ydist = allrocks[rock][rock_line-1].y - allrocks[rock][rock_line].y;
                for i in 0..ydist.abs() {
                    map[allrocks[rock][rock_line].x as usize][(allrocks[rock][rock_line].y+i*ydist.signum()) as usize] = '#';
                }

                let xdist = allrocks[rock][rock_line-1].x - allrocks[rock][rock_line].x;
                for i in 0..xdist.abs() {
                    map[(allrocks[rock][rock_line].x+i*xdist.signum()) as usize][allrocks[rock][rock_line].y as usize] = '#';
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
            print!("{}", map[y][x]);
        }
        println!("");
    }
    println!("drop: {}",drop_point);
    let mut sand = Sand::new(drop_point,0  ,true,false);
    let mut cnt=1;
    loop {
        sand.fall(&mut map);

        if sand.falling == false {
            sand = Sand::new((drop_point+1),0  ,true,false);
            cnt +=1;
            println!("CNT {}",cnt);
            // for x in 0..map[0].len(){
            //     for y in 0..map.len() {
            //         print!("{}", map[y][x]);
            //     }
            //     println!("");
            // }    
        }

    }

}

pub fn part_2() -> usize {
    let mut map: Vec<Vec<char>>; 
    let drop_point: usize;
    (map, drop_point) = parse2();
    
    for x in 0..map[0].len(){
        for y in 0..map.len() {
            print!("{}", map[y][x]);
        }
        println!("");
    }
    // println!("drop: {}",drop_point);
    let mut sand = Sand::new(drop_point-1,0  ,true,false);
    let mut cnt=1;
    loop {
        sand.fall(&mut map);

        if sand.falling == false && sand.y==0 {

            for x in 0..map[0].len(){
                for y in 0..map.len() {
                    print!("{}", map[y][x]);
                }
                println!("");
            }                
            println!("ended at  {}",cnt);
            break;
        }        
        if sand.falling == false {
            sand = Sand::new((drop_point-1),0  ,true,false);
            cnt +=1;
            //println!("CNT {}",cnt);
        }

        

    }

        0    

}


fn main() {
    //p1 crashes with sand that falls out of board, so result is lats printed sand number -1 :)
    //println!("p1 result: {}", part_1());

    println!("p2 result: {}", part_2());

}
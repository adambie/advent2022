use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    dist: i32
}

impl Point{
    fn new(x: i32, y:i32, dist: i32) -> Self {
        Self {
            x,
            y,
            dist
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn can_go(a: char, b: char) -> bool {
    
    let mut wa: char = a;
    let mut wb: char = b;
    if wa == '$' {
        wa = 'a';
    }
    if wa == '&' {
        wa = 'z';
    }
    if wb == '$' {
        wb = 'a';
    }
    if wb == '&' {
        wb = 'z';
    }

    let alphabet: Vec<char> = "$abcdefghijklmnopqrstuvwxyz&".chars().collect();
    let posa = alphabet
        .iter()
        .position(|&x| x == wa)
        .unwrap();

    let posb = alphabet
        .iter()
        .position(|&x| x == wb)
        .unwrap();        

    if ((posb as i32) -(posa as i32)) <= 1 {
        //println!("checking {} to {} -> OK",a,b);
        true
    } else {
        //println!("checking {} to {} -> not OK",a,b);
        false
    }
}

fn check_neighbours(p: Point, map: &Vec<Vec<char>>, mut distances: &mut Vec<Vec<i32>>, mut end_found: &bool) -> 
    (Vec<Point>, bool) {
    let mut out: Vec<Point> = vec![];
    let mut end_found = false;

    //up
    if p.y>0 {
        if can_go(map[p.x as usize][p.y as usize], map[p.x as usize][(p.y-1) as usize]) &&
           (distances[p.x as usize][(p.y-1) as usize] == 9999 
            || distances[p.x as usize][(p.y-1) as usize] > (p.dist+1)) {
            if map[p.x as usize][(p.y-1) as usize] == '&' {
                end_found = true;
            }
            out.push(Point::new(p.x,p.y-1,p.dist+1));
            distances[p.x as usize][(p.y-1) as usize]=p.dist+1;
        }
    }

    //down
    if p.y< (map[0].len() as i32) -1 {
        if can_go(map[p.x as usize][p.y as usize], map[p.x as usize][(p.y+1) as usize]) && 
            (distances[p.x as usize][(p.y+1) as usize] == 9999 
            || distances[p.x as usize][(p.y+1) as usize]>(p.dist+1)){
                if map[p.x as usize][(p.y+1) as usize] == '&' {
                    end_found = true;
                }                
            out.push(Point::new(p.x,p.y+1,p.dist+1));
            distances[p.x as usize][(p.y+1) as usize]=p.dist+1;
        }
    }

    //left
    if p.x>0 {
        if can_go(map[p.x as usize][p.y as usize], map[(p.x-1) as usize][p.y as usize]) && 
            (distances[(p.x-1) as usize][p.y as usize] == 9999 
            || distances[(p.x-1) as usize][p.y as usize]> (p.dist+1)){
                if map[(p.x-1) as usize][p.y as usize] == '&' {
                    end_found = true;
                }                
            out.push(Point::new(p.x-1,p.y,p.dist+1));
            distances[(p.x-1) as usize][p.y as usize]=p.dist+1;
        }
    }

    //right
    if p.x< (map.len() as i32) -1 {
        if can_go(map[p.x as usize][p.y as usize], map[(p.x+1) as usize][p.y as usize]) && 
            (distances[(p.x+1) as usize][p.y as usize] == 9999  
            || distances[(p.x+1) as usize][p.y as usize] > (p.dist+1) ) {
                if map[(p.x+1) as usize][p.y as usize] == '&' {
                    end_found = true;
                }                
            out.push(Point::new(p.x+1,p.y,p.dist+1));
            distances[(p.x+1) as usize][p.y as usize]=p.dist+1;
        }
    }
    if end_found {
        //println!("end at: {}", p.dist);
    }
    (out, end_found)
}

fn main() {

    let lines = lines_from_file("12.txt").expect("Could not load lines");
    let mut map: Vec<Vec<char>> = vec![vec![' '; lines.len()]; lines[0].len()];
    
    let mapx=lines.len();
    let mapy=lines[0].len();

    let mut x = 0;
    let mut y = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        x = 0;
        for char in chars {
            map[x][y]=char;
            x += 1;
        }
        y += 1;
    }

    let mut startx =0;
    let mut starty =0;


    let mut all_starts: Vec<Point> = vec![];

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '$' {
                //startx = x;
                //starty = y;
                all_starts.push(Point::new(x as i32,y as i32,9999));
            }         
            if map[x][y] == 'a' {
                //startx = x;
                //starty = y;
                all_starts.push(Point::new(x as i32,y as i32 ,9999));
            }         
        }
    }

    for start in 0..all_starts.len() {

        let mut distances: Vec<Vec<i32>> = vec![vec![9999; mapx.clone()]; mapy.clone()];
        let mut Q: Vec<Point> = vec![];

        startx = all_starts[start].x.clone() as usize;
        starty = all_starts[start].y.clone() as usize;
        //println!("checking start {} {} ",startx, starty);
    
    //println!("start {} {}", startx, starty);
    //println!("end   {} {}", endx, endy);    

    distances[startx][starty]=0;
    Q.push(Point::new(startx as i32, starty as i32, 0));
    
    let mut end_found: bool = false;

    loop {

        if end_found {
            println!("end after: {}",Q[0].dist);
            all_starts[start].dist=Q[0].dist;
            break;
        }
        if Q.len() == 0 {
            break;
        }
        //println!("Q len: {}, shortest {}", Q.len(), Q[0].dist);
        let current: Point = Q.pop().unwrap();
        let mut new_ones:Vec<Point>;
         (new_ones, end_found) = 
            check_neighbours(current, &map, &mut distances, &mut end_found);
        //println!("add: {}", new_ones.len());
        for _ in 0..new_ones.len() {
            Q.push(new_ones.pop().unwrap());
        }

        Q.sort_by_key(|q| q.dist); 
        Q.reverse();

    }
}

    all_starts.sort_by_key(|q| q.dist);
    
    println!("shortest is: {}",all_starts[0].dist);
    // for x in 0..Q.len() {
    //     println!("{:?}",Q[x]);
    // }

    // for x in 0..distances.len() {
    //     for y in 0..distances[0].len() {
    //         print!("{:04} ", distances[x][y]);
    //     }
    //     println!("");
    // }


}
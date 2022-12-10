use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use substring::Substring;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y:i32 )->Self {
        Self {x,y}
    }

    fn still_touch(&self, x: i32, y: i32) -> bool {
        if (self.x-x).abs() <= 1 && (self.y-y).abs() <= 1 {
            true
        }else {
            false
        }
    }

    fn move_point(mut self, direction: String) -> Self {
        //println!("before: {} , {}",self.x ,self.y);
        //println!("dir: {}",direction);
        match direction.as_str() {
            "U" => { self.y -= 1; },
            "D" => { self.y += 1; },
            "L" => { self.x -= 1; },
            "R" => { self.x += 1; },
            _ => { }
        }
        //println!("after: {} , {}",self.x ,self.y);
        self
    }

    fn follow(mut self, head: &Point, prev_head: Point) -> Self {
        if (self.x-head.x).abs() > 1 {
            //self.x += head.x-self.x;
            //self.y = head.y;
            self.x = prev_head.x;
            self.y = prev_head.y;
        }
        if (self.y-head.y).abs() > 1 {
            //self.y += head.y-self.y;
            //self.x = head.x;
            self.x = prev_head.x;
            self.y = prev_head.y;

        }        
        //println!("y pos: {} , {}", self.x, self.y);
        self
    }
}


fn main() {

    let lines = lines_from_file("09.txt").expect("Could not load lines");
    let horiz: i32 = 1200;
    let vert: i32 = 1200;
    let mut map: Vec<Vec<i32>> = vec![vec![0; horiz as usize]; vert as usize];
    let mut map2: Vec<Vec<i32>> = vec![vec![0; horiz as usize]; vert as usize];

    let mut head: Point = Point::new(horiz/2, vert/2);
    let mut tail: Point = Point::new(horiz/2, vert/2);

    let mut direction: String;
    let mut distance: i32;

    let mut rope: Vec<Point> = vec![];
    for x in 0..10 {
        rope.push(Point::new(horiz/2, vert/2));
    }    

    let mut cnt:i32=0;
    for line in lines {
        cnt +=1;
        distance = line.substring(2,line.len()).parse::<i32>().unwrap();
        direction = line.substring(0,1).to_string();

        for x in 0..distance {
            let head_before = Point::new(head.x.clone(),  head.y.clone());
            head = head.move_point(direction.clone());
            tail = tail.follow(&head, head_before);
            map[tail.x as usize][tail.y as usize] = 1;
        }


        for x in 0..distance {
            
            let mut before: Vec<Point> = vec![];
            for j in 0..10 {
                before.push(rope[j].clone());
            }
            for elem in 0..10 {
                //let mut before: Point = Point::new(0,0);

                //if elem > 0 {
                //before = Point::new(rope[elem].x.clone(),  rope[elem].y.clone());
                //}

                if elem == 0 {      
                    
                    rope[elem] = rope[elem].move_point(direction.clone());
                } else {
                    rope[elem] = rope[elem].follow(&rope[elem-1], before[elem-1]);
                }

                if elem == 9 {
                    map2[rope[9].x as usize][rope[9].y as usize] = 1;
                }
                
            }
            
            println!("tails pos after step {}: {}:{} ", cnt, rope[9].x, rope[9].y);
        }




    }

    let mut count=0;
    for x in 0..horiz {
        for y in 0..vert {
            if map[x as usize][y as usize] == 1 {
                count += 1;
            }
        }
    }
    println!("cnt: {}",count);

    let mut count2: i32 =0;
    for x in 0..horiz {
        for y in 0..vert {
            if map2[x as usize][y as usize] == 1 {
                count2 += 1;
            }
        }
    }
    println!("cnt2: {}",count2);
}
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}




fn main() {

    let lines = lines_from_file("08.txt").expect("Could not load lines");
    let horiz = lines[0].len();
    let vert = lines.len();
    let mut array: Vec<Vec<i32>> = vec![vec![-1; horiz]; vert];
    let mut visible_array: Vec<Vec<bool>> = vec![vec![false; horiz]; vert];
    
    let mut y =0;   
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut x =0;
        for c in chars {
            let w = c.to_digit(10).unwrap();
            array[x][y] = w as i32;
            x += 1;
        }
        y += 1;
    }
    
    for x in 0..horiz {
        for y in 0..vert {
            print!("{}", array[x][y])
        }
        println!("");
    }
    //println!("{:?}", array);

    let mut visible = 0;
    for x in 0..horiz {
        let mut highest: i32 = -1;
        for y in 0..vert {
            //println!("checking {}",y);
            if array[x][y] > highest && visible_array[x][y] == false {
                if array[x][y] == 4 {
                    println!("tu1");
                }
                visible += 1;
                visible_array[x][y] = true;
                highest = array[x][y];
            } else if array[x][y] > highest{
                highest = array[x][y];
            }
        }

        highest = -1;
        for y in (0..vert).rev() {
            //println!("checking {}",y);
            if array[x][y] > highest  && visible_array[x][y] == false {
                if array[x][y] == 4 {
                    println!("tu2, {}, {}, highest {}",x,y, highest);
                }

                visible += 1;
                visible_array[x][y] = true;
                highest = array[x][y];
            } else if array[x][y] > highest{
                highest = array[x][y];
            }
        }

    }

    for y in (0..vert) {
        let mut highest: i32 = -1;
        for x in 0..horiz {
            //println!("checking {}",y);
            if array[x][y] > highest  && visible_array[x][y] == false {
                if array[x][y] == 4 {
                    println!("tu3");
                }                
                visible += 1;
                visible_array[x][y] = true;
                highest = array[x][y];
            } else if array[x][y] > highest{
                highest = array[x][y];
            }
        }

        highest = -1;
        for x in (0..horiz).rev() {
            //println!("checking {}",y);
            if array[x][y] > highest  && visible_array[x][y] == false {
                if array[x][y] == 4 {
                    println!("tu4");
                }
                visible += 1;
                visible_array[x][y] = true;
                highest = array[x][y];
            } else if array[x][y] > highest{
                highest = array[x][y];
            }
        }

    }

    println!("visible: {}",visible);
    for x in 0..horiz {
        for y in 0..vert {
            if visible_array[x][y] == true {
                print!("X");
            } else {
                print!(" ");
            }
            
        }
        println!("");
    }
}
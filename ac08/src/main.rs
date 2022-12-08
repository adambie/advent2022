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
    let mut score_array: Vec<Vec<usize>> = vec![vec![0; horiz]; vert];


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

    for x in 0..horiz {
        for y in 0..vert {
            score_array[x][y]=count_visible(x, y, array.clone(),horiz.clone(), vert.clone());
        }
    }    

    let mut max_score: usize =0;
    for x in 0..horiz {
        for y in 0..vert {
            if score_array[x][y]> max_score {
                max_score = score_array[x][y];
            }
        }
    }        
    println!("max score: {}",max_score);


    fn count_visible (x: usize, y:usize, array: Vec<Vec<i32>>, horiz: usize, vert: usize) -> usize {
        let mut points1: usize = 0;
        let mut points2: usize = 0;
        let mut points3: usize = 0;
        let mut points4: usize = 0;
        //println!("----- checks for {} : {} ",x,y);
        //down

        'i_loop: for w in x+1..horiz { 
            //println!("inner check {} : {}",w,y);
            if array[w][y] < array[x][y] {
                points1 += 1;
            }
            else if array[w][y] >= array[x][y] {
                points1 += 1;
                break 'i_loop;
            } 
        }
        

        //up
        
        'ii_loop:  for w in (0..x).rev() { 
            
            if array[w][y] < array[x][y] {
                points2 += 1;
            }
            else if array[w][y] >= array[x][y] {
                points2 += 1;
                break 'ii_loop;
            } 
        }
        

        //right
        
        'iii_loop : for w in y+1..vert { 
            
            if array[x][w] < array[x][y] {
                points3 += 1;
            }
            else if array[x][w] >= array[x][y] {
                points3 += 1;
                break 'iii_loop;
            } 
        }
        

        //left
        
        'iiii_loop : for w in (0..y).rev() { 
            
            if array[x][w] < array[x][y] {
                points4 += 1;
            }
            else if array[x][w] >= array[x][y] {
                points4 += 1;
                break 'iiii_loop;
            } 
        }

        if x==2 && y ==3 {
            println!("TEST:{} {} {} {} {}",array[x][y], points1,points2,points3,points4);
        }
        
        points1 * points2 * points3 * points4

    }


}
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
//use substring::Substring;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn main() {

    let lines = lines_from_file("10.txt").expect("Could not load lines");
    let mut comm: String;
    let mut value: String;

    let mut x: i32 =0; 
    let mut cycles: Vec<i32> =vec![];

    cycles.push(1);

    for line in lines {
        match line.split_once(' ') {
            Some((first, second)) => {
                comm = first.to_string();
                value = second.to_string();
            }
            None => {
                comm = line;
                value = "".to_string();
            }            
        }

        match comm.as_str() {
            "noop" => {
                cycles.push(cycles[cycles.len()-1].clone());
            }

            "addx" => {
                cycles.push(cycles[cycles.len()-1].clone());
                cycles.push(cycles[cycles.len()-1].clone() + value.trim().parse::<i32>().unwrap());
            }

            _ => {}
        }

    }
   
    let result = cycles[19] * 20 +
        cycles[59] * 60 +
        cycles[99] * 100 +
        cycles[139] * 140 +
        cycles[179] * 180 +
        cycles[219] * 220;   

    println!("result: {}", result); 
    
    
    let mut screen: Vec<char> = vec![];
    let mut cnt: i32 = 0;
    for x in 0..cycles.len() {

        let value: char;
        if (cycles[x]-cnt as i32).abs() <= 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        cnt +=1;
        if cnt == 40 {
            cnt = 0;
        }
        
    }
    let mut cnt: i32 = 0;
    for x in 0..screen.len() {
        print!("{}",screen[x]);
        cnt += 1;
        if cnt == 40 {
            println!("");
            cnt = 0;
        }
    }
}
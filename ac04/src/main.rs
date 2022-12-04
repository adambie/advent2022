use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {

    let mut _number: i32 = 0;
    let mut _number2: i32 = 0;
    let lines = lines_from_file("04.txt").expect("Could not load lines");
    for line in lines {
        let mut split = line.split(",");
        let pair: Vec<&str> = split.collect();

        split = pair[0].split("-");
        let first: Vec<&str> = split.collect();

        split = pair[1].split("-");
        let second: Vec<&str> = split.collect();

        let first_lo: i32 = first[0].parse().unwrap();
        let first_hi: i32 = first[1].parse().unwrap();
        let second_lo: i32 = second[0].parse().unwrap();
        let second_hi: i32 = second[1].parse().unwrap();
        
        
        if (first_lo <= second_lo && first_hi >= second_hi ) ||
           (first_lo >=second_lo && first_hi <= second_hi) {
            _number += 1;
        }

        if (second_lo>=first_lo && second_lo<=first_hi ) ||
           (second_hi>=first_lo && second_hi<=first_hi ) ||
           (first_lo>=second_lo && first_lo<=second_hi ) ||
           (first_hi>=second_lo && first_hi<=second_hi )             
           
        {
            _number2 += 1;
        }        
        
    }
    
    println!(" number: {}",_number);
    println!(" number2: {}",_number2);
}

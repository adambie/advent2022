use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let mut cals: Vec<i64> = vec![];

    let lines = lines_from_file("01.txt").expect("Could not load lines");
    let mut sum: i64 = 0;
    for line in lines {
        if line == "" {
            cals.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i64>().unwrap();
        }
    }
    cals.sort();
    println!("top is {}", cals[cals.len()-1]);
    println!("top3 are {}", cals[cals.len()-1] + cals[cals.len()-2] + cals[cals.len()-3]);
}

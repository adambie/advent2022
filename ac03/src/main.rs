use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use array_tool::vec::Intersect;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn getPrio (line: &String) -> u64  {
    let mut res: u64 = 0;
    
    let allvec: Vec<char> = line.chars().collect();
    let left: Vec<char> = Vec::from_iter(allvec[0..allvec.len()/2].iter().cloned());
    let right: Vec<char> = Vec::from_iter(allvec[allvec.len()/2..allvec.len()].iter().cloned());
    //println!("all: {:?}, left {:?}, right {:?}",allvec, left, right);

    res = getPoints(getCommon(left,right));
    res
}

fn getCommon (left: Vec<char>, right: Vec<char>) -> char {
    let mut found: char;

    let mut common: Vec<char> = left.intersect(right);

    found = common[0];
    found
}

fn getPoints (what : char) -> u64 {
    
    let seq = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let index_element = seq.find(what).unwrap();

    let points: u64 = index_element.try_into().unwrap();
    points + 1
}

fn main() {

    let mut priosum: u64 = 0;

    let lines = lines_from_file("03.txt").expect("Could not load lines");
    for line in lines {

        priosum += getPrio(&line);

    }
    println!("priosum: {}",priosum);
}

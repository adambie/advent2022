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

fn findCommon (group: Vec<String>) -> char {
    let mut found: char;

    let a: Vec<char> = group[0].chars().collect();
    let b: Vec<char> = group[1].chars().collect();
    let c: Vec<char> = group[2].chars().collect();

    let common: Vec<char>  = a.intersect(b.intersect(c));
    found = common[0];
    found
}

fn main() {

    let mut priosum: u64 = 0;
    let mut priosum2: u64 = 0;

    let lines = lines_from_file("03.txt").expect("Could not load lines");
    let mut count: i32 =0;
    let mut linesgroup: Vec<String> ;

    linesgroup = Vec::new();
    for line in lines {
        priosum += getPrio(&line);
        
        linesgroup.push(line);

        if linesgroup.len() == 3 {
            println!("group: {:?}",linesgroup.clone());
            priosum2 +=getPoints(findCommon(linesgroup.clone()));
            linesgroup = Vec::new();
        }       
        

    }
    println!("priosum: {}",priosum);
    println!("priosum2: {}",priosum2);

}

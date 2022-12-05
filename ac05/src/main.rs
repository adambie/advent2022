use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {

    let mut st1: Vec<char> = vec!['L','N','W','T','D'];
    let mut st2: Vec<char> = vec!['C','P','H'];
    let mut st3: Vec<char> = vec!['W','P','H','N','D','G','M','J'];
    let mut st4: Vec<char> = vec!['C','W','S','N','T','Q','L'];
    let mut st5: Vec<char> = vec!['P','H','C','N'];
    let mut st6: Vec<char> = vec!['T','H','N','D','M','W','Q','B'];
    let mut st7: Vec<char> = vec!['M','B','R','J','G','S','L'];
    let mut st8: Vec<char> = vec!['Z','N','W','G','V','B','R','T'];
    let mut st9: Vec<char> = vec!['W','G','D','N','P','L'];

    

    let mut all = Vec::new();

    all.push(st1);
    all.push(st2);
    all.push(st3);
    all.push(st4);
    all.push(st5);
    all.push(st6);
    all.push(st7);
    all.push(st8);
    all.push(st9);

    let lines = lines_from_file("05.txt").expect("Could not load lines");
    for line in lines {
        let mut instruction: String = line.replace("move", "");
        let mut instruction2: String = instruction.replace("from", ",");
        let mut instruction3: String = instruction2.replace("to", ",");

        let mut split = instruction3.split(",");
        let ins = split.collect::<Vec<&str>>();
    

        let howmany: i32 = ins[0].trim().parse::<i32>().unwrap();
        let from: usize = ins[1].trim().parse::<usize>().unwrap();
        let to: usize = ins[2].trim().parse::<usize>().unwrap();

        //print!(" howmany: {}", howmany);
        //print!( " from {}", from);
        //println!(" to {}", to);
        let mut work: Vec<char> = vec![];
        for n in 1..howmany+1 {
            let x: char = all[from-1].pop().unwrap();
            work.push(x);
        }
        for n in 1..howmany+1 {
            let x: char = work.pop().unwrap();
            all[to-1].push(x);
        }
        println!(" first : {:?}", all[from-1]);
        println!(" second : {:?}", all[to-1]); 

    }
    for n in 0..all.len() {
        print!("{:?}",all[n].pop().unwrap());
    }
}
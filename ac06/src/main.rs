use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use array_tool::vec::Uniq;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn isUniqe (inp : Vec<char>) -> bool {
    let mut outcome : bool = false;
    let filtered = inp.unique();
    if filtered.len() == inp.len() {
        outcome = true; 
    }

    outcome
}


fn main() {

    let lines = lines_from_file("06.txt").expect("Could not load lines");
    let mut key: Vec<char> = vec![];
    
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut cnt: i32 = 0;
        for c in chars {
            cnt +=1 ;
            print!("{}",c);
            if key.len() == 14 {
                if isUniqe(key.clone()) {
                    println!("got uniqe in {} steps, key {:?}", cnt-1, key);
                    return;
                } else {
                    key.remove(0);
                    key.push(c.clone());
                    //key[0] = key[1];
                    //key[1] = key[2];
                    //key[2] = key[3];
                    //key[3] = c.clone();
                }
            } else if key.len() < 14 {
                println!("push step {}", cnt);
                key.push(c.clone());
            }
        }

    }
}
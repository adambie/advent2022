use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn getResult (a: char, b: char) -> i32  {
    let mut res: i32 = 0;

    if b == 'X' {   //rock
        if a == 'A' { res = 4; }
        else if a == 'B' { res = 1;}
        else if a == 'C' { res = 7;};
    }

    if b == 'Y' {   //paper
        if a == 'A' { res = 8;}
        else if a == 'B' { res = 5; }
        else if a == 'C' { res = 2; };
    }

    if b == 'Z' {   //scissors
        if a == 'A' { res = 3; }
        else if a == 'B' { res = 9; }
        else if a == 'C' { res = 6; };
    }    

    res
}

fn getResult2 (a: char, b: char) -> i32  {
    let mut res: i32 = 0;


    //rock
    //paper
    //scissors

    if b == 'X' {   //loose
        if a == 'A' { res = 3; }
        else if a == 'B' { res = 1;}
        else if a == 'C' { res = 2;};
    }

    if b == 'Y' {   //draw
        if a == 'A' { res = 4;}
        else if a == 'B' { res = 5; }
        else if a == 'C' { res = 6; };
    }

    if b == 'Z' {   //win
        if a == 'A' { res = 8; }
        else if a == 'B' { res = 9; }
        else if a == 'C' { res = 7; };
    }    

    res
}


fn main() {
    let mut first: char;
    let mut second :char;
    let mut score: i32 = 0;
    let mut score2: i32 = 0;


    let lines = lines_from_file("ac02.txt").expect("Could not load lines");
    for line in lines {
        first = line.chars().nth(0).unwrap();
        second = line.chars().nth(2).unwrap();

        score += getResult(first, second);
        score2 += getResult2(first, second);
    }
    println!("score is {}", score);
    println!("2nd score is {}", score2);
}

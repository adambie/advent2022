use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(Debug)]
struct CodeNumber {
    number: i32,
    place: usize
}

fn main() {

    let lines = lines_from_file("20.txt").expect("Could not load lines");
    let mut numbers: Vec<CodeNumber> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        numbers.push(
            CodeNumber {
                number: line.parse::<i32>().unwrap(),
                place: idx                  
            }
        );
    }  

    //-------------------------------part1
    let list_len: i32 = (numbers.len()-1 ) as i32;

    for current in 0..numbers.len() {
        let curr_idx = numbers.iter().position(|x| x.place == current ).unwrap();
        
        let mut new_idx: i32 = curr_idx as i32 + numbers[curr_idx].number ;
        let temp = numbers.remove(curr_idx);
        
        new_idx = ((new_idx % list_len) + list_len) % list_len;
        numbers.insert( new_idx as usize, temp);

    }

    //get sum of items at 1000 2000 3000 pos
    let results = vec![1000,2000,3000];
    let startpos =  numbers.iter().position(|x| x.number == 0 ).unwrap();
    println!("val {:?} pos {}",numbers[startpos], startpos);

    let mut sum=0;
    for result in results {
        let mut idx = ((result + startpos) as i32 % list_len) as usize;
        if idx>numbers.len(){
            //idx += 1;
            loop {
                if idx>numbers.len(){
                    idx -= numbers.len();
                } else {
                    break;
                }
            }
        }
        println!("res: {:?}", numbers[idx]);
        sum += numbers[idx].number;

    }
    println!("sum of part1: {}", sum);

    //-------------------------------part2



    
}

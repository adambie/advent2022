use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(Debug, Clone, Copy)]
struct CodeNumber {
    number: i64,
    place: usize
}

fn main() {

    let lines = lines_from_file("20.txt").expect("Could not load lines");
    let mut numbers: Vec<CodeNumber> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        numbers.push(
            CodeNumber {
                number: line.parse::<i64>().unwrap(),
                place: idx                  
            }
        );
    }  

    let mut numbers2 = numbers.clone();

    //-------------------------------part1
    let list_len: i64 = (numbers.len()-1 ) as i64;

    for current in 0..numbers.len() {
        let curr_idx = numbers.iter().position(|x| x.place == current ).unwrap();
        
        let mut new_idx: i64 = curr_idx as i64 + numbers[curr_idx].number ;
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
        let mut idx = ((result + startpos) as i64 % list_len) as usize;
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
    for x in 0..numbers2.len() {
        numbers[x].number *= 811589153;
    }

    for _ in 0..10 {
        for current2 in 0..numbers2.len() {
            let curr_idx = numbers2.iter().position(|x| x.place == current2 ).unwrap();
            
            let mut new_idx: i64 = curr_idx as i64 + numbers2[curr_idx].number ;
            let temp = numbers2.remove(curr_idx);
            
            new_idx = ((new_idx % list_len) + list_len) % list_len;
            numbers2.insert( new_idx as usize, temp);
    
        }        
    }
    //get sum of items at 1000 2000 3000 pos
    let results = vec![1000,2000,3000];
    let startpos =  numbers.iter().position(|x| x.number == 0 ).unwrap();
    println!("val {:?} pos {}",numbers[startpos], startpos);

    let mut sum=0;
    for result in results {
        let mut idx = ((result + startpos) as i64 % list_len) as usize;
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

}

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    collections::HashMap,
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

    // for (idx, number) in numbers.iter().enumerate() {
    //     println!("{:?} at pos {}",number,idx);
    // }

    //-------------------------------part1
    let list_len = numbers.len();

    for current in 0..numbers.len() {
        // println!("moving item {}", current);
        let curr_idx = numbers.iter().position(|x| x.place == current ).unwrap();
        // println!("curridx:{}", curr_idx);
        let mut new_idx: i32 = curr_idx as i32 + numbers[curr_idx].number ;
        // println!("newidx:{}", new_idx);
        if numbers[curr_idx].number < 0{
            new_idx -= 1;
        } else if new_idx>list_len as i32{
            new_idx += 1;
        }
        // println!("moving item {} pos {} into {}",numbers[curr_idx].number, current, new_idx);
        let temp = numbers.remove(curr_idx);
        if new_idx>0 {
        loop {
            if new_idx>list_len as i32 {
                new_idx -= list_len as i32;
            } else {
                break;
            } 
        }
        // println!("recalcidx:{}",new_idx);

        } else if new_idx<0 {
            loop {
                if new_idx<0  {
                    new_idx += list_len as i32;
                } else {
                    break;
                } 
            }   
        }
        if new_idx<=numbers.len() as i32 {
            numbers.insert(new_idx as usize, temp);
        } else {
            numbers.push(temp);
        }
        // println!("after");
        // for (idx, number) in numbers.iter().enumerate() {
        //     println!("{:?} at pos {}",number,idx);
        // }
    }

    //get sum of items at 1000 2000 3000 pos
    let results = vec![1000,2000,3000];
    let startpos =  numbers.iter().position(|x| x.number == 0 ).unwrap();
    println!("val 0 pos {}",startpos);
    println!("{:?}",numbers[(startpos + 1000) as usize]);
    println!("{:?}",numbers[(startpos + 2000) as usize]);
    println!("{:?}",numbers[(startpos + 3000) as usize]);
    for result in results {
        let mut idx = result + startpos;
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

    }

}

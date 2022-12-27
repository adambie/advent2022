use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use substring::Substring;

#[derive(Debug, Clone)]
struct Instruction {
    name: String,
    result: i64,
    parm1val: i64,
    parm2val: i64, 
    parm1: String,
    operation: char,
    parm2: String
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn parse_data() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let lines = lines_from_file("21.txt").expect("Could not load lines");
    for line in lines {
        let name = line.substring(0,4);
        let rest:String = line.substring(6,25).to_string();
        let mut result: i64 = 0;
        let mut parm1 = "".to_string();
        let mut operation: char= ' ';
        let mut parm2 = "".to_string(); 
        
        match rest.trim().parse::<i64>() {
            Ok(value) => {
                result = value;
            }
            Err(_) => {
                parm1 = rest.substring(0,4).to_string();
                match rest.chars().nth(5) {
                    Some(value) => {
                        operation=value;
                    }
                    None => {
                        operation=' ';
                    }

                }
                parm2 = rest.substring(7,11).to_string();
            }
        }

        instructions.push(Instruction {
            name: name.to_string(),
            result: result,
            parm1val: 0,
            parm2val: 0,
            parm1: parm1,
            operation: operation,
            parm2: parm2
        });

    }
    instructions
}



fn look_for (input: String, solved: &Vec<Instruction>) -> (String, i64, bool) {
    let mut param =input.clone();
    let mut paramval = 0;
    let mut found = false;

    for x in 0..solved.len() {
        if solved[x].name == input {
            param = "".to_string();
            paramval = solved[x].result;
            found = true;
            break;
        }
    }
    // println!("search returns {} {} {}",input, paramval, found);
    (param, paramval, found)
}

fn clean_unsolved (to_remove: Vec<String>, unsolved: &mut Vec<Instruction> ) {
    for item in to_remove {
        let idx = unsolved.iter().position(|x| x.name == item.to_string() ).unwrap();
        unsolved.remove(idx);
    }
}

fn main() {

    let instructions = parse_data();
    let mut solved: Vec<Instruction> = Vec::new();
    let mut unsolved: Vec<Instruction> = Vec::new();

    for instruction in instructions{ 
        if instruction.parm1.len()==0 && instruction.parm2.len()==0 {
            solved.push(instruction);
        } else {
            unsolved.push(instruction);
        }
    }

    // for x in 0..solved.len(){
    //     println!("solved: {:?}",solved[x]);
    // }

    let mut root_found = false;
    loop {


        if unsolved.len() == 0 || root_found {
            println!("found end");
            break;
            
        }

        let mut to_remove: Vec<String> = Vec::new();

        for x in 0..unsolved.len() {
            // println!("solving {:?}",unsolved[x].clone());
            let mut solved1 = true;
            let mut solved2 = true;
            if unsolved[x].parm1.len()>0 {
                (unsolved[x].parm1, 
                 unsolved[x].parm1val, 
                 solved1) = look_for(unsolved[x].parm1.clone(), &solved);
            } 
            if unsolved[x].parm2.len()>0 {
                (unsolved[x].parm2, 
                 unsolved[x].parm2val, 
                 solved2) = look_for(unsolved[x].parm2.clone(), &solved);
            } 
            if solved1 && solved2 {
                match unsolved[x].operation {
                    '+' => {
                        unsolved[x].result = unsolved[x].parm1val + unsolved[x].parm2val;
                    },
                    '-' => {
                        unsolved[x].result = unsolved[x].parm1val - unsolved[x].parm2val;
                    },
                    '*' => {
                        unsolved[x].result = unsolved[x].parm1val * unsolved[x].parm2val;
                    },
                    '/' => {
                        unsolved[x].result = unsolved[x].parm1val / unsolved[x].parm2val;
                    },
                    _ => {}
                }
                if unsolved[x].name == "root" {
                    root_found = true;
                    println!("root found: {:?}", unsolved[x]);
                }
                solved.push(unsolved[x].clone());
                to_remove.push(unsolved[x].name.clone());

            }
        }

        clean_unsolved(to_remove, &mut unsolved);

        // for x in 0..to_remove.len() {
        //     println!("removed solved {:?}",to_remove[x]);
        //     // unsolved.remove(to_remove[x]);
            
        // }

        // root_found =true;
        // println!("after 1st");
        // for x in 0..solved.len(){
        //     println!("solved: {:?}",solved[x]);
        // }
    }


}
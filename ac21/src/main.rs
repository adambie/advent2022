use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use substring::Substring;

#[derive(Debug, Clone)]
struct Instruction {
    name: String,
    result: i32,
    parm1val: i32,
    parm2val: i32, 
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
        let mut result: i32 = 0;
        let mut parm1 = "".to_string();
        let mut operation: char= ' ';
        let mut parm2 = "".to_string(); 
        
        match rest.trim().parse::<i32>() {
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



fn look_for (input: String, solved: &Vec<Instruction>) -> (String, i32, bool) {
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
    println!("search returns {} {} {}",input, paramval, found);
    (param, paramval, found)
}

fn main() {

    let mut instructions = parse_data();
    let mut solved: Vec<Instruction> = Vec::new();
    let mut unsolved: Vec<Instruction> = Vec::new();

    for instruction in instructions{ 
        if instruction.parm1.len()==0 && instruction.parm2.len()==0 {
            solved.push(instruction);
        } else {
            unsolved.push(instruction);
        }
    }

    for x in 0..solved.len(){
        println!("solved: {:?}",solved[x]);
    }

    let mut root_found = false;
    loop {
        if unsolved.len() == 0 || root_found {
            println!("found end");
            break;
            
        }

        let mut to_remove: Vec<String> = Vec::new();

        for x in 0..unsolved.len() {
            // println!("{:?}",unsolved[x].clone());
            let mut onesolved = true;
            if unsolved[x].parm1.len()>0 {
                (unsolved[x].parm1, 
                 unsolved[x].parm1val, 
                 onesolved) = look_for(unsolved[x].parm1.clone(), &solved);
            } 
            if unsolved[x].parm2.len()>0 {
                (unsolved[x].parm2, 
                 unsolved[x].parm2val, 
                 onesolved) = look_for(unsolved[x].parm2.clone(), &solved);
            } 
            if onesolved {
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

        for x in 0..to_remove.len() {
            println!("removed solved {:?}",to_remove[x]);
            // unsolved.remove(to_remove[x]);
            
        }

    }


}
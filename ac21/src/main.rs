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

fn get_replacement (input: String, instructions: &Vec<Instruction>) -> String {
    let mut out = "".to_string();
    for x in 0..instructions.len() {
        if instructions[x].name == input {
            let mut p1 = "".to_string();
            let mut p2 = "".to_string();

            if instructions[x].name =="humn".to_string() {
                out="humn".to_string();
                break;
            }
            // println!("instr: {:?}",instructions[x].clone());
            if instructions[x].parm1.len()>0 && instructions[x].parm2.len()>0 {
            if instructions[x].parm1.len()>0 {
                p1 = "#".to_string() + instructions[x].parm1.trim();
            } else {
                p1 = instructions[x].parm1val.to_string();
            }
            if instructions[x].parm2.len()>0 {
                p2 = "#".to_string() + instructions[x].parm2.trim();
            } else {
                p2 = instructions[x].parm2val.to_string();
            }               

            

            out="(".to_string()
                + &p1
                + &instructions[x].operation.to_string()
                + &p2
                +&")".to_string();
        } else {
            out = instructions[x].result.to_string();
        }
            // println!("solved {} into {}", input, &out);
            break;
        }
    }
    out
}

fn clean_unsolved (to_remove: Vec<String>, unsolved: &mut Vec<Instruction> ) {
    for item in to_remove {
        let idx = unsolved.iter().position(|x| x.name == item.to_string() ).unwrap();
        unsolved.remove(idx);
    }
}

fn get_pair (inp: Vec<Instruction>) -> (String,String) {
    let  mut left ="".to_string();
    let  mut right = "".to_string();
    for x in 0..inp.len(){
        if inp[x].name == "root".to_string() {
            left=inp[x].parm1.clone();
            right=inp[x].parm2.clone();
        }
    }
    (left, right)
}

fn set_humn(solved: &mut Vec<Instruction>, val: i64) {
    let idx = solved.iter().position(|x| x.name == "humn".to_string() ).unwrap();
    let work = solved.remove(idx);
    solved.push(Instruction {
        name: work.name,
        result: val,
        parm1val: 0,
        parm2val: 0,
        parm1: "".to_string(),
        operation: ' ',
        parm2: "".to_string()
    });

}

fn get_instr(start: &String, instuctions: &Vec<Instruction>) -> String {
    let mut instr = "#".to_string()+&start.to_string();

    loop {
        
        let to_find = instr.chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i)
        .collect::<Vec<_>>(); 
        
        // println!("instr: {} with {} elements to solve",&instr, to_find.len());
        if to_find.len() == 0 {
            break;
        }

        let mut replacements: Vec<String> = vec![];
        for c in 0..to_find.len() {
            let item = instr.substring(to_find[c]+1, to_find[c]+5);
            // println!("resolving {}",item);
            let replacement = get_replacement(item.to_string(), &instuctions);
            replacements.push(replacement);
        }

        let parts: Vec<&str> = instr.split("#").collect();
        // println!("parts: {:?}",parts);
        let mut work="".to_string();
        if parts.len() > 0 {
            work = parts[0].to_string();
            for (idx, part) in parts.into_iter().enumerate() {
                if idx>0 {
                    work = work.clone().trim().to_string() + &replacements[idx-1] + part.substring(4, part.len());
                }
            }
            instr = work;
        }
    }
    instr
}

fn part1() {
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
    }

}

fn simplify(inp:String, num: i64) -> String {
    let mut out = "".to_string();

    let mut work = inp.clone();

    loop {
    //find first ) then evaluate if possible
    let idx_end: usize;
    match work.find(")"){
        Some(number) => {idx_end=number;}
        None => { 
            out=work;
            break; 
        }
    }
    //let idx_end = work.find(")").unwrap();
    let mut idx_begin =0;
    loop{ 
        idx_begin +=1;
        let chars: Vec<char> = work.chars().collect();
        if chars[idx_end-idx_begin] == '(' {
            let work = idx_begin.clone();
            idx_begin = idx_end-work;
            break;
        }
    }
    let to_eval = work.substring(idx_begin, idx_end+1);
    // println!("to eval: {}", to_eval);

    let work2 = work.substring(0, idx_begin).to_owned() +
                        &evaluate(to_eval, num).to_string() + 
                        work.substring(idx_end+1, work.len());

    // println!("{}",&work2);
    work = work2;
    }
    out
}

fn evaluate(inp: &str, num: i64) -> String {
    let mut out="".to_string();
    let mut left =0;
    let mut right =0;
    let mut oper = ' ';
    let mut result: i64 = 0;
    let sides: Vec<&str> = inp.split(['+','-','/','*'].as_ref()).collect();
    // println!("left: {} right: {}",sides[0], sides[1]);

    if sides[1].to_string().substring(0, sides[1].to_string().len()-1) == "humn".to_string() {
        right = num;
    } else {
        right=sides[1].to_string().substring(0, sides[1].to_string().len()-1).parse::<i64>().unwrap();    
    }

    left=sides[0].to_string().substring(1, sides[0].to_string().len()).parse::<i64>().unwrap();
    

    let mut oper:char = inp.chars().nth(sides[0].to_string().len()).unwrap();
    // println!("oper: {}",oper);

    match oper {
        '+' => { result = left+right },
        '-' => { result = left-right },
        '*' => { result = left*right },
        '/' => { result = left/right },
        _ => {}
    }
    out = result.to_string();
    // println!("result: {}",&result);
    out
}

fn part2() {
    let instructions = parse_data();
    let mut solved: Vec<Instruction> = Vec::new();
    let mut unsolved: Vec<Instruction> = Vec::new();

    for instruction in instructions.clone(){ 
        if instruction.parm1.len()==0 && instruction.parm2.len()==0 {
            solved.push(instruction);
        } else {
            unsolved.push(instruction);
        }
    }

    let (left, right) = get_pair(instructions.clone());

    let right_instr = get_instr(&right, &instructions);    
    let simple_right=simplify(right_instr, 1).parse::<i64>().unwrap();
    println!("result right: {}",&simple_right);

    let left_instr = get_instr(&left, &instructions);    
    // println!("left: {}",left_instr);
    let mut x = 0;
    let mut direction = 1;
    let mut factor = 100000000000;
    loop {

        //this section is only to compare number I got 3093175982597 with
        //number got from internet solution 3093175982595
        //they both give good result however only 3093175982595 is accepted..
        // x=3093175982597;
        // let simple_left=simplify(left_instr.clone(), x).parse::<i64>().unwrap();
        // println!("result at {} : {}",x,simple_left);
        // x=3093175982595;
        // let simple_left=simplify(left_instr.clone(), x).parse::<i64>().unwrap();
        // println!("result at {} : {}",x,simple_left);
        // break;
        //test end
        
        x += factor * direction;
        
        let simple_left=simplify(left_instr.clone(), x).parse::<i64>().unwrap();
        println!("checked at {} : {}",x,simple_left);
        if simple_left < simple_right && direction == 1 {
            direction *= -1;
            factor = factor/10;
        }
        if simple_left > simple_right && direction == -1 {
            direction *= -1;
            factor = factor/10;
        }
        if simple_left == simple_right {
            println!("result at {} : {}",x,simple_left);
            break;
        }
    }


}

fn main() {

    // part1();
    part2();
}
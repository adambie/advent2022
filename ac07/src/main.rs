use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use substring::Substring;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
struct TreeNode {
  pub value: i32,
  pub children: Vec<Rc<RefCell<TreeNode>>>,
  pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  pub fn new() -> TreeNode {
    return TreeNode {
      value: 0,
      children: vec![],
      parent: None,
    };
  }

  pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
    self.children.push(new_node);
  }

  pub fn print(&self) -> String {
      return self.value.to_string() + &String::from("[")
        + &self
          .children
          .iter()
          .map(|tn| tn.borrow().print())
          .collect::<Vec<String>>()
          .join(",")
        + "]";
    
  }
  pub fn print_sizes(&self) -> String {
    let mut filtered: i32=0;
    if self.value <= 100000 {
      filtered = self.value;
    }
    return ",".to_string() + &filtered.to_string() + &String::from("\n")
      + &self
        .children
        .iter()
        .map(|tn| tn.borrow().print_sizes())
        .collect::<Vec<String>>()
        .join(",")
      ;
  
}  

pub fn print_all_sizes(&self) -> String {
  let mut filtered: i32=0;
  //if self.value <= 100000 {
    filtered = self.value;
  //}
  return ",".to_string() + &filtered.to_string() + &String::from("\n")
    + &self
      .children
      .iter()
      .map(|tn| tn.borrow().print_all_sizes())
      .collect::<Vec<String>>()
      .join(",")
    ;

}  
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {

    let lines = lines_from_file("07.txt").expect("Could not load lines");   
    
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);

    for line in lines {

        if line.substring(0,4) == "$ cd" {  
            let directory = line.substring(5,40).trim();    
            println!(">>> dir size: {}",current.borrow().value);
            println!("dir: {}", directory);
            if directory == ".." {
              let current_clone = Rc::clone(&current);
              let child_size = current.borrow().value;
              current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
              let mut mut_current = current.borrow_mut();
              mut_current.value += child_size;
            } 
            else if directory != "/" {
              
                let mut child = Rc::new(RefCell::new(TreeNode::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                    mut_child.value = 0;
                }    
                current = child;            
            } 
           
        } 
        else if line.substring(0,4) == "$ ls" {
            
        } 
        else if line.substring(0,4) == "dir " {

        } 
        else {
          
            let mut space = line.find(" ");
            if space != None {
                let size: i32 = line.substring(0,space.unwrap()).parse::<i32>().unwrap();
                //let current_clone = Rc::clone(&current);
                let mut mut_child = current.borrow_mut();
                mut_child.value += size;
                println!("add file: {}",size);      
            }

        }
    }
    println!("{}",root.borrow().print());
    println!("=========================");


    //let mut sizeslist = root.borrow().print_sizes();

    let mut sizeslist = root.borrow().print_all_sizes();

    let mut split = sizeslist.split(",");
    let vec: Vec<&str> = split.collect();
    let mut sizes: Vec<i32>=vec![];
    let mut sumsize:i32 = 0;
    for v in vec {
      match v.trim().parse::<i32>() {
        Ok(n) => {
          sumsize += n;
          sizes.push(n); 
        },
        Err(e) => {sumsize += 0}
      }
      
    }
    println!("sumsize for 1: {}", sumsize);


    
    let mut to_free:i32 = 30000000-  (70000000 - root.borrow().value);
    println!("to free up {}", to_free);
    sizes.sort();

    let mut prev:i32=0;
    for s in sizes {
     //
      if s > to_free && prev == 0 { 
        
        prev=s;
      }
      println!("s {}",s);
    }
    println!("candidate {}", prev);

}
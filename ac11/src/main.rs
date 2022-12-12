use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: String,
    operationby: usize,
    test: usize,
    iftrue :usize,
    iffalse :usize,
    inspected: usize
}

impl Monkey {
    fn process(&mut self) -> (usize, usize) {
      
        if self.items.len()>0 {
            let mut work:usize = 0;
            let mut val: usize = 0 ;

            work = self.operationby.clone();
            if self.operationby == 0 {
                work = self.items[0].clone();
            }

            self.inspected += 1;

            match self.operation.as_str() {
                "*" => { val = self.items[0] * work },
                "+" => {val = self.items[0] + work },
                _ => {}
            }
            
            //val = val / 3 ;
            val %= 9699690;
            self.items.remove(0);

            if val % self.test == 0 {
                return(val, self.iftrue)
            } else {
                return(val, self.iffalse)
            }

            
        }

        return(0,0);
        

    }

    fn receive(&mut self, item: usize)  {
        //println!("beofre add: {:?} ", self.items.clone());
        self.items.push(item);
        //println!("after rem: {:?} ", self.items.clone());
        
    }
}

fn main() {

    let mut monkeys: Vec<Monkey> = vec![];

    // monkeys.push( Monkey {
    //     items: vec![79, 98],
    //     operation: "*".to_string(),
    //     operationby: 19,
    //     test: 23,
    //     iftrue: 2,
    //     iffalse: 3,
    //     inspected: 0
    //     }
    // ); 
    
    // monkeys.push( Monkey {
    //     items: vec![54, 65, 75, 74],
    //     operation: "+".to_string(),
    //     operationby: 6,
    //     test: 19,
    //     iftrue: 2,
    //     iffalse: 0,
    //     inspected: 0
    //     }
    // );
    
    // monkeys.push( Monkey {
    //     items: vec![79, 60, 97],
    //     operation: "*".to_string(),
    //     operationby: 0,
    //     test: 13,
    //     iftrue: 1,
    //     iffalse: 3,
    //     inspected: 0
    //     }
    // );    

    // monkeys.push( Monkey {
    //     items: vec![74],
    //     operation: "+".to_string(),
    //     operationby: 3,
    //     test: 17,
    //     iftrue: 0,
    //     iffalse: 1,
    //     inspected: 0
    //     }
    // );    

    monkeys.push( Monkey {
        items: vec![98, 70, 75, 80, 84, 89, 55, 98],
        operation: "*".to_string(),
        operationby: 2,
        test: 11,
        iftrue: 1,
        iffalse: 4,
        inspected: 0
        }
    ); 
    
    monkeys.push( Monkey {
        items: vec![59],
        operation: "*".to_string(),
        operationby: 0,
        test: 19,
        iftrue: 7,
        iffalse: 3,
        inspected: 0
        }
    );
    
    monkeys.push( Monkey {
        items: vec![77, 95, 54, 65, 89],
        operation: "+".to_string(),
        operationby: 6,
        test: 7,
        iftrue: 0,
        iffalse: 5,
        inspected: 0
        }
    );    

    monkeys.push( Monkey {
        items: vec![71, 64, 75],
        operation: "+".to_string(),
        operationby: 2,
        test: 17,
        iftrue: 6,
        iffalse: 2,
        inspected: 0
        }
    );    

    monkeys.push( Monkey {
        items: vec![74, 55, 87, 98],
        operation: "*".to_string(),
        operationby: 11,
        test: 3,
        iftrue: 1,
        iffalse: 7,
        inspected: 0
        }
    );    
    monkeys.push( Monkey {
        items: vec![90, 98, 85, 52, 91, 60],
        operation: "+".to_string(),
        operationby: 7,
        test: 5,
        iftrue: 0,
        iffalse: 4,
        inspected: 0
        }
    );    
        monkeys.push( Monkey {
        items: vec![99, 51],
        operation: "+".to_string(),
        operationby: 1,
        test: 13,
        iftrue: 5,
        iffalse: 2,
        inspected: 0
        }
    );    

        monkeys.push( Monkey {
        items: vec![98, 94, 59, 76, 51, 65, 75],
        operation: "+".to_string(),
        operationby: 5,
        test: 2,
        iftrue: 3,
        iffalse: 6,
        inspected: 0
        }
    );    

    let absolute_limit: usize = monkeys.iter().map(|m| m.test).product();
    println!("limit: {}",absolute_limit);

    for y in 0..10000 {
        //println!("round {}",&y);
        for x in 0..monkeys.len() {
        //println!("monkey {} items {:?}", x, monkeys[x].items.clone());    
        loop {
            let mut cmonkey = monkeys[x].clone();
            let (item, num) = cmonkey.process();
            monkeys[x]=cmonkey.clone();
            if item == 0 && num == 0 {
                break;
            }
            //println!("monkey {} gives item {} to monkey {}",&x, &item, &num );
            cmonkey = monkeys[num as usize].clone();
            cmonkey.receive(item);
            monkeys[num as usize] = cmonkey.clone();
        }
        
        }   
    
    }

    let mut res: Vec<usize> = vec![];
    for x in 0..monkeys.len() {
        res.push(monkeys[x].inspected.clone());
    }

    res.sort();
    res.reverse();
    for x in 0..res.len() {
    println!("monkey {}", res[x]);
    }

    println!("result: {}", res[0]*res[1]);

}
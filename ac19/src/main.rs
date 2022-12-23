use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
#[derive(PartialEq, Debug,  Clone)]
enum Collector {
    Ore,
    Clay,
    Obsidian,
    Geode
}
#[derive(Debug,  Clone)]
struct Inventory {
    ore_amnt: i32,
    clay_amnt: i32,
    obsidian_amnt: i32,
    geode_amnt: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    construct: Collector
}

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    ore_robot_cost_ore: i32,
    clay_robot_cost_ore: i32,
    obsidian_robot_cost_ore: i32,
    obsidian_robot_cost_clay: i32,
    geode_robot_cost_ore: i32,
    geode_robot_cost_obsidian: i32,
    max_ore_use: i32
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn read_blueprints() -> Vec<Blueprint> {
    let lines = lines_from_file("19.txt").expect("Could not load lines");
    

    let mut ore_robot_cost_ore: i32 = 0;
    let mut clay_robot_cost_ore: i32 = 0;
    let mut obsidian_robot_cost_ore: i32 = 0;
    let mut obsidian_robot_cost_clay: i32 = 0;
    let mut geode_robot_cost_ore: i32 = 0;
    let mut geode_robot_cost_obsidian: i32 = 0;   
    let mut max_ore_use: i32 = 0;   

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for line in lines {
        let words = line.split(" ").collect::<Vec<&str>>();
        ore_robot_cost_ore = words[6].parse::<i32>().unwrap();    
        clay_robot_cost_ore = words[12].parse::<i32>().unwrap();    
        obsidian_robot_cost_ore = words[18].parse::<i32>().unwrap();    
        obsidian_robot_cost_clay = words[21].parse::<i32>().unwrap();    
        geode_robot_cost_ore = words[27].parse::<i32>().unwrap();    
        geode_robot_cost_obsidian = words[30].parse::<i32>().unwrap();    
        max_ore_use = max_ore_use.max(obsidian_robot_cost_ore.max(geode_robot_cost_ore.max(clay_robot_cost_ore)));
        blueprints.push(
                    Blueprint {
                        ore_robot_cost_ore,
                        clay_robot_cost_ore,
                        obsidian_robot_cost_ore,
                        obsidian_robot_cost_clay,
                        geode_robot_cost_ore,
                        geode_robot_cost_obsidian,
                        max_ore_use                        
                    }
        );
        ore_robot_cost_ore = 0;
        clay_robot_cost_ore = 0;
        obsidian_robot_cost_ore = 0;
        obsidian_robot_cost_clay = 0;
        geode_robot_cost_ore = 0;
        geode_robot_cost_obsidian = 0;  
        max_ore_use = 0;                 

        
    }
    blueprints
}


fn find_best_config(mut inventory: Inventory, bleuprint: &Blueprint, mut turns_left:i32 ) -> i32 {

    let mut robot_done = false;
    while robot_done == false && turns_left>0 {
          
        //construct robots
        match inventory.construct {
            Collector::Ore => {
                if bleuprint.ore_robot_cost_ore <= inventory.ore_amnt {
                    inventory.ore_amnt -= bleuprint.ore_robot_cost_ore;
                    robot_done = true;
                }
            }
            Collector::Clay => {
                if bleuprint.clay_robot_cost_ore <= inventory.ore_amnt {
                    inventory.ore_amnt -= bleuprint.clay_robot_cost_ore;
                    robot_done = true;
                }
            }
            Collector::Obsidian => {
                if bleuprint.obsidian_robot_cost_ore <= inventory.ore_amnt 
                && bleuprint.obsidian_robot_cost_clay <= inventory.clay_amnt {
                    inventory.ore_amnt -= bleuprint.obsidian_robot_cost_ore;
                    inventory.clay_amnt -= bleuprint.obsidian_robot_cost_clay;
                    robot_done = true;
                }
            }
            Collector::Geode => {
                if bleuprint.geode_robot_cost_ore <= inventory.ore_amnt 
                && bleuprint.geode_robot_cost_obsidian <= inventory.obsidian_amnt {
                    inventory.ore_amnt -= bleuprint.geode_robot_cost_ore;
                    inventory.obsidian_amnt -= bleuprint.geode_robot_cost_obsidian;
                    robot_done = true;
                }
            }
        }
                //collect goods
                inventory.ore_amnt += inventory.ore_robots;
                inventory.clay_amnt += inventory.clay_robots;
                inventory.obsidian_amnt += inventory.obsidian_robots;
                inventory.geode_amnt += inventory.geode_robots; 

        turns_left -= 1;    

        if robot_done {
            match inventory.construct {
                Collector::Ore =>  {inventory.ore_robots += 1},
                Collector::Clay => {inventory.clay_robots += 1},
                Collector::Obsidian => {inventory.obsidian_robots += 1},
                Collector::Geode => {inventory.geode_robots += 1}
            }
        }
    }

    let mut bestresult = inventory.geode_amnt;    
    if turns_left > 0 {
        //decide what robot to make
        for robot in [Collector::Ore, Collector::Clay, Collector::Obsidian, Collector::Geode] {
            
            if robot == Collector::Geode && inventory.obsidian_robots == 0 {
                continue;
            }
            if robot == Collector::Obsidian && inventory.clay_robots == 0 {
                continue;
            }            
            if robot == Collector::Ore && bleuprint.max_ore_use ==  inventory.ore_robots ||
            robot == Collector::Clay && bleuprint.obsidian_robot_cost_clay ==  inventory.clay_robots ||
            robot == Collector::Obsidian && bleuprint.geode_robot_cost_obsidian ==  inventory.obsidian_robots           
            {
                continue;
            }

            let mut new_inv = inventory.clone();
            new_inv.construct = robot;
            let result = find_best_config(new_inv, &bleuprint, turns_left);
            bestresult = bestresult.max(result);
        }
    }
    bestresult
}

fn main() {

    let blueprints = read_blueprints();

    //part1 
    //---------------------------------------------------------
    let mut total =0;
    for (idx, blueprint)  in blueprints.iter().enumerate() {
        let inventory = Inventory { 
            ore_amnt: blueprint.ore_robot_cost_ore,
            clay_amnt: 0, 
            obsidian_amnt: 0, 
            geode_amnt: 0, 
            ore_robots: 0, 
            clay_robots: 0, 
            obsidian_robots: 0, 
            geode_robots: 0, 
            construct: Collector::Ore
        };

        if (idx+1 == 4) {
            println!("{:?}",blueprint);
        }

        let best_result = find_best_config(inventory.clone(), &blueprint, 24+1);
        println!("best for {} is {}", idx+1, best_result);
        total += (idx as i32 + 1)*best_result;
        
    }
    println!("total result: {}",total);

    //part2 
    //---------------------------------------------------------
    let mut total =1;
    for (idx, blueprint)  in blueprints[0..3].iter().enumerate() {
        let inventory = Inventory { 
            ore_amnt: blueprint.ore_robot_cost_ore,
            clay_amnt: 0, 
            obsidian_amnt: 0, 
            geode_amnt: 0, 
            ore_robots: 0, 
            clay_robots: 0, 
            obsidian_robots: 0, 
            geode_robots: 0, 
            construct: Collector::Ore
        };

        let best_result = find_best_config(inventory.clone(), &blueprint, 32+1);
        println!("best for {} is {}", idx+1, best_result);
        total *= best_result;
        
    }
    println!("total result 2: {}",total);

    //for test
    //---------------------------------------------------------
    // let max = count(1, 3);
    // println!("Hello, world! {}", max);
}

fn count(num: i32, times: i32) -> i32 {
    let arr:Vec<i32>=vec![1,2,3,4];
    let mut maks: i32 = 0;

    if (times-1)>0 {
        for (idx, x) in arr.iter().enumerate(){
            let mut result = count((num*x), (times-1));
            maks = maks.max(result);        
        }
    } else {
        maks = maks.max(num);
    }
    //println!("ret max: {}", maks);
    maks
}

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
#[derive(Debug,  Clone)]
enum Collector {
    Ore,
    Clay,
    Obsidian,
    Geode
    // Skip
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
    geode_robot_cost_obsidian: i32
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn read_blueprints() -> Vec<Blueprint> {
    let lines = lines_from_file("19.txt").expect("Could not load lines");
    

    let mut Tore_robot_cost_ore: i32 = 0;
    let mut Tclay_robot_cost_ore: i32 = 0;
    let mut Tobsidian_robot_cost_ore: i32 = 0;
    let mut Tobsidian_robot_cost_clay: i32 = 0;
    let mut Tgeode_robot_cost_ore: i32 = 0;
    let mut Tgeode_robot_cost_obsidian: i32 = 0;   

    let mut blueprints: Vec<Blueprint> = Vec::new();

    let mut cnt=0;
    for line in lines {
        cnt += 1;
        let words = line.split(" ").collect::<Vec<&str>>();
        match cnt {
            2 => {
                Tore_robot_cost_ore = words[6].parse::<i32>().unwrap();    
            }              
            3 => {
                Tclay_robot_cost_ore = words[6].parse::<i32>().unwrap();    
            }
            4 => {
                Tobsidian_robot_cost_ore = words[6].parse::<i32>().unwrap();    
                Tobsidian_robot_cost_clay = words[9].parse::<i32>().unwrap();    
            }
            5 => {
                Tgeode_robot_cost_ore = words[6].parse::<i32>().unwrap();    
                Tgeode_robot_cost_obsidian = words[9].parse::<i32>().unwrap();    
            }
            6 => {
                cnt =0;
                blueprints.push(
                    Blueprint {
                        ore_robot_cost_ore: Tore_robot_cost_ore,
                        clay_robot_cost_ore: Tclay_robot_cost_ore,
                        obsidian_robot_cost_ore: Tobsidian_robot_cost_ore,
                        obsidian_robot_cost_clay: Tobsidian_robot_cost_clay,
                        geode_robot_cost_ore: Tgeode_robot_cost_ore,
                        geode_robot_cost_obsidian: Tgeode_robot_cost_obsidian                        
                    }
                );
                Tore_robot_cost_ore = 0;
                Tclay_robot_cost_ore = 0;
                Tobsidian_robot_cost_ore = 0;
                Tobsidian_robot_cost_clay = 0;
                Tgeode_robot_cost_ore = 0;
                Tgeode_robot_cost_obsidian = 0;                   
            }

            _ => {}
        }
    }
    blueprints
}

fn find_best_config(mut inventory: Inventory, bleuprint: &Blueprint, turns_left:i32 ) -> i32 {

        //collect goods
        inventory.ore_amnt += inventory.ore_robots;
        inventory.clay_amnt += inventory.clay_robots;
        inventory.obsidian_amnt += inventory.obsidian_robots;
        inventory.geode_amnt += inventory.geode_robots;

        println!("inv after {:?}",inventory);

        //prodcue robots
        match inventory.construct {
            Collector::Ore => {
                inventory.ore_robots += 1;
            }
            Collector::Clay => {
                inventory.clay_robots += 1;
            }
            Collector::Obsidian => {
                inventory.obsidian_robots += 1;
            }
            Collector::Geode => {
                inventory.geode_robots += 1;
            }
            // Collector::Skip => {}
        }
        let mut bestresult = inventory.ore_amnt;    

    if turns_left > 0 {
        //decide what robot to make
        'lupa: for robot in [Collector::Ore, Collector::Clay, Collector::Obsidian, Collector::Geode] {
            match robot {
                Collector::Ore => {
                    if inventory.ore_amnt >= bleuprint.ore_robot_cost_ore {
                        inventory.construct = robot;
                        inventory.ore_amnt -= bleuprint.ore_robot_cost_ore;
                    } else {
                        // inventory.construct = Collector::Skip;
                        continue 'lupa;
                    }
                }
                Collector::Clay => {
                    if inventory.ore_amnt >= bleuprint.clay_robot_cost_ore {
                        inventory.construct = robot;
                        inventory.ore_amnt -= bleuprint.clay_robot_cost_ore;
                    } else {
                        // inventory.construct = Collector::Skip;
                        continue 'lupa;
                    }
                }
                Collector::Obsidian => {
                    if inventory.ore_amnt >= bleuprint.obsidian_robot_cost_ore &&
                       inventory.clay_amnt >= bleuprint.obsidian_robot_cost_clay {
                        inventory.construct = robot;
                        inventory.ore_amnt -= bleuprint.obsidian_robot_cost_ore;
                        inventory.clay_amnt -= bleuprint.obsidian_robot_cost_clay;
                        println!("producing obsidian robot");
                    } else {
                        // inventory.construct = Collector::Skip;
                        continue 'lupa;
                        // println!("not producing obsidian robot {} {}",inventory.ore_amnt, inventory.clay_amnt);
                    }
                }
                Collector::Geode => {
                    if inventory.ore_amnt >= bleuprint.geode_robot_cost_ore &&
                       inventory.obsidian_amnt >= bleuprint.geode_robot_cost_obsidian {
                        inventory.construct = robot;
                        inventory.ore_amnt -= bleuprint.geode_robot_cost_ore;
                        inventory.obsidian_amnt -= bleuprint.geode_robot_cost_obsidian;
                    } else {
                        // inventory.construct = Collector::Skip;
                        continue 'lupa;
                    }
                }
                // Collector::Skip => {}
            }
            let result = find_best_config(inventory.clone(), &bleuprint, turns_left-1);
            // if result> bestresult{
                println!("new best {}", &result);
            // }
            bestresult = bestresult.max(result);

        }
    }
    bestresult
}

fn main() {

    let blueprints = read_blueprints();

    //part1 
    //---------------------------------------------------------
    //println!("{:?}",blueprints);

    let inventory = Inventory { 
        ore_amnt: 0,
        clay_amnt: 0, 
        obsidian_amnt: 0, 
        geode_amnt: 0, 
        ore_robots: 0, 
        clay_robots: 0, 
        obsidian_robots: 0, 
        geode_robots: 0, 
        construct: Collector::Ore
    };

    for blueprint in blueprints {
        let best_result = find_best_config(inventory.clone(), &blueprint, 24+1);
        println!("best result: {}",best_result);
    }
    //part2 
    //---------------------------------------------------------


    //for test
    //---------------------------------------------------------
    let max = count(1, 3);
    println!("Hello, world! {}", max);
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

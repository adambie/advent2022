use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Blueprint{
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost_ore: i32,
    obsidian_robot_cost_clay: i32,
    geode_robot_cost_ore: i32,
    geode_robot_cost_obsidian: i32,

    max_ore_use_per_turn: i32,
}

#[derive(PartialEq,Clone,Eq, Hash)]
enum Construction{
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot
}

#[derive(Clone,PartialEq, Eq,Hash)]
struct State{
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,

    ore_robots: i32,    
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,

    construction: Construction
}

fn find_most_geodes( blueprint: &Blueprint, mut remaining_turns: i32, mut state: State ) -> i32 {
    let mut robot_constructed = false;
    while robot_constructed == false && remaining_turns > 0 {
        match state.construction {
            Construction::OreRobot => {
                if state.ore >= blueprint.ore_robot_cost {
                    state.ore -= blueprint.ore_robot_cost;
                    robot_constructed = true;
                }
            },
            Construction::ClayRobot => {
                if state.ore >= blueprint.clay_robot_cost {
                    state.ore -= blueprint.clay_robot_cost;
                    robot_constructed = true;
                }
            }, 
            Construction::ObsidianRobot => {
                if state.ore >= blueprint.obsidian_robot_cost_ore && state.clay >= blueprint.obsidian_robot_cost_clay {
                    state.ore -= blueprint.obsidian_robot_cost_ore;
                    state.clay -= blueprint.obsidian_robot_cost_clay;
                    robot_constructed = true;
                }
            }, 
            Construction::GeodeRobot => {
                if state.ore >= blueprint.geode_robot_cost_ore && state.obsidian >= blueprint.geode_robot_cost_obsidian {
                    state.ore -= blueprint.geode_robot_cost_ore;
                    state.obsidian -= blueprint.geode_robot_cost_obsidian;
                    robot_constructed = true;
                }
            }
        }
        state.ore += state.ore_robots;
        state.clay += state.clay_robots;
        state.obsidian += state.obsidian_robots;
        state.geodes += state.geode_robots;
        remaining_turns -= 1;
        if robot_constructed {
            match state.construction{
                Construction::OreRobot => { state.ore_robots += 1 },
                Construction::ClayRobot => { state.clay_robots += 1 },
                Construction::ObsidianRobot => { state.obsidian_robots += 1 }, 
                Construction::GeodeRobot => { state.geode_robots += 1 }
            }
        }
    }

    // Decide what do build next. Always possible to build ore and clay robots but 
    let mut max_geodes = state.geodes;
    if remaining_turns > 0 {
        for next_robot in [ Construction::OreRobot, Construction::ClayRobot, Construction::ObsidianRobot, Construction::GeodeRobot ] {
            // Dont evaluate paths that have unbuildablle robots
            if next_robot == Construction::ObsidianRobot && state.clay_robots == 0 { 
                continue;
            }
            if next_robot == Construction::GeodeRobot && state.obsidian_robots == 0 { 
                continue;
            }
            // Dont build more robots for a resource than could be consumed
            if next_robot == Construction::OreRobot && state.ore_robots == blueprint.max_ore_use_per_turn ||
                next_robot == Construction::ClayRobot && state.clay_robots == blueprint.obsidian_robot_cost_clay ||
                next_robot == Construction::ObsidianRobot && state.obsidian_robots == blueprint.geode_robot_cost_obsidian
            {
                continue;
            }

            let mut search_state = state.clone();
            search_state.construction = next_robot;
            let num_geodes = find_most_geodes( blueprint, remaining_turns, search_state);
            max_geodes = max_geodes.max(num_geodes);
        }
    }
    return max_geodes;
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn read_blueprints() -> Vec<Blueprint> {
    let lines = lines_from_file("19.txt").expect("Could not load lines");
    

    let mut ore_robot_cost: i32 = 0;
    let mut clay_robot_cost: i32 = 0;
    let mut obsidian_robot_cost_ore: i32 = 0;
    let mut obsidian_robot_cost_clay: i32 = 0;
    let mut geode_robot_cost_ore: i32 = 0;
    let mut geode_robot_cost_obsidian: i32 = 0;   
    let mut max_ore_use_per_turn: i32 = 0;   

    let mut blueprints: Vec<Blueprint> = Vec::new();

    let mut cnt=0;
    for line in lines {
        cnt += 1;
        let words = line.split(" ").collect::<Vec<&str>>();
        match cnt {
            2 => {
                ore_robot_cost = words[6].parse::<i32>().unwrap();    
            }              
            3 => {
                clay_robot_cost = words[6].parse::<i32>().unwrap();    
            }
            4 => {
                obsidian_robot_cost_ore = words[6].parse::<i32>().unwrap();    
                obsidian_robot_cost_clay = words[9].parse::<i32>().unwrap();    
            }
            5 => {
                geode_robot_cost_ore = words[6].parse::<i32>().unwrap();    
                geode_robot_cost_obsidian = words[9].parse::<i32>().unwrap();    
            }
            6 => {
                cnt =0;
                max_ore_use_per_turn = max_ore_use_per_turn.max(obsidian_robot_cost_ore.max(geode_robot_cost_ore));
                blueprints.push(
                    Blueprint {
                        ore_robot_cost,
                        clay_robot_cost,
                        obsidian_robot_cost_ore,
                        obsidian_robot_cost_clay,
                        geode_robot_cost_ore,
                        geode_robot_cost_obsidian,
                        max_ore_use_per_turn                        
                    }
                );
                ore_robot_cost = 0;
                clay_robot_cost = 0;
                obsidian_robot_cost_ore = 0;
                obsidian_robot_cost_clay = 0;
                geode_robot_cost_ore = 0;
                geode_robot_cost_obsidian = 0;  
                max_ore_use_per_turn = 0;                 
            }

            _ => {}
        }
    }
    blueprints
}

fn main() {
    // Part1
    let blueprints = read_blueprints();


    // Part 1
    let mut quality_level = 0;
    for (idx, blueprint )in blueprints.iter().enumerate() {
        // Start with enough ore to build an ore-robot ( the problem states you already have it but this way the search starts after the 'first' one is built) 
        // (for the same reason we add an extra turn)
        let initial_state =  State{ ore: blueprint.ore_robot_cost, clay: 0, obsidian: 0, geodes: 0, ore_robots: 0, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
            construction: Construction::OreRobot };
        let most_geods = find_most_geodes( &blueprint, 24+1, initial_state );
        println!("best: {}", most_geods);
        quality_level += (idx as i32 + 1)*most_geods;
        //println!("{}, {}, {}", idx+1, quality_level, most_geods );
    } 
    //println!( "Total quality level is {}",quality_level );

    // Part 2
    // let mut result = 1;
    // for (idx, blueprint )in blueprints[0..3].iter().enumerate() {
    //     // Start with enough ore to build an ore-robot ( the problem states you already have it but this way the search starts after the 'first' one is built) 
    //     // (for the same reason we add an extra turn)
    //     let initial_state =  State{ ore: blueprint.ore_robot_cost, clay: 0, obsidian: 0, geodes: 0, ore_robots: 0, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
    //         construction: Construction::OreRobot };
    //     let most_geods = find_most_geodes( &blueprint, 32+1, initial_state );
    //     result *= most_geods;
    //     println!("{}, {}, {}", idx+1, result, most_geods );
    // } 
    // println!( "Part 2 result is {}",result );
}
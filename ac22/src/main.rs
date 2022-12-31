use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn load_data(filename: &str) -> (Vec<Vec<char>>, Vec<String>) {
    let lines = lines_from_file(filename).expect("Could not load lines");
    let width: usize = lines.iter().fold(0, |mut acc, x| {
        if x.len() > acc {
            acc = x.len();
        }
        acc
    });
    let height = lines.len() - 2;
    println!("W: {} H: {}", width,height);

    let mut map: Vec<Vec<char>> = vec![vec![' '; height]; width];

    for y in 0..lines.len()-2 {
        let line = lines[y].clone();
        for x in 0..line.len() {
            map[y][x] = line.chars().nth(x).unwrap();
        }
    }


    let work: Vec<&str> = lines[lines.len()-1].split_inclusive(['U','D','R','L']).into_iter().collect();
    let mut instruction: Vec<String> = Vec::new();
    for x in 0..work.len() {
        instruction.push(work[x].to_string());
    }

    (map, instruction)
}

#[derive(Debug,Copy,Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down
}
#[derive(Debug,Copy,Clone)]
struct Position {
    x: usize,
    y: usize,
    facing: Direction
}

impl Position {
    fn moveit(&mut self, inst: String, map: &Vec<Vec<char>>){
        
        let last = inst.chars().last().unwrap();
        let mut num = inst.clone();
        num.remove(num.len()-1);
        let mut dir: Direction;
        match last {
            'R' => {    
                match self.facing {
                    Direction::Left => {
                        dir = Direction::Up;
                    },
                    Direction::Up => {
                        dir = Direction::Right;
                    },
                    Direction::Right => {
                        dir = Direction::Down;
                    },
                    Direction::Down => {
                        dir = Direction::Left;
                    }
                }
            },
            'L' => {
                match self.facing {
                    Direction::Left => {
                        dir = Direction::Down;
                    },
                    Direction::Up => {
                        dir = Direction::Left;
                    },
                    Direction::Right => {
                        dir = Direction::Up;
                    },
                    Direction::Down => {
                        dir = Direction::Right;
                    }
                }
            },
            _ => {
                dir = Direction::Down;
                num = inst.clone();
            }
        }

        let distance;
        match num.parse::<usize>() { 
            Ok(res) => {
                distance = res;
            },
            Err(_) => { distance =0; }
        }

        self.reallymove(distance, &map);
        self.facing = dir;

    }
    fn reallymove(&mut self, distance: usize, map: &Vec<Vec<char>>) {
        for x in 0..distance {
            match self.facing {
                Direction::Up => {
                    if self.y == 0 {
                        let mut yb = map.len()-1;
                        if map[yb][self.x] == '.' {
                            self.y = yb;
                        } else {
                            loop {
                                yb -= 1;
                                if map[yb][self.x] == '.' {
                                    self.y = yb;
                                    break;
                                } else if map[yb][self.x] == '#' {
                                    break;
                                }
                                if yb == 0 {
                                    break;
                                }
                            }
                        }
                    } else if self.y>=1 && map[(self.y - 1) as usize][self.x] == '.' {
                        self.y -= 1;
                    }
                },
                Direction::Down => {
                    if map[(self.y + 1) as usize][self.x] == '.' && (self.y+1)<map.len() {
                        self.y += 1;
                    } else if (self.y+1) == map.len() {
                        let mut yb = 0;
                        if map[yb][self.x] == '.' {
                            self.y = yb;
                        } else {
                            loop {
                                yb += 1;
                                if map[yb][self.x] == '.' {
                                    self.y = yb;
                                    break;
                                } else if map[yb][self.x] == '#' {
                                    break;
                                }
                                if yb == map.len() {
                                    break;
                                }
                            }
                        }
                    }                    
                },
                Direction::Left => {
                    if map[self.y][(self.x-1) as usize] == '.' && (self.x-1)>0 {
                        self.x -= 1;
                    } else if (self.x-1) == 0 {
                        let mut xb = map[self.y].len()-1;
                        if map[self.y][xb] == '.' {
                            self.x = xb;
                        } else {
                            loop {
                                xb -= 1;
                                if map[self.y][xb] == '.' {
                                    self.x = xb;
                                    break;
                                } else if map[self.y][xb] == '#' {
                                    break;
                                }
                                if xb == 0 {
                                    break;
                                }
                            }
                        }
                    }    
                },
                Direction::Right => {
                    if map[self.y][(self.x+1) as usize] == '.' && (self.x+1)<map[self.y].len() {
                        self.x += 1;
                    } else if (self.x+1) == map[self.y].len() {
                        let mut xb = 0;
                        if map[self.y][xb] == '.' {
                            self.x = xb;
                        } else {
                            loop {
                                xb += 1;
                                if map[self.y][xb] == '.' {
                                    self.x = xb;
                                    break;
                                } else if map[self.y][xb] == '#' {
                                    break;
                                }
                                if xb == map[self.y].len() {
                                    break;
                                }
                            }
                        }
                    }    
                }
            }
        }
    }
}

fn findstart(map: &Vec<Vec<char>>) -> Position  {
    let mut xpos: usize=0;
    let mut ypos: usize=0;

    'mainloop: for y in 0..map.len(){
        for x in 0..map[y].len() {
            if map[y][x]=='.' {
            xpos=x;
            ypos=y;
            break 'mainloop;
            }
        }
    }
    Position {x:xpos, y: ypos, facing: Direction::Right}
}

fn main() {
    let (map, instruction) = load_data("22.txt");
    let mut pos: Position = findstart(&map);


    // for y in 0..map.len() {
    //     for x in 0..map[y].len() {
    //         print!("{}",map[y][x]);

    //     }
    //     println!("");
    // }

    println!("startpos: {:?}",&pos);

    for (idx, inst) in instruction.iter().enumerate() {
        pos.moveit(inst.to_string(),&map);
        // println!("moved to: {:?}",pos);
    }
    println!("endpos: {:?}",pos);
    println!("result p1 is: {}", 1000*pos.y + 4*pos.x);

}

use std::fs;
use std::time::Instant;
use std::collections::HashMap;


fn part_1(data: &str) -> usize {

    let (map_data, moves) = data.split_once("\r\n\r\n").unwrap();

    let mut map = map_data.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let x_max = map[0].len();
    let y_max = map.len();

    let (mut r_x, mut r_y) = (0,0);
    for y in 0..y_max{
        for x in 0..x_max{

            if map[y][x] == '@'{
                r_x = x as i32;
                r_y = y as i32;
            }
        }
    }

    let mv_map = HashMap::from([('^', (0,-1)), ('v', (0,1)),('>', (1,0)),('<', (-1,0))]);

    for mv in moves.chars(){

        if !mv_map.contains_key(&mv){
            continue;
        }

        let dir = mv_map[&mv];
        
        //Move to empty location
        if map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] == '.'{
            //Update map values
            map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] = '@';
            map[r_y as usize][r_x as usize] = '.';
            
            //Move robot
            (r_x, r_y) = (r_x + dir.0, r_y+dir.1);
        } else if map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] == 'O'{

            let mut n_rocks = 1;
            let can_move;

            loop{
                let spot = map[(r_y+dir.1*(n_rocks+1)) as usize][(r_x+dir.0*(n_rocks+1)) as usize];

                if  spot == '.'{
                    can_move = true;
                    break;
                } else if spot == '#'{
                    can_move = false;
                    break;
                } else{
                    n_rocks += 1;
                }
            }

            if can_move {
                
                //Move rocks
                for n in 0..n_rocks{
                    //Update map values
                    map[(r_y+dir.1*(n+2)) as usize][(r_x+dir.0*(n+2)) as usize] = 'O';
                }
                //Move robot
                map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] = '@';
                map[r_y as usize][r_x as usize] = '.';   
                (r_x, r_y) = (r_x + dir.0, r_y+dir.1);
            }

        }

        //Print the current map state
        // for y in 0..y_max{
        //     for x in 0..x_max{
        //         print!("{}", map[y][x]);
        //     }
        //     println!();
        // }

    }

    //Sum GPS coordinates
    let mut sum = 0;
    for y in 0..y_max{
        for x in 0..x_max{
            if map[y][x] == 'O'{
                sum += x + 100*y;
            }
        }
    }


    sum
}

fn part_2(data: &str) -> usize {

    let (map_data, moves) = data.split_once("\r\n\r\n").unwrap();

    let new_map = HashMap::from([('#', "##"), ('O', "[]"), ('.', ".."), ('@', "@.") ]);
    let mut map = map_data.lines().map(|line| line.chars().flat_map(|c| new_map[&c].chars()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let x_max = map[0].len();
    let y_max = map.len();

    let (mut r_x, mut r_y) = (0,0);
    for y in 0..y_max{
        for x in 0..x_max{
            if map[y][x] == '@'{
                r_x = x as i32;
                r_y = y as i32;
            }
        }
    }

    let mv_map = HashMap::from([('^', (0,-1)), ('v', (0,1)),('>', (1,0)),('<', (-1,0))]);

    for mv in moves.chars(){

        if !mv_map.contains_key(&mv){
            continue;
        }

        //Directions
        let dir = mv_map[&mv];
        
        //Move to empty location
        if map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] == '.'{
            
            //Move robot
            map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] = '@';
            map[r_y as usize][r_x as usize] = '.';            
            (r_x, r_y) = (r_x + dir.0, r_y+dir.1);
        }
        //Move left and right 
        else if ['[', ']'].contains(&map[(r_y+dir.1) as usize][(r_x+dir.0) as usize]){

            if ['<','>'].contains(&mv){

                let mut n_rocks = 1;
                let can_move;

                loop{
                    let spot = map[(r_y+dir.1*(n_rocks+1)) as usize][(r_x+dir.0*(n_rocks+1)) as usize];

                    if  spot == '.'{
                        can_move = true;
                        break;
                    } else if spot == '#'{
                        can_move = false;
                        break;
                    } else{
                        n_rocks += 1;
                    }
                }

                if can_move {                    
                    //Move rocks
                    for n in 0..n_rocks{
                        //Update map values
                        map[(r_y+dir.1*((n_rocks-n)+1)) as usize][(r_x+dir.0*(n_rocks-n+1)) as usize] = map[(r_y+dir.1*(n_rocks-n)) as usize][(r_x+dir.0*(n_rocks-n)) as usize];
                    }
                    //Move robot
                    map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] = '@';
                    map[r_y as usize][r_x as usize] = '.';   
                    (r_x, r_y) = (r_x + dir.0, r_y+dir.1);
                }
            } 
            //Move up or down
            else {

                let mut crates: Vec<(i32, i32, i32)> = Vec::new(); //(x1, x2, y)

                fn add_crate(x: i32, y: i32, crates: &mut Vec<(i32, i32, i32)>, map: &[Vec<char>]){
                    let spot = map[y as usize][x as usize];
                    let crate_pos;

                    if spot == ']'{
                        crate_pos = (x-1, x, y);
                        
                    } else {
                        crate_pos = (x, x+1, y);
                    }

                    if !crates.contains(&crate_pos){
                            crates.push(crate_pos);
                        }                    
                }

                add_crate(r_x+dir.0, r_y+dir.1, &mut crates, &map);
                
                let mut can_move = true;
                loop {
                    
                    let mut new_crates = crates.clone(); //This is propably inefficient
                    for block in &crates {
    
                        let spot_1 = map[(block.2+dir.1) as usize][(block.0+dir.0) as usize];
                        let spot_2 = map[(block.2+dir.1) as usize][(block.1+dir.0) as usize];
    
                        if ['[',']'].contains(&spot_1){
                            add_crate(block.0+dir.0, block.2+dir.1, &mut new_crates, &map);
                        } 
    
                        if ['[',']'].contains(&spot_2){
                            add_crate(block.1+dir.0, block.2+dir.1, &mut new_crates, &map);
                        }
    
                        if spot_1 == '#' || spot_2 == '#' {
                            can_move = false;
                            break;
                        }
                    }

                    if !can_move || crates.len() == new_crates.len(){
                        break;
                    }

                    crates = new_crates;
                }

                if can_move{
                    crates.reverse();

                    for block in crates{
    
                        map[(block.2+dir.1) as usize][(block.0+dir.0) as usize] = map[(block.2) as usize][(block.0) as usize];
                        map[(block.2+dir.1) as usize][(block.1+dir.0) as usize] = map[(block.2) as usize][(block.1) as usize];
    
                        map[(block.2) as usize][(block.0) as usize] = '.';
                        map[(block.2) as usize][(block.1) as usize] = '.';
                    }
    
                    //Move robot
                    map[(r_y+dir.1) as usize][(r_x+dir.0) as usize] = '@';
                    map[r_y as usize][r_x as usize] = '.';   
                    (r_x, r_y) = (r_x + dir.0, r_y+dir.1);

                }
            }
        }
    }

    let mut sum = 0;
    for (y, line) in map.iter().enumerate(){
        for (x, c) in line.iter().enumerate(){
            if c == &'['{
                sum += x + 100*y;
            }
        }
    }


    sum
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 15***");

    //Part 1
    let timer = Instant::now();
    let result_1 = part_1(&data);
    let time_1 = timer.elapsed();

    println!("Part 1: {:?} - Duration: {:?}", result_1, time_1);

    //Part 2
    let timer = Instant::now();

    let result_2 = part_2(&data);
    let time_2 = timer.elapsed();
    println!("Part 2: {:?} - Duration: {:?}", result_2, time_2);
}
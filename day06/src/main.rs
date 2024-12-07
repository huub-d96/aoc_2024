use std::fs;
use std::collections::HashMap;
use ndarray::{arr1, arr2};

fn main() {
    let raw_data= fs::read_to_string("src/data.txt").expect("Failed to read");
    let data:  Vec<Vec<char>> = raw_data.lines().map(|line| line.trim().chars().collect()).collect();

    let directions = HashMap::from([('^', arr1(&[0,-1])), ('>', arr1(&[1,0])), ('v', arr1(&[0,1])), ('<', arr1(&[-1,0]))]);
    let right = arr2(&[[0,-1],
                                                        [1,0]]);

    let x_max = data[0].len();
    let y_max = data.len();

    let mut start = (0,0);
    let mut dir = 'v';
    
    //Find start point and direction
    for y in 0..y_max{
        for x in 0..x_max{
            
            if ['^', 'v', '>', '<'].contains(&data[y][x]){
                start = (x as i32, y as i32);
                dir = data[y][x];
            }
        }
    }

                    
    let mut count_1 = 0;
    let mut loop_count = 0;
    for y in 0..y_max{
        for x in 0..x_max{

            let mut obst_data = data.clone();
            if ['^', 'v', '>', '<'].contains(&obst_data[y][x]){
                continue;
            }

            obst_data[y][x] = '#';

            let mut pos = start;
            let mut direction = directions[&dir].clone(); 
            loop{

                let x_next = pos.0+direction[0];
                let y_next = pos.1+direction[1];

                let dir_char = directions.iter().find_map(|(k,v)| if v == direction { Some(k)} else { None });

                if x_next < 0 || y_next < 0 || x_next >= x_max as i32 || y_next >= y_max as i32{
                    
                    if count_1 == 0 {
                        for row in obst_data{
                            for c in row{
                                if ['^', 'v', '>', '<'].contains(&c){
                                    count_1 += 1;
                                }
                            }
                        }
                    }
                    break;
                }        

                if obst_data[y_next as usize][x_next as usize] == '#'{
                    direction = right.dot(&direction);
                } else{

                    if obst_data[y_next as usize][x_next as usize] == *dir_char.unwrap(){
                        loop_count +=1;
                        break;
                    }
                    
                    obst_data[y_next as usize][x_next as usize] = *dir_char.unwrap();            
                    pos = (x_next, y_next);
                }
            }
        }
    }

    //Print results
    println!("Part 1: {}", count_1);
    println!("Part 2: {}", loop_count)



}
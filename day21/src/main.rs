use std::collections::HashMap;
use std::{char, fs};
use std::time::Instant;
use std::cmp::{min, max};

const KEYPAD: [[char;3];4] = [['7','8','9'], ['4', '5', '6'], ['1', '2', '3'], ['#', '0', 'A']];
const DIRPAD: [[char;3];2] = [['#','^','A'], ['<', 'v', '>']];

fn manhattan(x1:usize, y1:usize, x2:usize, y2:usize) -> usize{

    max(x1, x2) - min(x1, x2) + max(y1,y2) - min(y1, y2)
}

fn backtrack_keypad(p: (usize, usize), e: (usize, usize), map: &[[char;3];4], path: String) -> Vec<String>{

    if p == e{
        let out = Vec::from([path+"A"]);
        return out
    }

    let mut final_path = Vec::new();
    for (dx, dy, dir) in [(0,1, "v"), (0,-1, "^"), (1,0, ">"), (-1,0, "<")]{

        let x_next = p.0 as i32 + dx;
        let y_next = p.1 as i32 + dy;

        if x_next < 0 || y_next < 0 || x_next >= KEYPAD[0].len() as i32 || y_next >= KEYPAD.len() as i32{
            continue;
        }

        if map[y_next as usize][x_next as usize] == '#'{
            continue;
        }

        if !path.ends_with(dir) && path.contains(dir){
            continue;
        }

        if manhattan(p.0, p.1, e.0, e.1) - 1  == manhattan(x_next as usize, y_next as usize, e.0, e.1){
            final_path.append(&mut backtrack_keypad((x_next as usize, y_next as usize), e, map, path.clone() + dir));
        }   
    } 
    final_path
} 

fn backtrack_dirpad(p: (usize, usize), e: (usize, usize), map: &[[char;3];2], path: String) -> Vec<String>{

    if p == e{
        let out = Vec::from([path+"A"]);
        return out
    }

    let mut final_path = Vec::new();
    for (dx, dy, dir) in [(0,1, "v"), (0,-1, "^"), (1,0, ">"), (-1,0, "<")]{

        let x_next = p.0 as i32 + dx;
        let y_next = p.1 as i32 + dy;

        if x_next < 0 || y_next < 0 || x_next >= DIRPAD[0].len() as i32 || y_next >= DIRPAD.len() as i32{
            continue;
        }

        if map[y_next as usize][x_next as usize] == '#'{
            continue;
        }

        if !path.ends_with(dir) && path.contains(dir){
            continue;
        }

        if manhattan(p.0, p.1, e.0, e.1) - 1 == manhattan(x_next as usize, y_next as usize, e.0, e.1){
             
            final_path.append(&mut backtrack_dirpad((x_next as usize, y_next as usize), e, map, path.clone() + dir));
        }   
    } 
    final_path
}

fn find_keypad_loc(button: &char) -> (usize, usize){
    for (y, row) in KEYPAD.iter().enumerate(){
        for (x, num) in row.iter().enumerate(){
            if num == button{
                return (x,y)
            }
        }
    }

    (0,0)
}

fn find_dirpad_loc(button: &char) -> (usize, usize){
    for (y, row) in DIRPAD.iter().enumerate(){
        for (x, num) in row.iter().enumerate(){
            if num == button{
                return (x,y)
            }
        }
    }

    (0,0)
}


fn find_sequence(input: String, depth: u32, max_depth: u32, cache: &mut HashMap<(String, u32), usize> ) -> usize{

    if cache.contains_key(&(input.clone(), depth)){
        return cache[&(input, depth)]
    }

    if depth == max_depth{
        return input.len();
    }

    let mut r_pos = find_dirpad_loc(&'A'); 
    let mut sequence = 0;

    for c in input.chars(){
        let target = find_dirpad_loc(&c);    

        let outputs = backtrack_dirpad(r_pos, target, &DIRPAD, "".to_string());

        //Always hit the left most key first
        let out = {
            if outputs.len()==2 {
                if outputs[0].starts_with("<") {
                    &outputs[0]
                } else if outputs[1].starts_with("<") {
                    &outputs[1]
                } else if outputs[0].starts_with("^") || outputs[0].starts_with("v") {
                    &outputs[0]
                } else if outputs[1].starts_with("^") || outputs[1].starts_with("v") {
                    &outputs[1]
                } else{
                    &outputs[0]
                }
            } else {
                &outputs[0]
            }
        };
        
        r_pos = target;
        sequence += find_sequence(out.to_string(), depth+1, max_depth, cache);
    }

    cache.insert((input, depth), sequence);

    sequence
}


fn solve_sequence(data: &str, n_dirpads: u32) -> usize {

    let mut sum = 0;
    let mut cache = HashMap::new();

    for code in data.lines(){

    
    
    let mut sequence = 0;

    let mut r1_pos = find_keypad_loc(&'A'); //A button
    
    //Do robot 1 manual, since it uses a different keypad
    for c in code.chars(){

        let target = find_keypad_loc(&c);

        let outputs = backtrack_keypad(r1_pos, target, &KEYPAD, "".to_string());

        let out = {
            if outputs.len()==2 {
                if outputs[0].starts_with("<") {
                    &outputs[0]
                } else if outputs[1].starts_with("<") {
                    &outputs[1]
                } else if outputs[0].starts_with("^") || outputs[0].starts_with("v") {
                    &outputs[0]
                } else if outputs[1].starts_with("^") || outputs[1].starts_with("v") {
                    &outputs[1]
                } else{
                    &outputs[0]
                }
            } else {
                &outputs[0]
            }
        };

        r1_pos = target;  

        //Recursively find sequence lengths for directional pad robots
        sequence += &find_sequence(out.to_string(), 1, 1+n_dirpads, &mut cache);
    }

    sum  += sequence*code.replace("A", "").parse::<usize>().unwrap();

}

sum
}

fn part_1(data: &str) -> usize {

    solve_sequence(data, 2)
}
    

fn part_2(data: &str) -> usize {

    solve_sequence(data, 25)
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 21***");

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
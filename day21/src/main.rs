use std::fs;
use std::time::Instant;
use std::cmp::{min, max};

const KEYPAD: [[char;3];4] = [['7','8','9'], ['4', '5', '6'], ['1', '2', '4'], ['#', '0', 'A']];
const DIRPAD: [[char;3];2] = [['#','^','A'], ['<', 'v', '>']];

pub fn manhattan(x1:usize, y1:usize, x2:usize, y2:usize) -> usize{

    max(x1, x2) - min(x1, x2) + max(y1,y2) - min(y1, y2)
}

fn backtrack_keypad(p: (usize, usize), e: (usize, usize), map: &[[char;3];4], path: String) -> Vec<String>{

    if p == e{
        let out = Vec::from([path]);
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

        if manhattan(p.0, p.1, e.0, e.1) - 1  == manhattan(x_next as usize, y_next as usize, e.0, e.1){
            final_path.append(&mut backtrack_keypad((x_next as usize, y_next as usize), e, map, path.clone() + dir));
        }   
    } 
    final_path
} 

fn backtrack_dirpad(p: (usize, usize), e: (usize, usize), map: &[[char;3];2], path: String) -> Vec<String>{

    if p == e{
        let out = Vec::from([path]);
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



fn part_1(_data: &str) -> u32 {
    
    let code = "029A";

    let mut r1_pos = find_keypad_loc(&'A'); //A button
    let mut r2_pos = find_dirpad_loc(&'A'); //A button
    let mut r3_pos = find_dirpad_loc(&'A'); //A button

    let mut seqs: Vec<String> = Vec::from(["".to_string()]);

    //Robot 1 possible routes
    for c in code.chars(){

        let target = find_keypad_loc(&c);

        let output = backtrack_keypad(r1_pos, target, &KEYPAD, "".to_string());

        println!("{:?}", output);

        let mut new_seqs = Vec::new();
        for s in &seqs{
            for o in &output{

                new_seqs.push([s, o, "A"].join(""))
            }
        }
        seqs = new_seqs;

        r1_pos = target;
    }

    println!("{:?}", seqs);

    //Robot 2 possible routes
    for seq in seqs.clone(){
        
        for c in seq.chars(){

            let target = find_dirpad_loc(&c);
            
    
            let output = backtrack_dirpad(r2_pos, target, &DIRPAD, "".to_string());
            println!("{:?}", output);
        
            // let mut new_seqs = Vec::new();
            // for s in &seqs{
            //     for o in &output{
    
            //         new_seqs.push([s, o, "A"].join(""))
            //     }
            // }
            // println!("{:?}",new_seqs);
            // seqs = new_seqs;
    
            r2_pos = target;
        }

    }



0
}

fn part_2(_data: &str) -> u32 {

0    
}


fn main() {
    let data= fs::read_to_string("src/test.txt").expect("Failed to read");

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
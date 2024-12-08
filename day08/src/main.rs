use std::fs;
use std::collections::{HashMap, HashSet};


fn find_antinodes(harmonics: bool) -> usize {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    let mut nodes= HashMap::<char, Vec<(_,_)>>::new();
    let mut antinodes = HashSet::<(_,_)>::new();
    
    let x_max = data.lines().count();
    let y_max = data.lines().next().unwrap().len();

    //Find nodes
    for (y, line) in data.lines().enumerate(){
        for (x, c) in line.chars().enumerate(){

            if c != '.'{
                if let std::collections::hash_map::Entry::Vacant(e) = nodes.entry(c) {
                    e.insert(vec!((x,y)));
                } else {
                    nodes.get_mut(&c).unwrap().push((x,y))
                }               
            }
        }
    }

    //Loop over each node frequency
    for v in nodes.values(){

        for n1 in 0..v.len() {
            for n2 in n1+1..v.len(){
               
                let dx = v[n2].0 as i8 - v[n1].0 as i8;
                let dy = v[n2].1 as i8 - v[n1].1 as i8;

                let mut i = 1;
                let mut n1_done = false;
                let mut n2_done = false;
                loop {

                    //Include nodes as antinodes and decrease every step
                    if harmonics {
                        i -= 1;
                    }

                    let a1_x = v[n1].0 as i8 - dx*i;
                    let a1_y = v[n1].1 as i8 - dy*i;
                    let a2_x = v[n2].0 as i8 + dx*i;
                    let a2_y = v[n2].1 as i8 + dy*i;

                    if !(a1_x < 0 || a1_y < 0 || a1_x >= x_max as i8 || a1_y >= y_max as i8 || n1_done) {
                        antinodes.insert((a1_x as usize, a1_y as usize));
                    } else {
                        n1_done = true;
                    }

                    if !(a2_x < 0 || a2_y < 0 || a2_x >= x_max as i8 || a2_y >= y_max as i8 || n2_done) {
                        antinodes.insert((a2_x as usize, a2_y as usize));
                    } else {
                        n2_done = true;
                    }

                    if !harmonics || (n1_done && n2_done){
                        break;
                    }
                }
            }

        }
    }

    //Print map for some nice visuals :D
    // for (y, line) in data.lines().enumerate(){
    //     for (x, c) in line.chars().enumerate(){
    //         if antinodes.contains(&(x,y)) && c=='.'{
    //             print!("#");
    //         } else{
    //             print!("{}", c);
    //         }
    //     }
    //     println!();
    // }

    antinodes.len()

    
}

fn main(){

    println!("Part 1: {}, Part 2: {}", find_antinodes(false),  find_antinodes(true));

}
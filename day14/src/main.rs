use std::fs;
use std::time::Instant;

struct Bathroom {
    width: i32,
    length: i32,
}

fn part_1(data: &str) -> u32 {

    let bathroom = Bathroom{width: 101, length: 103};
    let mut final_pos = Vec::new();
    let seconds = 100;

    for line in data.lines(){

        let splits = line.split_once(" ").unwrap();

        let mut p = splits.0.split_once('=').unwrap().1.split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut v = splits.1.split_once('=').unwrap().1.split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>();

        if p[0] < 0{
            p[0] += bathroom.width
        }

        if p[1] < 0{
            p[1] += bathroom.length
        }
        if v[0] < 0{
            v[0] += bathroom.width
        }
        if v[1] < 0{
            v[1] += bathroom.length
        }

        final_pos.push(((p[0]+v[0]*seconds)%bathroom.width, (p[1]+v[1]*seconds)%bathroom.length))

    }

    let mut qs = [0,0,0,0];
    for pos in &final_pos {

        if pos.0 < bathroom.width/2 && pos.1 < bathroom.length/2{
            qs[0] += 1;
        } else if pos.0 > bathroom.width/2 && pos.1 < bathroom.length/2{
            qs[1] += 1;
        } else if pos.0 < bathroom.width/2 && pos.1 > bathroom.length/2{
            qs[2] += 1;
        } else if pos.0 > bathroom.width/2 && pos.1 > bathroom.length/2{
            qs[3] += 1;
        }
    }  

   
    let mut safety_factor = 1;

    for q in qs{
        safety_factor *= q;
    }

    // Print map :D
    //     print!("Seconds: {} {}\n", seconds);
    //     for y in 0..bathroom.length{
    //         for x in 0..bathroom.width{
                
    //             if final_pos.contains(&(x,y)){
    //                 print!("#")
    //             } else {
    //                 print!(".")
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    
    safety_factor
}

fn part_2(data: &str) -> i32 {

    let bathroom = Bathroom{width: 101, length: 103};
    let mut ps = Vec::new();
    let mut vs = Vec::new();

    for line in data.lines(){

        let splits = line.split_once(" ").unwrap();

        let mut p = splits.0.split_once('=').unwrap().1.split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut v = splits.1.split_once('=').unwrap().1.split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>();

        if p[0] < 0{
            p[0] += bathroom.width
        }
        if p[1] < 0{
            p[1] += bathroom.length
        }
        if v[0] < 0{
            v[0] += bathroom.width
        }
        if v[1] < 0{
            v[1] += bathroom.length
        }

        ps.push((p[0], p[1]));
        vs.push((v[0], v[1]));
    }

    let (mut sec_x, mut sec_y) = (0,0);
    let (mut sec_x_max, mut sec_y_max) = (i32::MAX, i32::MAX);

    for seconds in 0..bathroom.width{
        
        let mut mse = 0;
        for i in 0..ps.len(){
            mse += ((ps[i].0+vs[i].0*seconds)%bathroom.width).pow(2);
        }

        if mse < sec_x_max{
            sec_x = seconds;
            sec_x_max = mse;
        }
    }

    for seconds in 0..bathroom.length{
        
        let mut mse = 0;
        for i in 0..ps.len(){
            mse += ((ps[i].1+vs[i].1*seconds)%bathroom.length).pow(2);
        }

        if mse < sec_y_max{
            sec_y = seconds;
            sec_y_max = mse;
        }
    }

    let mut i = 0;
    let tree_seconds;
    loop{        
        if (i*bathroom.length+(sec_y-sec_x)) % bathroom.width == 0{
            tree_seconds = sec_y + bathroom.length*i;
            break;
        }
        i += 1;
    }

    
    tree_seconds
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 14***");

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
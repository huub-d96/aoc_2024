use std::fs;
use std::time::Instant;

fn min_tokens(x_a: i32, x_b: i32, y_a: i32, y_b: i32, p_x: i32, p_y: i32, big: bool) -> i64 {
    
    let mut tokens = 0;
    if big {
        let big_num: i64 = 10000000000000;
        let b = ((p_x as i64 +big_num)*(y_a as i64) - (x_a as i64)*(p_y as i64 +big_num)) / ((x_b*y_a - x_a*y_b) as i64);
        let a = ((p_y as i64 +big_num) - b*y_b as i64)/(y_a as i64);
        

        if a*x_a as i64 + b*x_b as i64 == (p_x as i64 + big_num) && a*y_a as i64 + b*y_b as i64 == (p_y as i64 +big_num) && a > 0 && b > 0 {
            tokens += a*3 + b;
        }

    } else {    

        let b = (p_x*y_a - x_a*p_y) / (x_b*y_a - x_a*y_b);
        let a = (p_y - b*y_b)/y_a;
        

        if a*x_a + b*x_b == p_x && a*y_a + b*y_b == p_y && a > 0 && b > 0 {
            tokens += (a*3 + b) as i64;
        }
    }
   

    tokens
}

fn part_1(data: &str) -> i64 {

    let (mut x_a, mut x_b, mut y_a, mut y_b, mut p_x, mut p_y) = (0,0,0,0,0,0);

    let mut tokens = 0;
    for (i, line) in data.lines().enumerate(){

        match i % 4 {        
            0 => {
                x_a = ((line.as_bytes()[12]-b'0')*10 + line.as_bytes()[13]-b'0') as i32;
                y_a = ((line.as_bytes()[18]-b'0')*10 + line.as_bytes()[19]-b'0') as i32;
            }, 
            1 => {
                x_b = ((line.as_bytes()[12]-b'0')*10 + line.as_bytes()[13]-b'0') as i32;
                y_b = ((line.as_bytes()[18]-b'0')*10 + line.as_bytes()[19]-b'0') as i32;
            },
            2 => {
                let splits: Vec<_> = line.split(", ").collect();

                p_x = splits[0].split("=").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
                p_y = splits[1].split("=").collect::<Vec<_>>()[1].parse::<i32>().unwrap();

            },
            3 => {

                tokens += min_tokens(x_a, x_b, y_a, y_b, p_x, p_y, false);
            },
            _ => {}
        }  
       
    }
    tokens += min_tokens(x_a, x_b, y_a, y_b, p_x, p_y, false);    

    tokens
}

fn part_2(data: &str) -> i64 {

    let (mut x_a, mut x_b, mut y_a, mut y_b, mut p_x, mut p_y) = (0,0,0,0,0,0);

    let mut tokens = 0;
    for (i, line) in data.lines().enumerate(){

        match i % 4 {        
            0 => {
                x_a = ((line.as_bytes()[12]-b'0')*10 + line.as_bytes()[13]-b'0') as i32;
                y_a = ((line.as_bytes()[18]-b'0')*10 + line.as_bytes()[19]-b'0') as i32;
            }, 
            1 => {
                x_b = ((line.as_bytes()[12]-b'0')*10 + line.as_bytes()[13]-b'0') as i32;
                y_b = ((line.as_bytes()[18]-b'0')*10 + line.as_bytes()[19]-b'0') as i32;
            },
            2 => {
                let splits: Vec<_> = line.split(", ").collect();

                p_x = splits[0].split("=").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
                p_y = splits[1].split("=").collect::<Vec<_>>()[1].parse::<i32>().unwrap();

            },
            3 => {

                tokens += min_tokens(x_a, x_b, y_a, y_b, p_x, p_y, true);
            },
            _ => {}
        }  
       
    }
    tokens += min_tokens(x_a, x_b, y_a, y_b, p_x, p_y, true);    

    tokens
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 13***");

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
use regex::Regex;
use std::fs;

fn main() {
    let data = fs::read_to_string("src/data.txt").expect("Failed to read");

    let pattern = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))").unwrap();

    let matches = pattern.find_iter(&data);

    let mut sum_p1 = 0;
    let mut sum_p2 = 0; 
    let mut do_mul = true;
    for m in matches {

        if let Some((code, value)) = m.as_str().split_once('('){
            
            if code == "mul" {
                if let Some((x,y)) = value[0..value.len()-1].split_once(','){
                   
                    sum_p1 += x.parse::<i32>().unwrap()*y.parse::<i32>().unwrap();

                    if do_mul {
                        sum_p2 += x.parse::<i32>().unwrap()*y.parse::<i32>().unwrap();
                    }
                }
            }

            if code == "do"{
                do_mul = true;
            } 

            if code == "don't" {
                do_mul = false;
            }
        }
    };

    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2);
    
}

use std::fs;
use std::time::Instant;

fn part_1(data: &str) -> u32{

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for item in data.split("\r\n\r\n"){

        let mut reg = [0;5];

        for line in item.lines(){
            for (x, c) in line.bytes().enumerate(){                
                if c == b'#'{
                    reg[x] += 1;
                }
            }
        }

        if item.starts_with("#"){
            locks.push(reg);
        } else{
            keys.push(reg);
        }
    }

    let mut sum = 0;
    for lock in &locks{
        for key in &keys{
            let mut overlap = false;
            for i in 0..lock.len(){
                if lock[i] + key[i] > 7{
                    overlap = true;
                    break;
                } 
            }

            if !overlap{
                sum += 1;
            }

        }
    }

    sum
}

fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 25***");

    //Part 1
    let timer = Instant::now();
    let result_1 = part_1(&data);
    let time_1 = timer.elapsed();

    println!("Part 1: {:?} - Duration: {:?}", result_1, time_1);
}
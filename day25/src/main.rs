use std::fs;
use std::time::Instant;

fn part_1(data: &str) {

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for item in data.split("\r\n\r\n"){

        let mut reg = [0;5];

        for (y, line) in item.lines().enumerate(){
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

    println!("Locks: {:?}", locks);
    println!("Keys: {:?}", keys);


}

fn part_2(_data: &str) {

    
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 25***");

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
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn mix(mut num: u64) -> u64 {

    num ^= num << 6;
    num %= 16777216;

    num ^= num >> 5;
    num %= 16777216;

    num ^= num << 11;
    num %= 16777216;

    num
}

fn part_1(data: &str) -> usize {

    let mut sum = 0;

    for n in data.lines(){

        let mut num = n.trim().parse::<u64>().unwrap();

        for _ in 0..2000{        
            
            num = mix(num);
        }
    
        sum += num;  
    }
sum as usize
}

fn shift_register(mut register: u32, num: i64) -> u32{

    register <<= 5;
    register %= 2_u32.pow(20);
    register += ((num as u64) % 32) as u32;

    register
}

fn part_2(data: &str) -> usize {

    let mut all_vendor_seqs = HashMap::new();

    for n in data.lines(){

        let mut num = n.trim().parse::<u64>().unwrap();
        let mut price = num % 10;
        let mut sequence: u32 = 0;
        let mut vendor_seqs = HashSet::new();

        for i in 0..2000{        
            
            num = mix(num);

            let new_price = num % 10;
            let diff = new_price as i64 - price as i64;

            sequence = shift_register(sequence, diff);  

            if i > 2 && !vendor_seqs.contains(&sequence){
                if let Entry::Vacant(e) = all_vendor_seqs.entry(sequence) {
                    e.insert(new_price);
                } else {
                    *all_vendor_seqs.get_mut(&sequence).unwrap() += new_price
                }

                vendor_seqs.insert(sequence);
            }
            
            price = new_price;     
        } 
    }

    let max_sum = *all_vendor_seqs.values().max().unwrap();
    
max_sum as usize
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 22***");

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
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn build_output(k: &str, v: u8, output: &mut u64) {

    *output += (v as u64) << k[1..].parse::<u32>().unwrap();  
}

fn find_output<'a>(signals: &mut HashMap<&'a str, u8>, gates: &mut Vec<Vec<&'a str>>) -> u64{

    let mut output = 0;

    while let Some(gate) = gates.pop(){

        if !signals.contains_key(&gate[0]) || !signals.contains_key(&gate[2]){
            gates.insert(0, gate);
            continue;
        }
        
        match gate[1]{
            "AND" => {signals.insert(gate[4], signals[gate[0]] & signals[gate[2]]); if gate[4].starts_with("z") {build_output(gate[4], signals[gate[0]] & signals[gate[2]], &mut output);};},
            "OR"  => {signals.insert(gate[4], signals[gate[0]] | signals[gate[2]]); if gate[4].starts_with("z") {build_output(gate[4], signals[gate[0]] | signals[gate[2]], &mut output);};},
            "XOR" => {signals.insert(gate[4], signals[gate[0]] ^ signals[gate[2]]); if gate[4].starts_with("z") {build_output(gate[4], signals[gate[0]] ^ signals[gate[2]], &mut output);};},
            _ => {}
        };
    }

    output
}

fn part_1(data: &str) -> u64 {

    let (signal_data, mut gates) = data.split_once("\r\n\r\n").map(|(s, g)| (s, g.split("\r\n").map(|item| item.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>())).unwrap();

    let mut signals: HashMap<&str, u8> = signal_data.split("\n").map(|line| line.split_once(": ").map(|(k,v)| (k, v.as_bytes()[0] - b'0')).unwrap()).collect();


    find_output(&mut signals, &mut gates)
}

fn part_2(data: &str) -> u32{

    let (signal_data, mut gates) = data.split_once("\r\n\r\n").map(|(s, g)| (s, g.split("\r\n").map(|item| item.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>())).unwrap();

    let mut signals: HashMap<&str, u8> = signal_data.split("\n").map(|line| line.split_once(": ").map(|(k,v)| (k, v.as_bytes()[0] - b'0')).unwrap()).collect();

    let (mut x, mut y) = (0,0);

    for (k,v) in &signals{

        if k.starts_with("x"){
            build_output(k, *v, &mut x);
        }

        if k.starts_with("y"){
            build_output(k, *v, &mut y);
        }
    }

    let target_output = x & y;    

    let actual_output = find_output(&mut signals, &mut gates);

    println!("{} {} {} {}", x, y, target_output, actual_output);
    println!("{:08b}", target_output);
    println!("{:08b}", actual_output);
    println!("{:08b}", target_output ^ actual_output);

    0
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 24***");

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
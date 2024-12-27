use std::collections::{HashMap, HashSet};
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

    let (signal_data, mut gates) = data.split_once("\n\n").map(|(s, g)| (s, g.trim().split("\n").map(|item| item.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>())).unwrap();

    let mut signals: HashMap<&str, u8> = signal_data.split("\n").map(|line| line.split_once(": ").map(|(k,v)| (k, v.as_bytes()[0] - b'0')).unwrap()).collect();
		
    find_output(&mut signals, &mut gates)
}

fn part_2(data: &str) -> String{

    let (signal_data, gates) = data.split_once("\n\n").map(|(s, g)| (s, g.trim().split("\n").map(|item| item.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>())).unwrap();

    let mut culprits = HashSet::new();

	for i in 0..64{
		let num = format!("{:02}", i);
		let mut a = "";
		let mut b = "";
		let mut c = "";

		//Check if inputs map to the right outputs, if not they must be swapped
	    for gate in &gates{

		    if (gate[0].starts_with("x") || gate[0].starts_with("y")) && &gate[0].as_bytes()[1..] == num.as_bytes(){
				
				if gate[1] == "XOR" {
					a = gate[4];

					for sg in &gates{
						if (sg[0] == a || sg[2] == a)  && (sg[1] =="XOR"){
							if sg[1] =="XOR" && gate[0].as_bytes()[1..] != sg[4].as_bytes()[1..]{

								culprits.insert(sg[4].to_string());
								let out_wire = gate[0].replace("x","z").replace("y","z");
								culprits.insert(out_wire);
							}
						} else if (sg[0] == a || sg[2] == a) && (sg[1] == "AND") {					   
						   c = sg[4];
						}
					}
				} else if gate[1] == "AND"{
						b = gate[4];
				} 			    	
			}
		}

		// If we cannot find the output signal of the second AND gate, the XOR and AND gate outputs must be swapped
		if c == "" && a!= "" && b !="" && i > 0{
			culprits.insert(a.to_string());
			culprits.insert(b.to_string());
		}

	}

    let mut output: Vec<_> = culprits.iter().map(|x| x.clone()).collect();
    output.sort();

    output.join(",")
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

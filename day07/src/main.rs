use std::fs;
use itertools::Itertools;

fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    //Naive permuations
    let mut sum =0;
    for line in data.lines(){

        //Parse data
        let vals = line.split_once(": ").unwrap();
        let y = vals.0.parse::<i64>().unwrap();
        let args: Vec<_> = vals.1.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();

        //Define set of operations (For part 1, remove "cat")
        let ops_set = ["add", "mul"]; // "cat"];

        //Loop over all combinations of operations
        for ops in (0..args.len()-1).map(|_| ops_set).multi_cartesian_product()  {

            let mut result = args[0];

            //Execute operations
            for (i, op) in ops.iter().enumerate(){
                if op == &"mul"{
                    result *= args[i+1];
                } else if op == &"add"{
                    result += args[i+1];
                } else if op == &"cat"{
                    let mut concat = result.to_string();
                    concat.push_str(&args[i+1].to_string());
                    result = concat.parse::<i64>().unwrap();
                }
            }

            //If matching result, sum and break
            if result == y{
                sum += y;
                break;
            }
         }
    }

    //Print result
    println!("Result: {}", sum);
}
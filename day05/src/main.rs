use std::fs;
use std::collections::HashMap;

//Function to check if an ordering of numbers is invalid
fn is_invalid(nums: &[&str], after_map: &HashMap::<&str, Vec<&str>>) -> (bool, usize, usize){

    //Check inputs
    for i in 0..nums.len() {
        
        //Check if numbers are supposed to come after
        if after_map.contains_key(nums[i]){
            for j in i+1..nums.len(){
                if after_map[nums[i]].contains(&nums[j]){
                    return(true, i,j);
                }
            }
        }
    }

    //If not invalid, return false. Ids are irrelevant
    (false, 0, 0)

}

fn main() {
    let data = fs::read_to_string("src/data.txt").expect("Failed to read");

    let mut after_map = HashMap::<&str, Vec<_>>::new();

    let mut read_rules = true;
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for line in data.lines(){
        
        //Read the rules first
        if read_rules {

            if line.trim() == ""{
                read_rules = false;
                continue;
            }

            //Store rules in map
            let (before, after) = line.trim().split_once('|').unwrap();

            if after_map.contains_key(after){
                after_map.get_mut(after).unwrap().push(before);
            } else {
                after_map.insert(after, vec![before]);
            }

        //Apply the rules
        } else {
            
            let mut nums: Vec<&str> = line.trim().split(',').collect();
            let (mut invalid, mut i, mut j) = is_invalid(&nums, &after_map);

            //Check if line is in the correct order
            if !invalid{
                sum_1 += nums[nums.len()/2].parse::<i32>().unwrap();

            //If in the wrong order, keep swapping wrong ids until line is in the correct order
            } else {

                loop {
                    nums.swap(i,j);
                    (invalid, i,j) = is_invalid(&nums, &after_map);

                    if !invalid{
                        sum_2 += nums[nums.len()/2].parse::<i32>().unwrap();
                        break;
                    }
                }
            }
        }
    }

    //Print the results
    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);


}
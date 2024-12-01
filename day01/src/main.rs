use std::fs;
use std::collections::HashMap;

fn main() {
    let data = fs::read_to_string("src/data.txt").expect("Failed to read");

    let mut l_list: Vec<i32> = Vec::new();
    let mut r_list: Vec<i32> = Vec::new();
    let mut count: HashMap<i32, i32> = HashMap::new();

    //Parse input data
    for line in data.lines(){

        let splits: Vec<_> = line.trim().split("   ").collect();
        
        //Add numbers to left and right lists (Part 1)
        l_list.push(splits[0].parse::<i32>().unwrap());
        r_list.push(splits[1].parse::<i32>().unwrap());

        //Count number is in right list (Part 2)
        let number: i32 = splits[1].parse::<i32>().unwrap();
        if count.contains_key(&number){
            count.insert(number, count[&number] + 1);
        } else {
            count.insert(number, 1);
        }
    }

    //Sort lists & sum differences (Part 1)
    l_list.sort();
    r_list.sort();

    let diff: Vec<i32> = l_list.iter().zip(r_list.iter()).map(|(&l, &r)| (l-r).abs()).collect();
    let sum: i32 = diff.iter().sum();

    println!("Part 1: {}", sum);

    // Sum similarity scores (Part 2)
    let mut similarity_score: i32 = 0;
    for l in l_list {
        if count.contains_key(&l) {
            similarity_score += l*count[&l];
        }
    }

    println!("Part 2: {}", similarity_score);
}
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn list_contains(s: &str, list: &[&str]) -> bool {

    for ls in list {
        if s.contains(ls){
            return true
        }
    }
    false
}

fn part_1(data: &str) -> usize{

    let (towels, patterns) = data.split_once("\r\n\r\n").unwrap();

    let towels = towels.split(", ").collect::<Vec<&str>>();

    
    let singles = towels.clone().into_iter().filter(|&t| t.len() ==1).collect::<Vec<&str>>();
    let no_singles = ["r", "g", "b", "u", "w"].into_iter().filter(|&c | !singles.contains(&c)).collect::<Vec<&str>>();
    let mut uniques= Vec::new();

    for t in &towels{

        for u in &no_singles{
            if t.contains(u){
                uniques.push(*t);
                break;
            }
        }
    }

    let mut count = 0;

    for pattern in patterns.lines() {

        if !list_contains(pattern, &no_singles){
            count += 1;
        } else if !list_contains(pattern, &uniques){ 
        }        
        else {
            
            let mut sub_patterns = Vec::from([pattern]);
            let mut n_ways = 0;
            loop {

                if sub_patterns.is_empty() || n_ways >= 1{
                    break;
                }

                // println!("{:?}", sub_patterns);

                let current_pattern = sub_patterns.pop().unwrap();

                if !list_contains(current_pattern, &no_singles){
                    // println!("Possible!");
                    n_ways += 1;
                    break;
                } else if !list_contains(current_pattern, &uniques){ 
                    // println!("Impossible!");
                    continue;
                }  

                for t in &towels{

                    //println!("{} {:?}", t, current_pattern);
                    if t.len() > current_pattern.len() {
                        continue;
                    }
                    else if *t == current_pattern{
                        n_ways += 1;
                        break;
                    } else if *t == &current_pattern[.. t.len()] {
                        sub_patterns.push(&current_pattern[t.len()..]);
                    }
                }

            }

            if n_ways > 0{
                count += 1;
            }
        }

    }

    count

}

fn combinations<'a>(current_pattern: &'a str, towels: &[&str], cache: &mut HashMap<&'a str,usize>) -> usize {

    let mut n_ways = 0;

    if cache.contains_key(current_pattern) {
        return cache[current_pattern];
    }

    for t in towels{

        if t.len() > current_pattern.len() {
            continue;
        }
        else if *t == current_pattern{
            n_ways += 1;
            cache.insert(current_pattern, n_ways);
        } else if *t == &current_pattern[.. t.len()] {

            n_ways += combinations(&current_pattern[t.len()..], towels, cache);
            cache.insert(current_pattern, n_ways);

        }
    }
    

    n_ways
}

fn part_2(data: &str) -> usize{

    let (towels, patterns) = data.split_once("\r\n\r\n").unwrap();

    let towels = towels.split(", ").collect::<Vec<&str>>();

    let mut sum = 0;

    let mut cache = HashMap::new();

    for pattern in patterns.lines() {

        let n_ways = combinations(pattern, &towels, &mut cache);
        if n_ways > 0{
            sum += n_ways;
        }
    }

    sum
    
}

fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 19***");

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
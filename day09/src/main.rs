use std::time::Instant;
use std::fs;
use std::collections::HashSet;

fn part_1(raw_data: &String) -> u64{    
    
    let data = raw_data.as_bytes();

    let mut position = 0;
    let mut checksum = 0;

    let mut end_buf = (0,0);
    let mut end_id = raw_data.len()-1;

    for start_id in  0..data.len(){
          

        if start_id % 2 == 0{
            let num = data[start_id]-b'0';  
            for _ in 0..num{
                checksum += (start_id/2) as u64 * position;
                position += 1;
            }
        } else {

            for _ in 0..data[start_id]-b'0' {
                
                //Fill the buffer if empty                                   
                while end_buf.0 == 0 {
                    if end_id % 2 == 0{  
                        end_buf = (data[end_id]-b'0', (end_id/2) as u64);
                        end_id -= 1;
                    } else {
                        end_id -= 1;
                        continue;
                    }
                }

                checksum += end_buf.1 * position;
                end_buf.0 -= 1;
                position += 1;
            }
        }

        if start_id >= end_id{
            for _ in 0..end_buf.0{
                checksum += end_buf.1 * position;
                end_buf.0 -= 1;
                position += 1;
            }
            break;
        }
    } 

    checksum
}

fn part_2(raw_data: &String) -> u64{    
    
    let data = raw_data.as_bytes();

    let mut position = 0;
    let mut checksum = 0;

    let mut end_buf = Vec::new();
    let mut end_id = raw_data.len()-1;

    let mut moved = HashSet::new();

    for start_id in  0..data.len(){
          

        if start_id % 2 == 0 && !moved.contains(&((start_id/2) as u64)){
            let num = data[start_id]-b'0';  
            for _ in 0..num{
                checksum += (start_id/2) as u64 * position;
                position += 1;
                // print!("{}", start_id/2);
            }
        } else {

            let mut start_buf = data[start_id]-b'0';
            let mut update_buf = false;


            while start_buf > 0 {
                
                //Fill the buffer if empty                                   
                while (end_buf.is_empty() || update_buf) && end_id > 0{
                    if end_id % 2 == 0{  
                        end_buf.push((data[end_id]-b'0', (end_id/2) as u64));
                        end_id -= 1;
                        update_buf = false;
                    } else {
                        end_id -= 1;
                        continue;
                    }
                }

                update_buf = true;
                for idx in 0..end_buf.len(){
                    if end_buf[idx].0 <= start_buf && start_id/2 < end_buf[idx].1 as usize{                        
                        for _ in 0..end_buf[idx].0{
                            checksum += end_buf[idx].1 * position;
                            position += 1;
                            // print!("{}", end_buf[idx].1);
                        }
                        moved.insert(end_buf[idx].1);
                        start_buf -= end_buf[idx].0;
                        update_buf = false;
                        end_buf.remove(idx);
                        break;
                    }
                }    

                if update_buf && end_id == 0{
                    // print!("*{}*", start_buf);
                    position += start_buf as u64;
                    break;
                }            
            }
        }
    } 

    //println!("");
    checksum
}

fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 09***");

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

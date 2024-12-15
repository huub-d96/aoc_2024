use std::time::Instant;
use std::fs;
use std::collections::HashSet;

fn count_heads(x: usize, y: usize, data: &Vec<Vec<u8>>, peaks: &mut HashSet<(i32,i32)>) -> i32 {

    let directions = [(1,0), (-1,0), (0,1), (0,-1)];

    let mut sum = 0;
    for (xs, ys) in directions{
        let x_next = x as i32 + xs;
        let y_next = y as i32 + ys;

        if data[y][x]-b'0' == 9 {
            peaks.insert((x_next, y_next));
            return 1
        }

        if x_next < 0 || y_next < 0 || x_next >= data[0].len() as i32 || y_next >= data.len() as i32{
            continue;
        }

        
        if data[y_next as usize][x_next as usize] == b'.'{
            continue;
        }



        if data[y_next as usize][x_next as usize]-b'0' == data[y][x]-b'0'+1{
            sum += count_heads(x_next as usize, y_next as usize, data, peaks);
        }
    }

    sum
}

fn part_1(raw_data: &str) -> usize{

    let data: Vec<Vec<_>> = raw_data.lines().map(|line| line.trim().bytes().collect()).collect();

    let mut sum = 0;
    for (y, line) in data.iter().enumerate(){
        for (x, databyte) in line.iter().enumerate(){

            if databyte == &b'.'{
                continue;
            }
            
            let point = databyte - b'0';
            let mut peaks = HashSet::new();

            if point == 0 {
                count_heads(x, y, &data, &mut peaks);
                sum += peaks.len();
                //println!("{} {} {:?}", x, y, peaks.len());

            }
        }

    }

    sum
}

fn part_2(raw_data: &str) -> i32{
    let data: Vec<Vec<_>> = raw_data.lines().map(|line| line.trim().bytes().collect()).collect();

    let mut sum = 0;
    for (y, line) in data.iter().enumerate(){
        for (x, databyte) in line.iter().enumerate(){

            if databyte == &b'.'{
                continue;
            }
            
            let point = databyte - b'0';
            let mut peaks = HashSet::new();

            if point == 0 {
                
                sum += count_heads(x, y, &data, &mut peaks);
                //println!("{} {} {:?}", x, y, peaks.len());

            }
        }

    }

    sum
}

fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 10***");

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
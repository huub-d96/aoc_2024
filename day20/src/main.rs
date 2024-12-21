use std::fs;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Point {
    byte: u8,
    dist: u32
}

fn dijkstra(start: (usize, usize), end: (usize, usize), maze: &mut [Vec<Point>]) -> u32 {

    let mut check_points = Vec::from([start]);
    maze[start.1][start.0].dist = 0;

    loop {

        if check_points.is_empty(){
            break;
        }

        let point = {
            let (mut min, mut min_id) = (u32::MAX,0); 
            for (id, p) in check_points.iter().enumerate(){
                if maze[p.1][p.0].dist < min{
                    min = maze[p.1][p.0].dist;
                    min_id = id;
                }
            }
            check_points.remove(min_id)
        };

        for (dx, dy) in [(0,1), (0,-1), (1,0), (-1,0)]{

            let x_next = (point.0 as i32 + dx) as usize;
            let y_next = (point.1 as i32 + dy) as usize;

            if maze[y_next][x_next].byte == b'#' || maze[y_next][x_next].dist < u32::MAX {
                continue;
            }            
            maze[y_next][x_next].dist = maze[point.1][point.0].dist+1;
            check_points.push((x_next, y_next));
        }
    }

    maze[end.1][end.0].dist
}

fn part_1(data: &str, range: i32) -> usize{

    let mut start = (0,0);
    let mut end = (0,0);
    let mut maze = Vec::new();

    for (y, line) in data.lines().enumerate(){
        let mut line_vec = Vec::new();
        for (x, byte) in line.bytes().enumerate(){

            if byte == b'S'{
                start = (x,y);
            } else if byte == b'E'{
                end = (x,y)
            }
            line_vec.push(Point{byte, dist: u32::MAX});
        }
        maze.push(line_vec)
    }

    dijkstra(start, end, &mut maze);
    let mut count = 0;
    
    for (y, line) in maze.iter().enumerate(){
        for (x, p) in line.iter().enumerate(){

            if p.byte != b'#'{

                for dy in -range.. range+1 {
                    for dx in -range..range+1{

                        let x_next = x as i32 + dx ;
                        let y_next = y as i32 + dy;

                        if x_next <= 0 || x_next +1 >= maze[0].len() as i32 || y_next <=0 || y_next +1>= maze.len() as i32 {
                            continue;
                        }
            
                        if maze[y_next as usize][x_next as usize].byte != b'#' && dy.abs() + dx.abs() <= range {
                            let next_dist = maze[y_next as usize][x_next as usize].dist;

                            if next_dist < p.dist {
                                continue;
                            }

                            let saved = (next_dist-p.dist) as i32 - (dy.abs() + dx.abs());

                            if saved >= 100  {
                                count += 1;
                            }
                        }                        
                    }
                }
            }
        }
    }

count
}

fn part_2(data: &str, range: i32) -> usize {

    part_1(data, range)
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 20***");

    //Part 1
    let timer = Instant::now();
    let result_1 = part_1(&data, 2);
    let time_1 = timer.elapsed();

    println!("Part 1: {:?} - Duration: {:?}", result_1, time_1);

    //Part 2
    let timer = Instant::now();

    let result_2 = part_2(&data, 20);
    let time_2 = timer.elapsed();
    println!("Part 2: {:?} - Duration: {:?}", result_2, time_2);
}
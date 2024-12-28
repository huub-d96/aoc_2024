use std::fs;
use std::time::Instant;
use std::collections::HashSet;

struct Point{
    w: u32,
    c: u8,
}

fn dijkstra(start: (usize, usize, (i32, i32)), map: &mut [Vec<Point>] ){
    let mut points = Vec::from([start]);

    while let Some(p) = points.pop() {

        for (dx, dy) in [(1,0),(-1,0),(0,1),(0,-1)]{

            let xn = (p.0 as i32 + dx) as usize;
            let yn = (p.1 as i32 + dy) as usize;

            if map[yn][xn].c == b'#' {
                continue;
            }

            if (dx, dy) == p.2{
                if map[yn][xn].w <= map[p.1][p.0].w {
                    continue;
                }
                map[yn][xn].w = map[p.1][p.0].w + 1;            
            } else {
                if map[yn][xn].w <= map[p.1][p.0].w + 1000 {
                    continue;
                }
                map[yn][xn].w = map[p.1][p.0].w + 1001;  
            }
            points.push((xn,yn, (dx,dy)));            
        }
    }

}

fn part_1(data: &str) -> u32{

    let mut map = Vec::new();
    let mut start = (0,0, (0,0));
    let mut end = (0,0);

    for (y, line) in data.lines().enumerate(){
        let mut line_map = Vec::new();
        for (x, c) in line.bytes().enumerate(){
            

            if c == b'S'{
                start = (x,y, (1,0));
                line_map.push(Point{w: 0, c});
            } else if c == b'E'{
                end = (x,y);
                line_map.push(Point{w: u32::MAX, c});
            } else {
                line_map.push(Point{w: u32::MAX, c});
            }
        }
        map.push(line_map);
    }

    //Dijkstra
    dijkstra(start, &mut map);


    map[end.1][end.0].w
}

fn backtrack(x: usize, y: usize, d:(i32, i32), map: &Vec<Vec<Point>> ) -> HashSet<(usize, usize)>{

    if map[y][x].w == 0{
        return HashSet::from([(x,y)]);
    }

    let mut path = HashSet::from([(x,y)]);
    for (dx, dy) in [(1,0),(-1,0),(0,1),(0,-1)]{

        let xn = (x as i32 + dx) as usize;
        let yn = (y as i32 + dy) as usize;



        if map[yn][xn].w < map[y][x].w || (d == (dx, dy) && map[yn][xn].w == map[y][x].w+999){
            // println!("{:?}", backtrack(xn, yn, map));
            path.extend(&mut backtrack(xn, yn, (dx, dy), map).iter());
        }

    }
    path
}

fn part_2(data: &str) -> usize {

    let mut map = Vec::new();
    let mut start = (0,0, (0,0));
    let mut end = (0,0);

    for (y, line) in data.lines().enumerate(){
        let mut line_map = Vec::new();
        for (x, c) in line.bytes().enumerate(){
            

            if c == b'S'{
                start = (x,y, (1,0));
                line_map.push(Point{w: 0, c});
            } else if c == b'E'{
                end = (x,y);
                line_map.push(Point{w: u32::MAX, c});
            } else {
                line_map.push(Point{w: u32::MAX, c});
            }
        }
        map.push(line_map);
    }

    //Dijkstra
    dijkstra(start, &mut map);

    //Backtrack
    let path = backtrack(end.0, end.1, (0,1) ,&map);
    
    path.len()
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 16***");

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
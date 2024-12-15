use std::fs;
use std::time::Instant;

fn flood_area(x: u8, y: u8, map: &mut[Vec<(u8, bool)>]) -> (usize, usize, usize) {

    if map[y as usize][x as usize].1 {
        return (0,0,0);
    }

    let mut neighbours = Vec::new();
    for (dx, dy) in [(1,0), (-1,0), (0,1), (0,-1)]{

        let xs = x as i16 + dx;
        let ys = y as i16 + dy;

        if  xs < 0 || ys < 0 || xs >= map[0].len() as i16 || ys >= map.len() as i16 {
            continue;
        }

        if map[ys as usize][xs as usize].0 == map[y as usize][x as usize].0{
            
            neighbours.push((xs, ys));
        }
    }

    map[y as usize][x as usize].1 = true;

    let mut perimeter = 4-neighbours.len();
    let mut area = 1;
    let mut corner = if neighbours.len()==2 && neighbours[0].0 != neighbours[1].0 && neighbours[0].1 != neighbours[1].1 {
                                let mut n_corners = 0;
                                for ((hx, hy), (vx, vy)) in [((0,-1), (1,0)), ((1,0), (0,1)), ((0,1), (-1,0)), ((-1,0), (0,-1))] {

                                    let hxs = x as i16 + hx;
                                    let hys = y as i16 + hy;
                                    let vxs = x as i16 + vx;
                                    let vys = y as i16 + vy;
                                    let dxs = x as i16 + hx + vx;
                                    let dys = y as i16 + hy + vy;
                            
                                    if  hxs < 0 || hys < 0 || hxs >= map[0].len() as i16 || hys >= map.len() as i16 || 
                                        vxs < 0 || vys < 0 || vxs >= map[0].len() as i16 || vys >= map.len() as i16 || 
                                        dxs < 0 || dys < 0 || dxs >= map[0].len() as i16 || dys >= map.len() as i16 {
                                        continue;
                                    }

                                    if map[hys as usize][hxs as usize].0 == map[y as usize][x as usize].0 && map[vys as usize][vxs as usize].0 == map[y as usize][x as usize].0 && map[dys as usize][dxs as usize].0 != map[y as usize][x as usize].0{
                                        n_corners += 1;
                                    }                                   
                                }

                                n_corners+1
                            } 
                            else if neighbours.len() == 0 {4}
                            else if neighbours.len() == 1 {2}
                            else {
                                let mut n_corners = 0;
                                for ((hx, hy), (vx, vy)) in [((0,-1), (1,0)), ((1,0), (0,1)), ((0,1), (-1,0)), ((-1,0), (0,-1))] {

                                    let hxs = x as i16 + hx;
                                    let hys = y as i16 + hy;
                                    let vxs = x as i16 + vx;
                                    let vys = y as i16 + vy;
                                    let dxs = x as i16 + hx + vx;
                                    let dys = y as i16 + hy + vy;
                            
                                    if  hxs < 0 || hys < 0 || hxs >= map[0].len() as i16 || hys >= map.len() as i16 || 
                                        vxs < 0 || vys < 0 || vxs >= map[0].len() as i16 || vys >= map.len() as i16 || 
                                        dxs < 0 || dys < 0 || dxs >= map[0].len() as i16 || dys >= map.len() as i16 {
                                        continue;
                                    }

                                    if map[hys as usize][hxs as usize].0 == map[y as usize][x as usize].0 && map[vys as usize][vxs as usize].0 == map[y as usize][x as usize].0 && map[dys as usize][dxs as usize].0 != map[y as usize][x as usize].0{
                                        n_corners += 1;
                                    }                                   
                                }

                                n_corners
                            };
    
    for (xs, ys) in neighbours {
        let (p, a, c) = flood_area(xs as u8, ys as u8, map);
        perimeter += p;
        area += a;
        corner += c;
    }

    (perimeter, area, corner)    
}

fn part_1(data: &str) -> usize {

    let mut map = data.lines().map(|line| line.bytes().map(|byte| (byte-b'A', false)).collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let mut price = 0;
    for y in 0..map.len(){
        for x in 0..map[0].len(){

            if !map[y][x].1{
                let (perimeter, area, _) = flood_area(x as u8, y as u8, &mut map);

                price += perimeter*area;
            }
        }
    }
    price
}

fn part_2(data: &str) -> usize {

    let mut map = data.lines().map(|line| line.bytes().map(|byte| (byte-b'A', false)).collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let mut price = 0;
    for y in 0..map.len(){
        for x in 0..map[0].len(){

            if !map[y][x].1{
                let (_, area, corners) = flood_area(x as u8, y as u8, &mut map);

                // println!("{} {} {}", map[y][x].0, area, corners);

                price += corners*area;
            }
        }
    }
    price
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 12***");

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
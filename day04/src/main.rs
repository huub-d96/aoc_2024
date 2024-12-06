use std::fs;


fn check(x: usize, y: usize, x_off: i32, y_off: i32, letter: char, data: &[Vec<char>]) -> bool{

    let x_target = x as i32 + x_off;
    let y_target = y as i32 + y_off;

    if x_target < 0 || y_target < 0 || x_target >= data[0].len() as i32 || y_target >=data.len() as i32{
       return false;
    }

     data[y_target as usize][x_target as usize] == letter
}
fn main() {
    let raw_data= fs::read_to_string("src/data.txt").expect("Failed to read");
    let data:  Vec<Vec<char>> = raw_data.lines().map(|line| line.trim().chars().collect()).collect();

    let directions = [(0,1), (1,1), (1,0),(1,-1), (0,-1),   (-1,-1),   (-1,0), (-1,1)];

    let x_max = data[0].len();
    let y_max = data.len();

    let mut count_1 =0;
    let mut count_2 =0;
    for y in 0..y_max {
        for x in 0..x_max{

            //Part 1
            if data[y][x] == 'X' {
                for dir in directions{
                    if check(x, y,  dir.0, dir.1, 'M', &data) && 
                       check(x, y, dir.0*2, dir.1*2, 'A', &data) && 
                       check(x, y, dir.0*3, dir.1*3, 'S', &data) {
                        count_1 += 1;
                    }
                }
            }

            //Part 2
            if data[y][x] == 'A' && (
                (check(x, y,  1, 1, 'M', &data) && check(x, y,  -1, -1, 'S', &data) && check(x, y,  -1, 1, 'M', &data) && check(x, y,  1, -1, 'S', &data)) ||
                (check(x, y,  1, 1, 'M', &data) && check(x, y,  -1, -1, 'S', &data) && check(x, y,  -1, 1, 'S', &data) && check(x, y,  1, -1, 'M', &data)) ||
                (check(x, y,  1, 1, 'S', &data) && check(x, y,  -1, -1, 'M', &data) && check(x, y,  -1, 1, 'S', &data) && check(x, y,  1, -1, 'M', &data)) ||
                (check(x, y,  1, 1, 'S', &data) && check(x, y,  -1, -1, 'M', &data) && check(x, y,  -1, 1, 'M', &data) && check(x, y,  1, -1, 'S', &data))
            ){
                count_2 += 1;
            }
                
                        
        }
    }

    println!("Part 1: {}", count_1);
    println!("Part 2: {}", count_2);


}
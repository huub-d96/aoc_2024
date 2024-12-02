use std::fs;

//Check if a input code is invalid
fn is_invalid(nums: &Vec<i32>) -> (bool, usize) {
    let mut inc: bool = false;
    let mut invalid: bool = false;
    
    let mut i: usize = 0;
    while i < nums.len()-1{
        
        let diff: i32 = nums[i] - nums[i+1];

        //Set increase/decrease flag
        if i == 0 {
            inc = diff < 0;
        }  
        
        //Sequential codes cannot be more than 1 or 3 apart and must follow increasing/decreasing pattern
        if diff.abs() < 1 || diff.abs() > 3 || (diff<0) != inc {
            invalid = true;
            break;
        }
        i += 1;
    }

    return (invalid, i);

}

fn main() {
    let data = fs::read_to_string("src/data.txt").expect("Failed to read");
    let mut sum_p1: u16 = 0;
    let mut sum_p2: u16 = 0;

    //Loop over input lines
    for line in data.lines(){
        
        let nums: Vec<i32> = line.trim().split(' ').map(|num| num.parse::<i32>().expect("Parse failed")).collect();
        
        //Check if line is invalid (Part 1)
        let (invalid, idx) = is_invalid(&nums);

        //If the line is invalid, check if subsets of the line are valid by checking 3 subsets surrounding the erroneous id (Part 2)
        if invalid{

            let mut damped_valid: bool = false;
            for j in 0..3 {
                
                if idx+j > 0 {
                    let mut sub_nums: Vec<i32> = nums.clone();
                    sub_nums.remove(idx + j -1);
                    let (damped_invalid, _) = is_invalid(&sub_nums);
                
                    if !damped_invalid{
                        damped_valid = true;
                    }
                }
            }

            //Sum if subset is valid
            if damped_valid {
                sum_p2 += 1;
            }
        } else {
            
            //Sum if line is valid
            sum_p1 +=1;
        }
    }

    println!{"Part 1: {}", sum_p1}
    println!{"Part 2: {}", sum_p1 + sum_p2}

}
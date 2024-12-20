use std::fs;
use std::time::Instant;


#[derive(Default)]
struct Emulator {

    registers : [u64;3],
    pointer: usize,
    output: Vec<u64>
}

impl Emulator {

    fn value(&self, operand: u64) -> u64{
        
        if operand <= 3 {
            operand
        } else if operand <= 6 {
            self.registers[(operand-4) as usize]
        } else {
            panic!("No valid operand!")
        }
    }
    
    fn execute(&mut self, opcode: u64, operand: u64){

        match opcode {
            0 => self.registers[0] >>= self.value(operand),
            1 => self.registers[1] ^= operand,
            2 => self.registers[1] = self.value(operand) % 8,
            3 => if self.registers[0] != 0 { self.pointer = operand as usize },
            4 => self.registers[1] ^= self.registers[2],
            5 => self.output.push(self.value(operand)%8),
            6 => self.registers[1] = self.registers[0] >> self.value(operand),
            7 => self.registers[2] = self.registers[0] >> self.value(operand),
            _ => panic!("Opcode does not exits")
        }

    } 

    fn run(&mut self, program: &[u64]) -> Vec<u64> {

        loop{

            if self.pointer >= program.len() {
                break;
            }

            // println!("{:?}", self.registers);

            let p = program[self.pointer];
            let o = program[self.pointer+1];
            self.pointer += 2;
            self.execute(p, o);
        }


        self.output.clone()
    }    
}



fn part_1(data: &str) -> Vec<u64> {

    let mut regs = [0;3];
    let mut program = Vec::new();

    for (i, line) in data.lines().enumerate(){
       
        if i < 3 {
            regs[i] = line.split_once(": ").unwrap().1.parse::<u64>().unwrap();
        } else if i == 4 {
            program = line.split_once(": ").unwrap().1.trim().split(',').map(|c|  c.parse::<u64>().unwrap() ).collect::<Vec<u64>>();
        }
    }

    let mut emulator = Emulator {registers: regs, ..Default::default()};

    emulator.run(&program)
}

fn part_2(data: &str) -> u64  {

    let mut program = Vec::new();

    for (i, line) in data.lines().enumerate(){
        if i == 4 {
            program = line.split_once(": ").unwrap().1.trim().split(',').map(|c|  c.parse::<u64>().unwrap() ).collect::<Vec<u64>>();
        }
    }

    let mut octs = program.iter().map(|_| 0).collect::<Vec<u64>>();
    octs[0] = 1;

    let mut oct_id = 0;
    let mut a_reg;
    loop {

        a_reg = 0;        
        for (i, oct) in octs.iter().enumerate(){
            a_reg += oct << ((octs.len()- i -1)*3);
        }

        let mut emulator = Emulator {registers: [a_reg, 0, 0], ..Default::default()};

        let output = emulator.run(&program);

        if program[program.len()-oct_id-1] == output[program.len()-oct_id-1]{
            oct_id += 1;
            if oct_id >= program.len(){
                break;
            }
        } else {

            if  octs[oct_id] == 7{
                octs[oct_id] = 0;
                oct_id -= 1;
            }

            octs[oct_id] += 1;

        }
    }


    a_reg
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 17***");

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
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, Debug)]
enum Operation {
    Break,
    Add(bool, bool),
    Multiply(bool, bool),
    Input,
    Output(bool),
    JIT(bool, bool),
    JIF(bool, bool),
    LT(bool, bool),
    EQ(bool, bool),
}

fn main() {
    let input = 5;
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    let mut line: String = String::new();
    if reader.read_to_string(&mut line).is_ok() {
        let mut opcodes = line
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut pos: usize = 0;
        'main_loop: loop {
            match read_op(opcodes[pos]) {
                Operation::Add(m1, m2) => {
                    let v1 = read_value(pos, 1, m1, &opcodes);
                    let v2 = read_value(pos, 2, m2, &opcodes);
                    let v3 = opcodes[pos + 3] as usize;
                    opcodes[v3] = v1 + v2;
                    pos += 4;
                }
                Operation::Multiply(m1, m2) => {
                    let v1 = read_value(pos, 1, m1, &opcodes);
                    let v2 = read_value(pos, 2, m2, &opcodes);
                    let v3 = opcodes[pos + 3] as usize;
                    opcodes[v3] = v1 * v2;
                    pos += 4;
                }
                Operation::Input => {
                    let v3 = opcodes[pos + 1] as usize;
                    opcodes[v3] = input;
                    pos += 2;
                }
                Operation::Output(m1) => {
                    let v1 = read_value(pos, 1, m1, &opcodes);
                    println!("Output: {}", v1);
                    pos += 2;
                }
                Operation::JIT(m1, m2) => {
                    let v1 = read_value(pos, 1, m1, &opcodes);
                    let v2 = read_value(pos, 2, m2, &opcodes);
                    if v1 != 0 {
                        pos = v2 as usize;
                    } else {
                        pos += 3;
                    }
                }
                Operation::JIF(m1, m2) => {
                    let v1 = read_value(pos, 1, m1, &opcodes);
                    let v2 = read_value(pos, 2, m2, &opcodes);
                    if v1 == 0 {
                        pos = v2 as usize;
                    } else {
                        pos += 3;
                    }
                }
                Operation::LT(m1, m2) => {
                    let v1 = read_value(pos, 1, m1, &opcodes);
                    let v2 = read_value(pos, 2, m2, &opcodes);
                    let v3 = opcodes[pos + 3] as usize;
                    opcodes[v3] = if v1 < v2 { 1 } else { 0 };
                    pos += 4;
                }
                Operation::EQ(m1, m2) => {
                    let v1 = read_value(pos, 1, m1, &opcodes);
                    let v2 = read_value(pos, 2, m2, &opcodes);
                    let v3 = opcodes[pos + 3] as usize;
                    opcodes[v3] = if v1 == v2 { 1 } else { 0 };
                    pos += 4;
                }
                Operation::Break => {
                    break 'main_loop;
                }
            }
        }
    }
}

fn read_op(value: i32) -> Operation {
    let op;
    let mut modes: Vec<bool> = vec![false; 3];
    if value < 0 {
        Operation::Break
    } else {
        if value >= 10 {
            op = value % 100;
            if value >= 100 {
                modes[0] = (value / 100) % 10 == 1;
                if value >= 1000 {
                    modes[1] = (value / 1000) % 10 == 1;
                    if value >= 10000 {
                        modes[2] = (value / 1000) % 10 == 1;
                    }
                }
            }
        } else {
            op = value;
        }
        match op {
            99 => Operation::Break,
            1 => Operation::Add(modes[0], modes[1]),
            2 => Operation::Multiply(modes[0], modes[1]),
            3 => Operation::Input,
            4 => Operation::Output(modes[0]),
            5 => Operation::JIT(modes[0], modes[1]),
            6 => Operation::JIF(modes[0], modes[1]),
            7 => Operation::LT(modes[0], modes[1]),
            8 => Operation::EQ(modes[0], modes[1]),
            _ => Operation::Break,
        }
    }
}

fn read_value(pos: usize, offset: usize, mode: bool, opcodes: &Vec<i32>) -> i32 {
    if mode {
        opcodes[pos + offset]
    } else {
        opcodes[opcodes[pos + offset] as usize]
    }
}

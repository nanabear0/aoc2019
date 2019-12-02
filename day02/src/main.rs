use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::option::Option;

fn main() {
    println!("part1: {}", part1(12, 2));
    'noun: for noun in 0..=99 {
        for verb in 0..=99 {
            if part1(noun, verb) == 19_690_720 {
                println!("part2: {}", 100 * noun + verb);
                break 'noun;
            }
        }
    }
}

fn read_op(opcodes: &[usize], pos: usize) -> Option<(usize, usize, usize, usize)> {
    if pos + 3 > opcodes.len() {
        None
    } else {
        Some((
            opcodes[pos],
            opcodes[pos + 1],
            opcodes[pos + 2],
            opcodes[pos + 3],
        ))
    }
}

fn part1(noun: usize, verb: usize) -> usize {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    let mut line: String = String::new();
    if reader.read_to_string(&mut line).is_ok() {
        let mut opcodes = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        opcodes[1] = noun;
        opcodes[2] = verb;
        let mut pos: usize = 0;
        'main_loop: loop {
            if let Some((op, pos1, pos2, pos3)) = read_op(&opcodes, pos) {
                match op {
                    99 => {
                        break 'main_loop;
                    }
                    1 => {
                        opcodes[pos3] = opcodes[pos1] + opcodes[pos2];
                    }
                    2 => {
                        opcodes[pos3] = opcodes[pos1] * opcodes[pos2];
                    }
                    _ => {}
                };
                pos += 4;
            } else {
                break 'main_loop;
            }
        }
        opcodes[0]
    } else {
        0
    }
}

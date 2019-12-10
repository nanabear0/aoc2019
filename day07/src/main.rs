use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::option::Option::{None, Some};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

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

#[derive(Debug)]
struct IntCodeMachine {
    recv: Option<Arc<Mutex<Receiver<i32>>>>,
    send: Option<Arc<Mutex<Sender<i32>>>>,
    opcodes: Vec<i32>,
    pos: usize,
    last_output: Option<i32>,
}

impl IntCodeMachine {
    pub fn new(file: &str) -> IntCodeMachine {
        let mut reader = BufReader::new(File::open(file).unwrap());
        let mut line: String = String::new();
        reader.read_to_string(&mut line).ok();
        IntCodeMachine {
            recv: None,
            send: None,
            opcodes: line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
            pos: 0,
            last_output: None,
        }
    }
    fn run(&mut self) -> i32 {
        while self.step() {}
        self.last_output.unwrap_or(std::i32::MIN)
    }
    fn step(&mut self) -> bool {
        match self.read_op() {
            Operation::Add(m1, m2) => {
                let v1 = self.read_value(1, m1);
                let v2 = self.read_value(2, m2);
                let v3 = self.opcodes[self.pos + 3] as usize;
                self.opcodes[v3] = v1 + v2;
                self.pos += 4;
                true
            }
            Operation::Multiply(m1, m2) => {
                let v1 = self.read_value(1, m1);
                let v2 = self.read_value(2, m2);
                let v3 = self.opcodes[self.pos + 3] as usize;
                self.opcodes[v3] = v1 * v2;
                self.pos += 4;
                true
            }
            Operation::Input => {
                if let Some(iq) = &mut self.recv {
                    if let Ok(input) = iq.lock().unwrap().recv() {
                        let v3 = self.opcodes[self.pos + 1] as usize;
                        self.opcodes[v3] = input;
                        self.pos += 2;
                    }
                }
                true
            }
            Operation::Output(m1) => {
                let v1 = self.read_value(1, m1);
                if let Some(iq) = &mut self.send {
                    iq.lock().unwrap().send(v1).ok();
                    self.last_output = Some(v1);
                    self.pos += 2;
                }
                true
            }
            Operation::JIT(m1, m2) => {
                let v1 = self.read_value(1, m1);
                let v2 = self.read_value(2, m2);
                if v1 != 0 {
                    self.pos = v2 as usize;
                } else {
                    self.pos += 3;
                }
                true
            }
            Operation::JIF(m1, m2) => {
                let v1 = self.read_value(1, m1);
                let v2 = self.read_value(2, m2);
                if v1 == 0 {
                    self.pos = v2 as usize;
                } else {
                    self.pos += 3;
                }
                true
            }
            Operation::LT(m1, m2) => {
                let v1 = self.read_value(1, m1);
                let v2 = self.read_value(2, m2);
                let v3 = self.opcodes[self.pos + 3] as usize;
                self.opcodes[v3] = if v1 < v2 { 1 } else { 0 };
                self.pos += 4;
                true
            }
            Operation::EQ(m1, m2) => {
                let v1 = self.read_value(1, m1);
                let v2 = self.read_value(2, m2);
                let v3 = self.opcodes[self.pos + 3] as usize;
                self.opcodes[v3] = if v1 == v2 { 1 } else { 0 };
                self.pos += 4;
                true
            }
            Operation::Break => false,
        }
    }
    pub fn read_value(&self, offset: usize, mode: bool) -> i32 {
        if mode {
            self.opcodes[self.pos + offset]
        } else {
            self.opcodes[self.opcodes[self.pos + offset] as usize]
        }
    }

    fn read_op(&self) -> Operation {
        let op;
        let mut modes: Vec<bool> = vec![false; 3];
        let value = self.opcodes[self.pos];
        if value < 0 {
            Operation::Break
        } else {
            op = value % 100;
            modes[0] = (value / 100) % 10 == 1;
            modes[1] = (value / 1000) % 10 == 1;
            modes[2] = (value / 1000) % 10 == 1;
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
}

fn run_pipeline(range: std::ops::Range<i32>) -> Option<(Vec<i32>, i32)> {
    range
        .permutations(5)
        .map(|i| {
            let mut machines = vec![];
            let mut channels: Vec<(Sender<i32>, Receiver<i32>)> = vec![];
            for j in 0..5 {
                machines.push(Arc::new(Mutex::new(IntCodeMachine::new("input.txt"))));
                channels.push(channel());
                channels[j].0.send(i[(j + 1) % 5]).ok();
            }
            channels[4].0.send(0).ok();
            for j in 0..5 {
                let ch = channels.remove(0);
                machines[j].lock().unwrap().send = Some(Arc::new(Mutex::new(ch.0)));
                machines[(j + 1) % 5].lock().unwrap().recv = Some(Arc::new(Mutex::new(ch.1)));
            }
            let v: Vec<JoinHandle<i32>> = machines
                .into_iter()
                .map(|i| thread::spawn(move || i.lock().unwrap().run()))
                .collect();
            (
                i,
                v.into_iter()
                    .map(|h| h.join().ok().unwrap_or(std::i32::MIN))
                    .last()
                    .unwrap_or(std::i32::MIN),
            )
        })
        .max_by(|(_, x), (_, y)| x.cmp(y))
}
fn main() {
    println!("{:?}", run_pipeline(0..5));
    println!("{:?}", run_pipeline(5..10));
}

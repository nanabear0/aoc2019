use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let sum: i32 = reader
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .map(p2)
        .sum();
    println!("{}", sum);
}

fn p2(module: i32) -> i32 {
    let v = p1(module);
    if v <= 0 {
        return 0;
    }
    v + p2(v)
}

fn p1(module: i32) -> i32 {
    module / 3 - 2
}

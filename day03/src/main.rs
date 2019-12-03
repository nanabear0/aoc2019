use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, Debug)]
enum Grid {
    Wire(usize, usize),
    Cross(usize),
    Central,
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut grid: HashMap<(i32, i32), Grid> = HashMap::new();
    grid.insert((0, 0), Grid::Central);
    for (wire, line) in reader.lines().map(|x| x.unwrap()).enumerate() {
        let mut position = (0, 0);
        let mut length = 0;
        for op in line.split(',') {
            let distance: i32 = op[1..].parse().unwrap();
            let direction = match op[..1].as_ref() {
                "U" => (0, 1, 1),
                "R" => (1, 0, 1),
                "D" => (0, 1, -1),
                "L" => (1, 0, -1),
                _ => (0, 0, 0),
            };
            for y in 1..=distance {
                length += 1;
                let index = (
                    position.0 + direction.0 * direction.2 * y,
                    position.1 + direction.1 * direction.2 * y,
                );
                if let Some(v) = grid.get_mut(&index) {
                    match v {
                        Grid::Wire(w, l2) => {
                            if *w != wire {
                                *v = Grid::Cross(*l2 + length);
                            }
                        }
                        _ => panic!("mehistan"),
                    }
                } else {
                    grid.insert(index, Grid::Wire(wire, length));
                }
            }
            position = (
                position.0 + direction.0 * direction.2 * distance,
                position.1 + direction.1 * direction.2 * distance,
            );
        }
    }
    let part1 = grid
        .iter()
        .filter_map(|(key, val)| match val {
            Grid::Cross(_) => Some(key.0.abs() + key.1.abs()),
            _ => None,
        })
        .min();
    println!("part1: {:?}", part1);
    let part2 = grid
        .values()
        .filter_map(|v| match v {
            Grid::Cross(x) => Some(x),
            _ => None,
        })
        .min();
    println!("part2: {:?}", part2);
}

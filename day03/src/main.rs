use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::option::Option;

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
        let mut pos = (0, 0); // x, y
        let mut l = 0;
        for op in line.split(',') {
            let mode: String = op[..1].to_string();
            let dist: i32 = op[1..].parse().unwrap();
            match mode.as_ref() {
                "U" => {
                    for y in (1..=dist).map(|v| v + pos.1) {
                        l += 1;
                        if let Some(v) = grid.get_mut(&(pos.0, y)) {
                            match v {
                                Grid::Wire(w, dist) => {
                                    if *w != wire {
                                        *v = Grid::Cross(*dist + l);
                                    }
                                }
                                Grid::Cross(_) => {
                                    // NANI?
                                }
                                Grid::Central => {
                                    // NANI?
                                }
                            }
                        } else {
                            grid.insert((pos.0, y), Grid::Wire(wire, l));
                        }
                    }
                    pos.1 = pos.1 + dist;
                }
                "R" => {
                    for x in (1..=dist).map(|v| v + pos.0) {
                        l += 1;
                        if let Some(v) = grid.get_mut(&(x, pos.1)) {
                            match v {
                                Grid::Wire(w, dist) => {
                                    if *w != wire {
                                        *v = Grid::Cross(*dist + l);
                                    }
                                }
                                Grid::Cross(_) => {
                                    // NANI?
                                }
                                Grid::Central => {
                                    // NANI?
                                }
                            }
                        } else {
                            grid.insert((x, pos.1), Grid::Wire(wire, l));
                        }
                    }
                    pos.0 = pos.0 + dist;
                }
                "D" => {
                    for y in (1..=dist).map(|v| pos.1 - v) {
                        l += 1;
                        if let Some(v) = grid.get_mut(&(pos.0, y)) {
                            match v {
                                Grid::Wire(w, dist) => {
                                    if *w != wire {
                                        *v = Grid::Cross(*dist + l);
                                    }
                                }
                                Grid::Cross(_) => {
                                    // NANI?
                                }
                                Grid::Central => {
                                    // NANI?
                                }
                            }
                        } else {
                            grid.insert((pos.0, y), Grid::Wire(wire, l));
                        }
                    }
                    pos.1 = pos.1 - dist;
                }
                "L" => {
                    for x in (1..=dist).map(|v| pos.0 - v) {
                        l += 1;
                        if let Some(v) = grid.get_mut(&(x, pos.1)) {
                            match v {
                                Grid::Wire(w, dist) => {
                                    if *w != wire {
                                        *v = Grid::Cross(*dist + l);
                                    }
                                }
                                Grid::Cross(_) => {
                                    // NANI?
                                }
                                Grid::Central => {
                                    // NANI?
                                }
                            }
                        } else {
                            grid.insert((x, pos.1), Grid::Wire(wire, l));
                        }
                    }
                    pos.0 = pos.0 - dist;
                }
                _ => {}
            }
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

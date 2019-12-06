use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut planets: HashMap<String, (i32, HashSet<String>)> = HashMap::new();
    for p in reader.lines().map(|x| {
        x.unwrap()
            .trim()
            .split(')')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }) {
        let entry = planets
            .entry(p[0].to_owned())
            .or_insert((0, HashSet::new()));
        entry.1.insert(p[1].to_owned());
        planets
            .entry(p[1].to_owned())
            .or_insert((0, HashSet::new()));
    }
    let mut job_list: Vec<(String, i32)> = vec![];
    job_list.push(("COM".to_owned(), -1));
    'main_loop: loop {
        let mut new_job_list: Vec<(String, i32)> = vec![];
        for i in job_list.iter_mut() {
            if let Some(p) = planets.get_mut(&i.0) {
                p.0 = i.1 + 1;
                for i in p.1.iter() {
                    new_job_list.push((i.clone(), p.0));
                }
            }
        }
        if new_job_list.is_empty() {
            break 'main_loop;
        } else {
            job_list = new_job_list;
        }
    }
    let checksum: i32 = planets.values().map(|x| x.0).sum();
    println!("part1: {}", checksum);
}

fn part2() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut planets: HashMap<String, String> = HashMap::new();
    for p in reader.lines().map(|x| {
        x.unwrap()
            .trim()
            .split(')')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }) {
        planets.insert(p[1].to_owned(), p[0].to_owned());
    }
    let mut set: HashSet<String> = HashSet::new();
    let mut start = "YOU".to_string();
    while let Some(next) = planets.get(&start) {
        set.insert(next.clone());
        start = next.clone();
    }
    start = "SAN".to_string();
    while let Some(next) = planets.get(&start) {
        if set.contains(next) {
            set.remove(next);
        } else {
            set.insert(next.clone());
        }
        start = next.clone();
    }
    println!("part2: {}", set.len());
}

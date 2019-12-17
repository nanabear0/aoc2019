use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    let mut line: String = String::new();
    reader.read_to_string(&mut line).ok();
    let width = 25;
    let height = 6;

    let pixels = line
        .trim()
        .chars()
        .map(|c: char| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let part1 = (0..(pixels.len() / (width * height)))
        .map(|l| {
            pixels
                .iter()
                .skip(l * width * height)
                .take(width * height)
                .fold((0, 0, 0), |acc, x| match x {
                    0 => (acc.0 + 1, acc.1, acc.2),
                    1 => (acc.0, acc.1 + 1, acc.2),
                    2 => (acc.0, acc.1, acc.2 + 1),
                    _ => acc,
                })
        })
        .min_by(|x, y| x.0.cmp(&y.0))
        .unwrap();
    println!("part1: {:?}", part1.1 * part1.2);
    let final_layer = (0..(pixels.len() / (width * height)))
        .map(|l| {
            pixels
                .iter()
                .skip(l * width * height)
                .take(width * height)
                .collect::<Vec<&u32>>()
        })
        .fold(vec![2; width * height], |acc, x| {
            acc.iter()
                .zip(x.iter())
                .map(|(x, y)| match 2.cmp(x) {
                    Ordering::Equal => y,
                    _ => x,
                })
                .copied()
                .collect::<Vec<u32>>()
        });
    println!("part2 :");
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                if final_layer[y * width + x] == 1 {
                    '█'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
}

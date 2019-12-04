use rayon::prelude::*;

fn main() {
    let input: (u32, u32) = (272_091, 815_432);
    let c = (input.0..input.1)
        .into_par_iter()
        .filter(check_pass)
        .count();
    println!("{}", c);
}

fn check_pass(pass: &u32) -> bool {
    let mut acc = (true, false);
    let mut last_d: u32 = 0;
    let mut repeat_count: u32 = 1;
    for x in pass
        .to_string()
        .chars()
        .map(|x| char::to_digit(x, 10).unwrap())
    {
        if last_d == x {
            repeat_count += 1;
        } else {
            acc.1 = acc.1 || repeat_count == 2;
            repeat_count = 1;
            acc.0 = acc.0 && last_d <= x;
            last_d = x;
        }
    }
    acc.1 = acc.1 || repeat_count == 2;
    acc.0 && acc.1
}

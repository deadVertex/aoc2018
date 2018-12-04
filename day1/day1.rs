use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    // Part 1
    let initial_value = 0;
    let total: i32 = s
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|word| match word.parse::<i32>() {
            Ok(x) => x,
            Err(_) => 0,
        }).fold(initial_value, |acc, x| acc + x);

    print!("Part 1 result = {}\n", total);

    // Part 2
    let mut sum = 0;
    let mut sums = HashSet::new();
    let deltas: Vec<i32> = s
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|word| match word.parse::<i32>() {
            Ok(x) => Some(x),
            Err(_) => None,
        }).collect();

    let mut cyclic_iter = deltas.iter().cycle();
    loop {
        let delta = cyclic_iter.next().unwrap();
        sum += delta;
        if sums.contains(&sum) {
            println!("Part 2 result = {}", sum);
            break;
        } else {
            sums.insert(sum);
        }
    }
}

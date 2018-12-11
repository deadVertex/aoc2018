use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn react(chars: &mut Vec<char>) {
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() {
            let x = chars[i];
            let y = chars[i + 1];
            if x.to_lowercase().to_string() == y.to_lowercase().to_string() {
                if x != y {
                    // Same letter but different case

                    // Perf is terrible because we are constantly shuffling down elements in
                    // the array
                    chars.remove(i);
                    chars.remove(i); // Acutally removing i+1
                    continue;
                }
            }
        } else {
            break;
        }

        i += 1;
    }
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut chars: Vec<char> = s.chars().filter(|x| *x != '\n').collect();
    let part2_chars = chars.clone();

    // Part 1
    loop {
        let previous_length = chars.len();

        react(&mut chars);

        if chars.len() == previous_length {
            println!("Day 5 part 1 result {}", chars.len());
            break; // No more reactions
        }
    }

    // Part 2
    let mut part2_result: Option<usize> = None;
    for c in b'a'..=b'z' {
        let lower = c as char;
        let upper = lower.to_uppercase().nth(0).unwrap();
        let mut removed: Vec<char> = part2_chars
            .clone()
            .into_iter()
            .filter(|&x| x != lower && x != upper)
            .collect();

        loop {
            let previous_length = removed.len();

            react(&mut removed);

            if removed.len() == previous_length {
                if part2_result == None || removed.len() < part2_result.unwrap() {
                    part2_result = Some(removed.len());
                }
                break; // No more reactions
            }
        }
    }

    println!("Day 5 part 2 result is {}", part2_result.unwrap());
}

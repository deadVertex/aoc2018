use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn compute_mask(a: &str, b: &str) -> u32 {
    if a.len() != b.len() {
        panic!("Mismatching string lengths");
    }
    if a.len() > 31 {
        panic!("Cannot fit mask into 32-bit integer");
    }

    let mut result = 0;
    for p in a.char_indices() {
        if p.1 != b.chars().nth(p.0).unwrap() {
            // Set corresponding bit
            result |= 1 << p.0;
        }
    }

    result
}

fn count_set_bits(v: u32) -> u32 {
    let mut result = 0;
    for i in 0..32 {
        if ((v >> i) & 1) == 1 {
            result += 1;
        }
    }

    result
}

fn find_first_set_bit(v: u32) -> Option<u32> {
    let mut result = None;
    for i in 0..32 {
        if ((v >> i) & 1) == 1 {
            result = Some(i);
            break;
        }
    }

    result
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

    let box_ids: Vec<&str> = s.lines().collect();

    // Part 1
    let mut count_a = 0;
    let mut count_b = 0;
    for box_id in &box_ids {
        let mut characters: HashMap<char, i32> = HashMap::new();
        box_id.chars().for_each(|c| {
            let count = characters.entry(c).or_insert(0);
            *count += 1;
        });

        let a = characters.values().filter(|v| **v == 2).count();
        let b = characters.values().filter(|v| **v == 3).count();

        if a > 0 {
            count_a += 1;
        }
        if b > 0 {
            count_b += 1;
        }
    }

    let hash = count_a * count_b;

    println!("Day 2 Part 1 result is {}", hash);

    // Part 2

    let mut id_found = false;
    for id in &box_ids {
        for other_id in &box_ids {
            let mask = compute_mask(id, other_id);
            let bit_count = count_set_bits(mask);
            if bit_count == 1 {
                let mut common_letters = String::new();
                for i in 0..31 {
                    if ((mask >> i) & 1) == 0 {
                        if let Some(letter) = id.chars().nth(i) {
                            common_letters.push(letter);
                        } else {
                            break;
                        }
                    }
                }
                let index = find_first_set_bit(mask).unwrap();
                let char0 = id.chars().nth(index as usize).unwrap();
                let char1 = other_id.chars().nth(index as usize).unwrap();
                println!("Day 2 Part 2 result");
                println!("IDs {} and {}", id, other_id);
                println!("Index {} characters {} and {}", index, char0, char1);
                println!("Common letters are {}", common_letters);
                id_found = true;
                break;
            }
        }

        if id_found {
            break;
        }
    }
}

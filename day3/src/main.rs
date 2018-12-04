use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate scan_fmt;

struct Claim {
    id: usize,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
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

    let claims: Vec<Claim> = s
        .lines()
        .map(|line| {
            let (id, x, y, w, h) = scan_fmt!(
                line,
                "#{d} @ {d}, {d}: {d}x{d}",
                usize,
                usize,
                usize,
                usize,
                usize
            );
            let max_x = x.unwrap() + w.unwrap();
            let max_y = y.unwrap() + h.unwrap();
            Claim {
                id: id.unwrap(),
                min_x: x.unwrap(),
                min_y: y.unwrap(),
                max_x,
                max_y,
            }
        }).collect();

    let fabric_width = 1000;
    let mut fabric: Vec<usize> = Vec::new();
    fabric.resize(fabric_width * fabric_width, 0);

    for claim in &claims {
        for y in claim.min_y..claim.max_y {
            for x in claim.min_x..claim.max_x {
                let index = y * fabric_width + x;
                fabric[index] += 1;
            }
        }
    }

    let overlap = fabric.iter().filter(|x| **x > 1).count();
    println!("Day3 Part 1 result = {}", overlap);

    // Part 2
    for claim in &claims {
        let mut claim_overlaps = false;
        for y in claim.min_y..claim.max_y {
            for x in claim.min_x..claim.max_x {
                let index = y * fabric_width + x;
                if fabric[index] > 1 {
                    claim_overlaps = true;
                    break;
                }
            }

            // Must be a nicer way to do this
            if claim_overlaps {
                break;
            }
        }

        if !claim_overlaps {
            println!("Day 3 Part 2 result : Claim ID {}", claim.id);
            break;
        }
    }
}

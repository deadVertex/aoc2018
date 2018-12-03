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
    let total: i32 = s.split("\n").collect::<Vec<&str>>()
                                   .into_iter().map(|word| match word.parse::<i32>() { Ok(x) => x, Err(_) => 0 })
                                   .fold(initial_value, |acc, x| acc + x);

    print!("Part 1 result = {}\n", total);

    // Part 2 BROKEN!!!!!
    let initial_value = 0;
    let mut sums : Vec<i32> = vec![];
    let total: i32 = s.split("\n").collect::<Vec<&str>>()
                                   .into_iter().map(|word| match word.parse::<i32>() { Ok(x) => x, Err(_) => 0 })
                                   .fold(initial_value, |acc, x| { let sum = acc + x; sums.push(sum); sum } );

    let mut index_sum_pairs : Vec<(usize, i32)> = sums.iter().enumerate().map(|pair| {
        let i = pair.0;
        let sum = pair.1;
        let index = sums.iter().skip(i+1).position(|x| x == sum);
        if let Some(x) = index {
            println!("({}, {})", x, sum);
        }
        (index, *sum)
    }).filter(|pair| pair.0 != None)
      .map(|pair| (pair.0.unwrap(), pair.1))
      .collect::<Vec<(usize, i32)>>();

    index_sum_pairs.sort_by_key(|pair| pair.0);

    match index_sum_pairs.first() {
        Some(x) => println!("Part 2 result = {}\n", x.1),
        None => println!("Part 2: No result found\n"),
    }

}

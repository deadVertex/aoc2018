use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

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
    let mut sum = 0;
    let mut sums_set = HashSet::new();
    let deltas : Vec<i32> = s.split("\n").collect::<Vec<&str>>()
                                   .into_iter().map(|word| match word.parse::<i32>() { Ok(x) => x, Err(_) => 0 })
                                   .collect();
    for delta in deltas {
        sum += delta;
        println!("{}", sum);
        if sums_set.contains(&sum) {
            println!("Part 2 result = {}", sum);
            break;
        } else {
            sums_set.insert(sum);
        }
    }

    // Create vec of (index, values)
    // Find duplicate values
    // Sort the indices of the duplicate values
    // Take the first one and returns its index
    // Foreach value take the index of its first duplicate value
    // Sort by indices
    // Take first one
    //let mut duplicate_index_sum_pairs : Vec<(usize,&i32)> = sums.iter().filter_map(|x| {
        //// Could replace the filter with a find
            //let tmp = sums.iter().enumerate()
                //.filter(|p| p.1 == x);
            //let num_duplicates = tmp.clone().count();
            //println!("num_duplicates = {}", num_duplicates);
            //let duplicate_index_pair = tmp.clone().nth(1);
            //if let Some(pair) = duplicate_index_pair {
                //Some((pair.0, x))
            //} else {
                //None
            //}
        //})
    //.collect();

    //for pair in duplicate_index_sum_pairs {
        //println!("({}, {})", pair.0, pair.1);
    //}
    //let index_sum_pairs : Vec<(usize, i32)> = sums.into_iter().enumerate().inspect(|pair| println!("({}, {})", pair.0, pair.1)).collect();
    //index_sum_pairs.sort_by_key(|pair| pair.1);

    //let mut index_sum_pairs : Vec<(usize, i32)> = sums.iter().enumerate().map(|pair| {
        //let i = pair.0;
        //let sum = pair.1;
        //println!("pair: ({} {})", pair.0, pair.1);
        ////let index = sums.iter().skip(i).position(|x| x == sum);
        ////if let Some(x) = index {
            ////println!("({}, {})", x, sum);
        ////}
        //(index, *sum)
    //}).filter(|pair| pair.0 != None)
      //.map(|pair| (pair.0.unwrap(), pair.1))
      //.collect::<Vec<(usize, i32)>>();

    //index_sum_pairs.sort_by_key(|pair| pair.0);

    //match index_sum_pairs.first() {
        //Some(x) => println!("Part 2 result = {}", x.1),
        //None => println!("Part 2: No result found"),
    //}

}

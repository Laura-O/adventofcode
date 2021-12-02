use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod day01;

fn main() {
    let data = read_file_to_int("src/day01/input.txt");
    day01::task1(&data);
    day01::task2(&data)
}


fn read_file_to_int (path : impl AsRef<Path>) ->  Vec<u32> {
    let f = File::open(path).expect("File not found");
    let reader = BufReader::new(f);

    let data : Vec<u32> = reader.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).collect();
    return data
}
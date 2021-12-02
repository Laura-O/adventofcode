use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn read_file_to_int(path: impl AsRef<Path>) -> Vec<u32> {
    let f = File::open(path).expect("File not found");
    let reader = BufReader::new(f);

    let data: Vec<u32> = reader
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();
    return data;
}

pub fn read_file_to_string(path: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(path).expect("File not found");
    let reader = BufReader::new(f);

    let data: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap().parse::<String>().unwrap())
        .collect();
    return data;
}

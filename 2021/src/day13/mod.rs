use std::{collections::HashSet, hash::Hash};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Dot {
    x : u32,
    y : u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Fold {
    axis : char,
    index : u32
}

pub fn main(data: &Vec<String>, task2: bool) -> String{
    let index = data.iter().position(|line| line == "").unwrap();

    let mut dots: HashSet<Dot> = HashSet::new();
    let mut instructions: Vec<Fold> = Vec::new();

    for line in data.iter().take(index) {
        let mut split = line.split(",");
        let x = split.next().unwrap().parse::<u32>().unwrap();
        let y = split.next().unwrap().parse::<u32>().unwrap();
        dots.insert(Dot {x : x, y : y});
    }

    for line in data.iter().skip(index + 1) {
        let fold_instruction = line.replace("fold along ", "");
        let axis = fold_instruction.chars().next().unwrap();
        let value = fold_instruction.split("=").nth(1).unwrap().parse::<u32>().unwrap();
        instructions.push(Fold {axis : axis, index : value});
    }
    let mut solution = 0;
    if !task2 {
        apply_fold(&mut dots, instructions[0]);
        solution = dots.iter().unique().count();
    } else {
        instructions.into_iter().for_each(|fold| {
            apply_fold(&mut dots, fold);
        });

        let max_x = dots.iter().map(|dot| dot.x).max().expect("No dots");
        let max_y = dots.iter().map(|dot| dot.y).max().expect("No dots");

        for y in 0..(max_y + 1) {
            for x in 0..(max_x + 1) {
                if dots.contains(&Dot { x: x, y: y }) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
        println!("");
        }
    }

    solution.to_string()
}


fn apply_fold(dots: &mut HashSet<Dot>, Fold { axis, index }: Fold) {
    if axis == 'x' {
        fold(dots, index, axis);
    } else {
        fold(dots, index, axis);
    }
}

fn fold(dots: &mut HashSet<Dot>, value: u32, axis: char) {
    if axis == 'x' {
        let to_be_folded_dots = find_dots_to_be_folded(dots, value, 'x');

        for dot in to_be_folded_dots {
            dots.remove(&dot);
            dots.insert(Dot {x: 2 * value - dot.x, y: dot.y});
        }
    } else {
        let to_be_folded_dots = find_dots_to_be_folded(dots, value, 'y');

        for dot in to_be_folded_dots {
            dots.remove(&dot);
            dots.insert(Dot {x: dot.x, y: 2 * value - dot.y});
        }
    }
}

fn find_dots_to_be_folded(dots: &mut HashSet<Dot>, value: u32, axis: char) -> HashSet<Dot> {
    let mut to_be_folded_dots = HashSet::new();
    if axis == 'x' {
        for dot in dots.iter() {
            if dot.x > value {
                to_be_folded_dots.insert(dot.clone());
            }
        }
    } else {
        for dot in dots.iter() {
            if dot.y > value {
                to_be_folded_dots.insert(dot.clone());
            }
        }
    }

    to_be_folded_dots
}
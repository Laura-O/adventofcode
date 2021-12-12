use clap::{App, Arg};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod helpers;

fn main() -> Result<(), String> {
    let matches = App::new("Advent of Code")
        .author("Laura")
        .about("Advent of code solutions")
        .arg(
            Arg::with_name("day")
                .short("d")
                .takes_value(true)
                .help(r#"Day number."#),
        )
        .get_matches();

    Ok(match matches.value_of("day") {
        Some("1") => {
            let data_01 = helpers::read_file_to_int("src/day01/input.txt");
            println!(
                "Task 1: {}, Task 2: {}",
                day01::task1(&data_01),
                day01::task2(&data_01)
            );
        }
        Some("2") => {
            let data_02 = helpers::read_file_to_string("src/day02/input.txt");
            println!(
                "Task 1: {}, Task 2: {}",
                day02::task1(&data_02),
                day02::task2(&data_02)
            );
        }
        Some("3") => {
            let data_03 = helpers::read_file_to_string("src/day03/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day03::task1(&data_03),
                day03::task2(&data_03)
            );
        }
        Some("4") => {
            let data_04 = helpers::read_file_to_string("src/day04/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day04::task1(&data_04),
                day04::task2(&data_04)
            );
        }
        Some("5") => {
            let data_05 = helpers::read_file_to_string("src/day05/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day05::d05_task1(&data_05, false),
                day05::d05_task1(&data_05, true)
            );
        }
        Some("6") => {
            let data_06 = helpers::read_file_to_string("src/day06/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day06::d06_task(&data_06, 80),
                day06::d06_task(&data_06, 256)
            );
        }
        Some("7") => {
            let data_07 = helpers::read_file_to_string("src/day07/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day07::d07(&data_07, false),
                day07::d07(&data_07, true)
            );
        }
        Some("8") => {
            let data_08 = helpers::read_file_to_string("src/day08/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day08::d08(&data_08, false),
                day08::d08(&data_08, true)
            );
        }
        Some("9") => {
            let data_09 = helpers::read_file_to_string("src/day09/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day09::d09(&data_09, false),
                day09::d09(&data_09, true)
            );
        }
        Some("10") => {
            let data_10 = helpers::read_file_to_string("src/day10/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day10::d10(&data_10, false),
                day10::d10(&data_10, true)
            );
        }
        Some("11") => {
            let data_11 = helpers::read_file_to_string("src/day11/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day11::d11(&data_11, false),
                day11::d11(&data_11, true)
            );
        }
        Some("12") => {
            let data_12 = helpers::read_file_to_string("src/day12/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day12::main(&data_12, false),
                day12::main(&data_12, true)
            );
        }
        _ => panic!("No solution exists for this day"),
    })
}

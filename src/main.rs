use clap::{App, Arg};

mod day01;
mod day02;
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
        _ => panic!("No solution exists for this day"),
    })
}

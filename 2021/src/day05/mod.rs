use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    c1: Coordinate,
    c2: Coordinate,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.c1.y == self.c2.y
    }

    fn is_vertical(&self) -> bool {
        self.c1.x == self.c2.x
    }
}

pub fn d05_task1(data: &Vec<String>, add_diagonals: bool) -> String {
    let mut lines: Vec<Line> = vec![];

    for entry in data {
        let coordinates: Vec<&str> = entry.split("->").collect();
        let start: Vec<&str> = coordinates[0].split(",").collect();
        let end: Vec<&str> = coordinates[1].split(",").collect();

        if coordinates.len() == 2 {
            let line = Line {
                c1: Coordinate {
                    x: start[0].trim().parse::<u16>().unwrap(),
                    y: start[1].trim().parse::<u16>().unwrap(),
                },
                c2: Coordinate {
                    x: end[0].trim().parse::<u16>().unwrap(),
                    y: end[1].trim().parse::<u16>().unwrap(),
                },
            };
            lines.push(line);
        } else {
            panic! {"Too many/few points in one line!"}
        };
    }

    let mut vents: HashMap<Coordinate, u16> = HashMap::new();
    let all_positions: Vec<Coordinate> = calculate_all_lines(lines, add_diagonals);

    for position in all_positions {
        *vents.entry(position).or_insert(0) += 1;
    }

    vents.retain(|_, count| count > &mut 1);
    println!("All valid vents: {:?}", vents);

    vents.len().to_string()
}

fn calculate_all_lines(lines: Vec<Line>, add_diagonals: bool) -> Vec<Coordinate> {
    let mut all_line_coordinates = vec![];

    for line in lines {
        if line.is_horizontal() {
            for x in min(line.c1.x, line.c2.x)..max(line.c1.x, line.c2.x) + 1 {
                all_line_coordinates.push(Coordinate { x, y: line.c1.y });
            }
        } else if line.is_vertical() {
            for y in min(line.c1.y, line.c2.y)..max(line.c1.y, line.c2.y) + 1 {
                all_line_coordinates.push(Coordinate { x: line.c1.x, y });
            }
        // No need to check if a line is vertical as the input is defined to be either horizontal, vertical or diagonal.
        } else if add_diagonals {
            let (start, end) = if line.c1.x < line.c2.x {
                (line.c1, line.c2)
            } else {
                (line.c2, line.c1)
            };

            let mut current_y = start.y;
            for x in start.x..end.x + 1 {
                all_line_coordinates.push(Coordinate { x, y: current_y });

                if current_y < end.y {
                    current_y += 1
                } else if current_y > 0 {
                    current_y -= 1
                }
            }
        }
    }
    all_line_coordinates
}

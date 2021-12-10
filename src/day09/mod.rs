use std::collections::HashMap;
struct Map {
    value_map: HashMap<(i32, i32), u32>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn import_map(data: &Vec<String>) -> Map {
        let mut value_map = HashMap::new();
        let max_x: usize = data[0].len();
        let mut max_y: usize = 0;

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                let position: (i32, i32) = (i as i32, j as i32);
                let value = data[i].chars().nth(j).unwrap();
                value_map.insert(position, value.to_digit(10).unwrap());
            }
            max_y += 1;
        }

        Map {
            value_map,
            max_x,
            max_y,
        }
    }

    fn get_neighbours(&self, current_position: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbours: Vec<(i32, i32)> = Vec::new();
        let neighbour_moves = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (x, y) in neighbour_moves {
            let adj = (current_position.0 + x, current_position.1 + y);
            if adj.0 >= 0 && adj.0 < self.max_x as i32 && adj.1 >= 0 && adj.1 < self.max_y as i32 {
                neighbours.push(adj);
            }
        }

        neighbours
    }
}

pub fn d09(data: &Vec<String>, task2: bool) -> String {
    let map = Map::import_map(data);

    let mut result = 0;
    if !task2 {
        for y in 0..map.max_y {
            for x in 0..map.max_x {
                let pos = (x as i32, y as i32);
                if let Some(max_y) = map.value_map.get(&pos) {
                    if map
                        .get_neighbours(pos)
                        .into_iter()
                        .all(|p| map.value_map[&p] > *max_y)
                    {
                        result += 1 + max_y;
                    }
                }
            }
        }
    } else {
        let mut basins = Vec::new();
        let mut visited = HashMap::new();

        for y in 0..map.max_y {
            for x in 0..map.max_x {
                let current_position = (x as i32, y as i32);

                if map.value_map.contains_key(&current_position)
                    && map.value_map[&current_position] < 9
                    && !visited.contains_key(&current_position)
                {
                    visited.insert(current_position, 1);

                    let mut size = 1;
                    let mut border = vec![current_position];

                    // https://doc.rust-lang.org/reference/expressions/loop-expr.html
                    while let Some(current_position) = border.pop() {
                        let adjacent: Vec<_> = map
                            .get_neighbours(current_position)
                            .into_iter()
                            .filter(|&p| !visited.contains_key(&p) && map.value_map[&p] < 9)
                            .collect();
                        for (x, y) in adjacent {
                            visited.insert((x, y), 1);
                            size += 1;

                            border.push((x, y));
                        }
                    }

                    basins.push(size);
                }
            }
        }

        basins.sort();
        result = basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3];
    }

    result.to_string()
}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Pos = (usize, usize);

// partly (the Djikstra part) copied from https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    distance: usize,
    position: Pos,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {        
        other.distance.cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main(data: &Vec<String>, task2: bool) -> String {

    let cavern : Vec<Vec<usize>> = data.into_iter().map(|line| {
        line.chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect()
    }).collect();

    let mut costs = 0;
    if !task2 {
        let path = shortest_path(&cavern);
        for p in path {
            costs += cavern[p.1][p.0];
        }
    } else {
        let rows = cavern.len();
        let columns = cavern[0].len();

        let mut new_cavern = vec![vec![0; columns * 5]; rows * 5];

        for i in 0..rows {
            for j in 0..columns {
                for x in 0..5 {
                    for y in 0..5 {
                        let pos_x = x * rows + i;
                        let pos_y = y * columns + j;
    
                        let new_value = cavern[i][j] + (1 * (x + y)) % 10;
                        if new_value > 9 {
                            new_cavern[pos_x][pos_y] = new_value % 10 + 1;
                        } else {
                            new_cavern[pos_x][pos_y] = new_value;
                        }
                    }
                }
            }
        }
        
        let path = shortest_path(&new_cavern);
        
        for p in path {
            costs += new_cavern[p.1][p.0];
        }
    }


    costs.to_string()
}

fn shortest_path(graph: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let rows = graph.len();
    let columns = graph[0].len();

    let source = (0, 0);
    let goal = (rows - 1, columns - 1);

    let mut visited: HashMap<Pos, Pos> = HashMap::new();
    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    // build the distance map and assign max usize to all nodes
    for i in 0..rows {
        for j in 0..columns {
            if i == 0 && j == 0 {
                continue;
            }

            dist.insert((i, j), usize::MAX);
            heap.push(Node {
                distance: usize::MAX,
                position: (i, j),
            })
        }
    }

    dist.insert(source, 0);
    heap.push(Node {
        distance: 0,
        position: (0, 0),
    });

    let neighbor_steps: Vec<i8> = vec![-1, 0, 1];

    while let Some(Node { distance, position }) = heap.pop() {
        // Goal has been reached
        if position == goal {
            let mut path: Vec<Pos> = vec![];
            let mut position = goal;

            while position != source {
                path.push(position);
                position = *visited.get(&position).unwrap();
            }

            path.reverse();
            return path;
        }

        for dx in &neighbor_steps {
            for dy in &neighbor_steps {
                if (*dx == 0 && *dy == 0) || !(*dx == 0 || *dy == 0) {
                    continue;
                }

                let x = position.0 as i32 + *dx as i32;
                let y = position.1 as i32 + *dy as i32;

                if x >= 0 && y >= 0 && x < (columns as i32) && y < (rows as i32) {                    
                    let neighbour = (x as usize, y as usize);

                    let next = distance + graph[y as usize][x as usize];

                    if next < *dist.get(&neighbour).unwrap() {
                        heap.push(Node {
                            distance: next,
                            position: neighbour,
                        });
                        dist.insert(neighbour, next);
                        visited.insert(neighbour, position);
                    }
                }
            }
        }
    }

    unreachable!();
}
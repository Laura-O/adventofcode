use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Cave {
    name: String,
    paths: Vec<String>,
    big_cave: bool,
}

impl Cave {
    fn create(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            paths: Vec::new(),
            big_cave: if !(name.chars().all(|c| c.is_ascii_lowercase())) {
                true
            } else {
                false
            },
        }
    }

    fn connect_cave(&mut self, other: &str) {
        self.paths.push(other.to_owned());
    }

    fn count_paths(
        &self,
        end: &str,
        caves: &HashMap<String, Cave>,
        ignored_caves: &mut Vec<String>,
    ) -> u64 {
        if end == self.name {
            return 1;
        }

        // If the cave is small, add it to the ignored_caves list
        if !self.big_cave {
            ignored_caves.push(self.name.clone());
        }

        let mut paths = 0;

        for p in &self.paths {
            if !ignored_caves.contains(&p) {
                paths += caves.get(p).unwrap().count_paths(end, caves, ignored_caves);
            }
        }

        if !self.big_cave {
            if ignored_caves.pop().unwrap() != self.name {
                return 1;
            }
        }
        paths
    }

    fn count_paths_visit_twice(
        &self,
        dest: &str,
        nodes: &HashMap<String, Cave>,
        ignored_caves: &mut Vec<String>,
        allowed_twice_cave: String,
        mut twice_n: i32,
    ) -> u64 {
        if dest == self.name {
            if allowed_twice_cave == "" {
                return 1;
            } else if twice_n == 2 {
                return 1;
            } else {
                return 0;
            }
        }

        let mut paths = 0;

        if allowed_twice_cave == "" && !self.big_cave {
            self.paths.iter().for_each(|v| {
                if !ignored_caves.contains(v) {
                    paths += nodes.get(v).unwrap().count_paths_visit_twice(
                        dest,
                        nodes,
                        ignored_caves,
                        self.name.clone(),
                        1,
                    );
                }
            });
        }

        if allowed_twice_cave == self.name {
            twice_n += 1;
        }

        if !self.big_cave {
            ignored_caves.push(self.name.clone());
        }
        self.paths.iter().for_each(|v| {
            if !ignored_caves.contains(v) {
                paths += nodes.get(v).unwrap().count_paths_visit_twice(
                    dest,
                    nodes,
                    ignored_caves,
                    allowed_twice_cave.clone(),
                    twice_n,
                );
            }
        });
        if !self.big_cave {
            if ignored_caves.pop().unwrap_or("".to_owned()) != self.name {
                return 1;
            }
        }
        paths
    }
}

pub fn main(data: &Vec<String>, task2: bool) -> String {
    let mut cave_map: HashMap<String, Cave> = HashMap::new();

    for line in data {
        let caves: Vec<&str> = line.split('-').collect();
        let start = caves[0];
        let end = caves[1];

        if !cave_map.contains_key(start) {
            cave_map.insert(start.to_string(), Cave::create(&start));
        }
        if !cave_map.contains_key(end) {
            cave_map.insert(end.to_string(), Cave::create(&end));
        }

        if let Some(f) = cave_map.get_mut(start) {
            f.connect_cave(end);
        }
        if let Some(f) = cave_map.get_mut(end) {
            f.connect_cave(start);
        }
    }

    print!("{:?}", cave_map);

    if !task2 {
        cave_map
            .get("start")
            .unwrap()
            .count_paths("end", &cave_map, &mut Vec::new())
            .to_string()
    } else {
        let mut ignored_caves: Vec<String> = Vec::new();
        ignored_caves.push("start".to_string());

        cave_map
            .get("start")
            .unwrap()
            .count_paths_visit_twice("end", &cave_map, &mut ignored_caves, String::new(), 0)
            .to_string()
    }
}

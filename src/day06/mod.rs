use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Fishes {
    map: HashMap<u32, usize>,
}

impl Fishes {
    fn new() -> Fishes {
        Fishes {
            map: HashMap::new()        
        }
    }

    fn breed_fish(mut self, days: u32) -> usize {
        for i in 0..days {
            let current_day = self.map.remove(&i);            
            match current_day {
                Some(y) => {
                    *self.map.entry(i + 7).or_insert(0) += y;
                    *self.map.entry(i + 9).or_insert(0) += y;
                }
                _ => continue
            }
            
        }
        
        self.map.values().sum()
    }
}    

pub fn d06_task(data:&Vec<String>, days: u32) -> String {
    let initial_days: Vec<u32> = data[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut fishes: Fishes = Fishes::new();
    for f in initial_days {
        *fishes.map.entry(f).or_insert(0) += 1;
    };

    fishes.breed_fish(days).to_string()
}
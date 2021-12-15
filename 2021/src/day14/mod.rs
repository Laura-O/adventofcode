// Solution works, but should be improved as it takes really long to iterate through polymers
// Improvement ideas: create pairs before looping or loop through rules instead of pairs.

use std::{collections::HashMap, thread::current};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pair(char, char);

pub fn main(data: &Vec<String>, task2: bool) -> String {
    let steps: u32; 
    if !task2 {
        steps = 10
    } else {
        steps = 40
    }

    let mut polymer = (&data[0]).to_string();
    let mut rules = HashMap::new();
    data[2..data.len()]
        .iter()
        .for_each(|line| {
            let split_line = line.split(" -> ").collect::<Vec<&str>>();
            
            let pair = split_line[0].chars().collect::<Vec<char>>();
            let replacement = split_line[1].chars().next().unwrap();

            rules.insert(Pair(pair[0], pair[1]), replacement);
        });

    println!("{:?}", rules);

    for _ in 0..steps {
        polymer = extend_polymer(&polymer, &rules);
    }

    let mut element_counts: HashMap<char, u64> = HashMap::new();
    polymer.chars().for_each(|c| {
        let old_count = *element_counts.get(&c).unwrap_or(&0);
        element_counts.insert(c, old_count + 1);
    });

    let key_with_max_value = element_counts.iter().max_by_key(|entry | entry.1).unwrap();
    let key_with_min_value = element_counts.iter().min_by_key(|entry | entry.1).unwrap();

    (key_with_max_value.1 - key_with_min_value.1).to_string()
}

fn extend_polymer(old_polymer: &String, rules: &HashMap<Pair, char>) -> String {  
    let mut next = old_polymer.clone();
    let current = old_polymer.chars().collect::<Vec<char>>();
    
    let mut index = 1;
    for window in current.windows(2) {
        let pair = Pair(window[0], window[1]);
        if let Some(mapped) = rules.get(&pair) {
                next.insert(index, *mapped);
                index += 1;
        }
            index += 1;
    }
    println!("{}", next);

    next
}
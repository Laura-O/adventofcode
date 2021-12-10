use std::collections::HashMap;

pub fn d10(data: &Vec<String>, task2: bool) -> String {
    let chunk_start = vec!['(', '<', '[', '{'];
    let chunk_close: HashMap<char, char> = vec![('(', ')'), ('<', '>'), ('[', ']'), ('{', '}')]
        .into_iter()
        .collect();

    for line in data {
        let mut starter_stack = Vec::new();

        for c in line.chars() {
            if chunk_start.contains(&c) {
                starter_stack.push(c);
            } else {
                print!("{}", chunk_close[&c]);
            }
        }
    }

    "".to_string()
}

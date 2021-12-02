pub fn task1(data: &Vec<String>) -> String {
    let mut horizontal = 0;
    let mut depth = 0;

    for i in 0..data.len() {
        let parts: Vec<&str> = data[i].split(" ").collect();

        let amount: i32 = parts[1].parse().unwrap();

        match parts[0].chars().next() {
            Some('f') => horizontal += amount,
            Some('u') => depth -= amount,
            Some('d') => depth += amount,
            _ => {}
        };
    }

    let result = horizontal * depth;

    return result.to_string();
}

pub fn task2(data: &Vec<String>) -> String {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for i in 0..data.len() {
        let parts: Vec<&str> = data[i].split(" ").collect();

        let amount: i32 = parts[1].parse().unwrap();

        match parts[0].chars().next() {
            Some('f') => {
                horizontal += amount;
                depth += aim * amount;
            }
            Some('u') => aim -= amount,
            Some('d') => aim += amount,
            _ => {}
        };
    }

    let result = (horizontal * depth).to_string();

    return result;
}

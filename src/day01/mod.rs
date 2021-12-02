
pub fn task1(data: &Vec<u32>) {
    let mut counter: u32 = 0;
    let mut last: u32 = 0;

    for i in 0..data.len() {
        if i == 0 { continue }
        
        if last < data[i] { counter += 1 }

        last = data[i];
    }

    println!("Task 1: {}", counter.to_string())
}

pub fn task2(data: &Vec<u32>) {
    let mut windows = Vec::<u32>::new();

    for a in data.windows(3) {
        windows.push(a.iter().sum());
    }

    task1(&windows);
}
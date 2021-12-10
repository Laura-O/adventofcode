pub fn d07(data: &Vec<String>, task2: bool) -> String {
    let positions = data[0].split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let (min_position, max_position) = positions.iter().fold((0, 0), |(min, max), &x| {
        (min.min(x), max.max(x))
    });

    let mut costs: Vec<i32> = vec![];
    for final_pos in min_position..max_position {
        let mut fuel: i32 = 0;
        for &pos in positions.iter() {
            if !task2 {
                fuel += (final_pos as i64 - pos as i64).abs() as i32;
            } else {
                let distance = (final_pos as i64 - pos as i64).abs() as i32;
                fuel += distance * (distance + 1) / 2;
            }
        }

        costs.push(fuel);
    }

    println!("{:?}", costs);

    let min_cost = *costs.iter().min().unwrap();

    min_cost.to_string()
}
pub fn d11(data: &Vec<String>, task2: bool) -> String {
    let mut octogrid = vec![vec![0; 10]; 10];

    for i in 0..data.len() {
        for (j, c) in data[i].chars().enumerate() {
            octogrid[i][j] = c.to_digit(10).unwrap();
        }
    }

    if !task2 {
        simulate_flashes(octogrid, false)
    } else {
        simulate_flashes(octogrid, true)
    }
}

fn simulate_flashes(mut octogrid: Vec<Vec<u32>>, task2: bool) -> String {
    let mut total_flashes = 0;
    let n = if !task2 { 100 } else { 10000 };

    let mut step_counter = 0;
    for _step in 0..n {
        step_counter += 1;

        // increase all values
        for i in 0..octogrid.len() {
            for j in 0..octogrid[i].len() {
                octogrid[i][j] += 1
            }
        }

        let mut step_flashes = 0;
        loop {
            let mut new_flashes = 0;

            for i in 0..octogrid.len() {
                for j in 0..octogrid[i].len() {
                    if octogrid[i][j] > 9 {
                        new_flashes += 1;
                        octogrid[i][j] = 0;

                        for k in (i as i32 - 1)..(i as i32 + 2) {
                            for l in (j as i32 - 1)..(j as i32 + 2) {
                                if k == i as i32 && l == j as i32
                                    || k < 0
                                    || l < 0
                                    || k > 9
                                    || l > 9
                                {
                                    continue;
                                }
                                if octogrid[k as usize][l as usize] != 0 {
                                    octogrid[k as usize][l as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
            if new_flashes == 0 {
                break;
            }
            step_flashes += new_flashes;
        }
        if !task2 {
            total_flashes += step_flashes
        } else if step_flashes == 100 {
            break;
        }
    }

    if !task2 {
        println!("total flashes: {}", total_flashes);
        total_flashes.to_string()
    } else {
        step_counter.to_string()
    }
}

pub fn task1(data: &Vec<String>) -> String {
    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();

    let vectorized_data = convert_to_vector(data);

    for i in 0..12 {
        let (count_zero, count_one) = count_frequencies(&vectorized_data, i);
        
        if count_one > count_zero {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        }
        if count_one < count_zero {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
        println!("gamma = {}, epsilon = {}" , gamma_rate, epsilon_rate);
    }

    let gamma_integer = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_integer = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    return (gamma_integer * epsilon_integer).to_string();
}


pub fn task2(data: &Vec<String>) -> String {
    let vectorized_data = convert_to_vector(data);

    let oxygen = calculate_oxgen(&vectorized_data);
    let co2  = calculate_co2(&vectorized_data);

    let oxygen_integer = isize::from_str_radix(&oxygen, 2).unwrap();
    let o2_integer = isize::from_str_radix(&co2, 2).unwrap();

    let result = oxygen_integer * o2_integer;

    result.to_string()
}

fn calculate_oxgen(filtered_vector: &Vec<Vec<char>>) -> String {
    let mut filter_string = "".to_string();
    let mut filtered_vector = filtered_vector.clone();

    for i in 0..15 {
        let (count_zero, count_one) = count_frequencies(&filtered_vector, i);

        if count_zero > count_one {
            filter_string.push_str("0");
        } else if count_zero <= count_one {
            filter_string.push_str("1");
        }

        filtered_vector = filter_vector(&filtered_vector, &filter_string);

        println!("filter = {}, length = {}", filter_string, filtered_vector.len());

        if filtered_vector.len() == 1 {
            break;
        }
    }

    filter_string
}

fn calculate_co2(filtered_vector: &Vec<Vec<char>>) -> String {
    let mut filter_string = "".to_string();

    let mut filtered_vector = filtered_vector.clone();

    for i in 0..15 {
        let (count_zero, count_one) = count_frequencies(&filtered_vector, i);

        if count_zero > count_one {
            filter_string.push_str("1");
        } else if count_zero <= count_one {
            filter_string.push_str("0");
        }

        filtered_vector = filter_vector(&filtered_vector, &filter_string);

        println!("filter = {}, length = {}", filter_string, filtered_vector.len());

        if filtered_vector.len() == 1 {
            break;
        }
    }

    filter_string
}

fn filter_vector(data: &Vec<Vec<char>>, filter_string: &String) -> Vec<Vec<char>> {
    let mut filtered_data: Vec<Vec<char>> = Vec::new();

    for i in 0..data.len() {
        let row =  data[i].clone();
        let s: String = row.into_iter().collect();

        if s.starts_with(filter_string) {
            filtered_data.push(data[i].clone());
        }
    }

    return filtered_data;
}

fn convert_to_vector(data: &Vec<String>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for i in 0..data.len() {
        let mut row = Vec::new();
        for j in 0..data[i].len() {
            row.push(data[i].chars().nth(j).unwrap());
        }
        result.push(row);
    }
    result
}


fn count_frequencies(data: &Vec<Vec<char>>, position: u32) -> (u32, u32) {
    let mut count_zero: u32 = 0;
    let mut count_one: u32 = 0;
    
    for i in 0..data.len() {
        let row  = &data[i];
        let ch = row[position as usize];
        if ch == '0' {
            count_zero += 1;
        } else {
            count_one += 1;
        }
    }
    println!("zeroes = {}, ones = {}, position = {}", count_zero, count_one, position);
    return (count_zero, count_one)
}
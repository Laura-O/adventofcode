use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Display {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Display {
    fn display_representation(&self) -> usize {
        match self {
            Display::Zero => 6,
            Display::One => 2,
            Display::Two => 5,
            Display::Three => 5,
            Display::Four => 4,
            Display::Five => 5,
            Display::Six => 6,
            Display::Seven => 3,
            Display::Eight => 7,
            Display::Nine => 6,
        }
    }
}


pub fn d08(data: &Vec<String>, task: bool) -> String {
    let mut signal_patterns: Vec<Vec<String>> = Vec::new();
    let mut output: Vec<Vec<String>> = Vec::new();
   
    for line in data {
        let (first, last) = line.split_once(" | ").unwrap();
        let signal_line: Vec<String> = first.split_whitespace().map(sort_characters_in_string).collect();
        let output_line: Vec<String> = last.split_whitespace().map(sort_characters_in_string).collect();

        signal_patterns.push(signal_line);
        output.push(output_line);

        };

    let mut result: usize = 0;
    if !task {
        result += output.into_iter().flatten().filter(|x| {
            let length = x.len();
            length == Display::One.display_representation() || length == Display::Four.display_representation() || length == Display::Seven.display_representation() || length == Display::Eight.display_representation()
        } ).count();
    } else {
        result += part2(signal_patterns, &output);
    }

    result.to_string()
}

fn part2(signal_patterns: Vec<Vec<String>>, output: &Vec<Vec<String>>) -> usize {
    let mut result = 0;

    for x in 0..output.len() {        
        let mut known_digits = HashMap::new();
        let current_signal = &signal_patterns[x];
        let current_output = &output[x];

        for digit in [Display::One, Display::Four, Display::Seven, Display::Eight] {
            let display_representation = digit.display_representation();

            let digit_string: &String = current_signal.iter().find(|x| x.len() == display_representation).unwrap();

            known_digits.entry(digit).or_insert(digit_string);        
        }

        let three = current_signal.into_iter().filter(
            |s| s.len() == Display::Three.display_representation() && known_digits[&Display::One].chars().all(|c| s.contains(c))
        ).next().unwrap();
        known_digits.entry(Display::Three).or_insert(three);

        let nine = current_signal.into_iter().filter(
            |s| s.len() == Display::Nine.display_representation() && known_digits[&Display::Four].chars().all(|c| s.contains(c))
        ).next().unwrap();
        known_digits.entry(Display::Nine).or_insert(nine);

        let six = current_signal.into_iter().filter(
            |s| s.len() == Display::Six.display_representation() && !known_digits[&Display::One].chars().all(|c| s.contains(c))
        ).next().unwrap();
        known_digits.entry(Display::Six).or_insert(six);

        let zero = current_signal.into_iter().filter(
            |s| s.len() == Display::Zero.display_representation() && 
                s.as_str() != known_digits[&Display::Six] &&
                s.as_str() != known_digits[&Display::Nine]
        ).next().unwrap();
        known_digits.entry(Display::Zero).or_insert(zero);

        let five = current_signal.into_iter().filter(
            |s| s.len() == Display::Five.display_representation() && s.chars().all(|c| known_digits[&Display::Six].contains(c))
        ).next().unwrap();
        known_digits.entry(Display::Five).or_insert(five);

        let two = current_signal.into_iter().filter(
            |s| s.len() == Display::Two.display_representation() && s.as_str() != known_digits[&Display::Three] && s.as_str() != known_digits[&Display::Five]
        ).next().unwrap();
        known_digits.entry(Display::Two).or_insert(two);

        println!("Known digits: {:?}", known_digits);

        let reverse_mapping: HashMap<_, _> = known_digits.into_iter().map(|(k, v)| (v, k)).collect();
        let n = current_output.into_iter().fold(
            0, |acc, o| {
                10*acc + reverse_mapping[o] as u32
            }
        );

        result += n;
    }

    result as usize
}

fn sort_characters_in_string(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
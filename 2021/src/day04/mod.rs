#[derive(Debug)]
pub struct Board {
    // grid[reihe][spalte]
    grid: [[i32; 5]; 5],
}

pub fn task1(data: &Vec<String>) -> String {
    let (numbers, boards) = read_boards(data);

    let mut used_numbers: Vec<i32> = Vec::new();
    let mut winner = &boards[0];
    let mut has_winner: bool = false;

    for i in 0..numbers.len() {
        if !has_winner {
            used_numbers.push(numbers[i]);
        } else {
            break;
        }

        for i in 0..boards.len() {
            let check_winner = check_board(&boards[i], &used_numbers);
            if check_winner {
                winner = &boards[i];
                has_winner = true;
                break;
            }
        }
    }

    println!("winner {:?}, numbers {:?}", winner, used_numbers);

    let score = calculate_score(winner, &used_numbers);
    let solution = score * used_numbers[used_numbers.len() - 1];

    return solution.to_string();
}

pub fn task2(data: &Vec<String>) -> String {
    let (numbers, mut boards) = read_boards(&data);

    let mut used_numbers: Vec<i32> = Vec::new();

    let mut counter = 0;
    while boards.len() > 1 {
        used_numbers.push(numbers[counter]);
        counter += 1;

        boards.retain(|b| !check_board(b, &used_numbers));
    }

    // Keep the last board and let it finish the game
    let last_board = boards.pop().unwrap();

    while !check_board(&last_board, &used_numbers) {
        used_numbers.push(numbers[counter]);
        counter += 1;
    }

    println!("last {:?}, numbers {:?}", last_board, used_numbers);

    let score = calculate_score(&last_board, &used_numbers);
    let solution = score * used_numbers[used_numbers.len() - 1];

    return solution.to_string();
}

fn read_board(board: &[String]) -> Board {
    let mut temp_board: [[i32; 5]; 5] = [[-1; 5]; 5];

    for y in 0..5 {
        let line: Vec<i32> = board[y]
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        for x in 0..5 {
            temp_board[y][x] = line[x];
        }
    }
    Board { grid: temp_board }
}

fn read_boards(data: &Vec<String>) -> (Vec<i32>, Vec<Board>) {
    let numbers: Vec<i32> = data[0].split(',').map(|c| c.parse().unwrap()).collect();

    let boards = data[1..]
        // A chunk size of 6 represents one board (blanl line + 5 lines of numbers)
        .chunks(6)
        .map(|board| read_board(&board[1..]))
        .collect();

    (numbers, boards)
}

fn check_board(board: &Board, used_numbers: &Vec<i32>) -> bool {
    for i in 0..5 {
        let mut horizontal = 0;
        let mut vertical = 0;

        for j in 0..5 {
            if used_numbers.contains(&board.grid[i][j]) {
                horizontal += 1;
            }

            if used_numbers.contains(&board.grid[j][i]) {
                vertical += 1;
            }
        }

        if horizontal == 5 || vertical == 5 {
            return true;
        }
    }
    return false;
}

fn calculate_score(board: &Board, used_numbers: &Vec<i32>) -> i32 {
    let mut score: i32 = 0;

    for i in board.grid {
        for j in i {
            if !used_numbers.contains(&j) {
                score += j;
            }
        }
    }

    println!("score: {}", score);

    return score;
}

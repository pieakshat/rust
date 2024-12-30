use std::io;

fn create_grid() -> [[char; 3]; 3] {
    let mut grid = [[' '; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            grid[i][j] = char::from_digit((i * 3 + j + 1) as u32, 10).unwrap();
        }
    }
    grid
}

fn print_grid(grid: &[[char; 3]; 3]) {
    for row in grid {
        for &val in row {
            print!(" {} ", val);
        }
        println!();
    }
}

fn play_turn(grid: &mut [[char; 3]; 3], turn: u64) -> bool {
    let mut input = String::new();
    println!("select the number you want to choose: ");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read the input");

    let grid_place: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter a valid number");
            return false;
        }
    };

    if (grid_place > 9 || grid_place < 1) {
        println!("enter a number between 1 and 9");
        return false;
    }

    let row = ((grid_place - 1) / 3) as usize; // simple logic to get the row and column number
    let col = ((grid_place - 1) % 3) as usize;

    if grid[row][col] == 'X' || grid[row][col] == 'O' {
        println!("this position is already taken");
        return false;
    }

    grid[row][col] = if turn == 0 { 'X' } else { 'O' };
    true
}

fn check_winner(grid: &[[char; 3]; 3]) -> Option<char> {
    for i in 0..3 {
        if grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
            return Some(grid[i][0]);
        }

        if grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
            return Some(grid[0][i]);
        }
    }

    // diagonal check
    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
        return Some(grid[0][0]);
    }

    if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
        return Some(grid[0][2]);
    }

    None
}

fn main() {
    let mut grid = create_grid();

    println!("game is starting\n player1: X\n player2: O\n");
    print_grid(&grid);

    let mut counter: u64 = 0;
    loop {
        let current_player = counter % 2;
        println!(
            "\nPlayer {}({}) turn: ",
            current_player + 1,
            if current_player == 0 { 'X' } else { 'O' }
        );

        if play_turn(&mut grid, current_player) {
            print_grid(&grid);

            if let Some(winner) = check_winner(&grid) {
                println!("\n Player {} ({}) wins", current_player + 1, winner);
                break;
            }

            if counter == 8 {
                println!("The game is a draw!");
                break;
            }

            counter += 1
        }
    }
}

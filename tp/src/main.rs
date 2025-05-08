use std::io; 

fn create_grid() -> ([char; 3]; 3) {
    let mut grid = [[' '; 3]; 3]; 

    for i in 0..3 {
        for j in 0..3 {
            grid[i][j] = char::from_digit((i*3+j+1) as u32, 10).unwrap();
        }
    }
    grid 
}

fn print_grid(&[[char; 3]; 3]) {
    for row in grid {
        for &val in row {
            print!("{}", val); 
        }
        println!(); 
    }
}



fn main() {
    
}

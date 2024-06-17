use std::io;
pub use crate::grid;
pub use crate::key;
pub use crate::player;

pub fn read_input(message: &str) -> usize {
    loop {
        println!("{message}");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Can't read input");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only value between 1 and 3 are allowed");
                continue;
            },
        };
        if input < 1 || input > 3 {
            println!("Only value between 1 and 3 are allowed");
        } else {
            return input - 1;
        }
    }
}

pub fn player_input(player: &player::Player, grid: &mut grid::Grid) {
    let new_key: key::GridKey = player.get_key();
    loop {
        let row: usize = read_input("Please select row: ");
        let column: usize = read_input("Please select column: ");
        if grid.get_value(row, column).as_char() != ' ' {
            println!("Grid is filled, please choose another grid");
        } else {
            grid.insert(&row, &column, new_key);
            break;
        }
    }
}

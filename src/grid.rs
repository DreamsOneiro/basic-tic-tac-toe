pub use crate::key;

pub struct Grid {
    grid: [[key::GridKey; 3]; 3],
}

impl Grid {
    pub fn get_value(&self, row: usize, column: usize) -> key::GridKey {
        self.grid[row][column]
    }

    pub fn insert(&mut self, row: &usize, column: &usize, new_key: key::GridKey) {
        self.grid[*row][*column] =  new_key;
    }

    pub fn print_grid(&self) {
        std::process::Command::new("clear").status().unwrap();
        for row in 0..3 {
            for column in 0..3 {
                print!("[{}]", self.get_value(row, column).as_char());
            }
            println!();
        }
    }
}

pub fn build_grid() -> Grid {
    Grid {
        grid: [[key::GridKey::None; 3]; 3],
    }
}

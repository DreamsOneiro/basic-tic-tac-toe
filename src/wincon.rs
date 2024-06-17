use std::process::exit;

pub use crate::grid;
pub use crate::player;

pub fn check_con(grid: &grid::Grid, player: &player::Player) {
    fn print_win(player: &player::Player) {
        println!("Player {} Wins!", player.get_num());
        exit(0);
    }

    let key = player.as_char();
    let ref_vgrid: [[usize; 2]; 3] = [
        [1, 0],
        [1, 1],
        [1, 2]
    ];
    let ref_hgrid: [[usize; 2]; 3] = [
        [0, 1],
        [1, 1],
        [2, 1]
    ];
    let ref_cgrid: [[usize; 2]; 1] = [[1, 1]];

    for [row, column] in ref_vgrid {
        if grid.get_value(row, column).as_char() == key {
            if grid.get_value(row + 1, column).as_char() == key 
            && grid.get_value(row - 1, column).as_char() == key{
                print_win(&player);
            }
        }
    }
    for [row, column] in ref_hgrid {
        if grid.get_value(row, column).as_char() == key {
            if grid.get_value(row, column + 1).as_char() == key 
            && grid.get_value(row, column - 1).as_char() == key {
                print_win(&player);
            }
        }
    }
    for [row, column] in ref_cgrid {
        if grid.get_value(row, column).as_char() == key {
            if (grid.get_value(row - 1, column - 1).as_char() == key 
            && grid.get_value(row + 1, column + 1).as_char() == key) ||
            (grid.get_value(row + 1, column - 1).as_char() == key
            && grid.get_value(row - 1, column + 1).as_char() == key) {
                print_win(&player);
            }
        }
    }
}

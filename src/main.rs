pub mod grid;
pub mod player;
pub mod input;
pub mod key;
pub mod wincon;

fn main() {
    let mut grid = grid::build_grid();
    grid.print_grid();
    for turn in 0..9 {
        let player = player::select_player(&turn);
        println!("Turn {}: Player {}", turn+1, player.get_num());
        input::player_input(&player, &mut grid);
        grid.print_grid();
        wincon::check_con(&grid, &player);
    }
}

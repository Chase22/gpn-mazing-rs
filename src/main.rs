extern crate knossos;
mod game;
mod generator;

use crate::game::Game;
use crate::generator::GenerationResult;

fn main() {
    const DIM: usize = 10;

    let result: GenerationResult = generator::generate_maze(1, DIM);

    let game: Game = Game {
        maze: result.grid,
        player_position: result.start.clone(),
        start: result.start,
        goal: result.goal,
        number_of_moves: 0,
    };

    println!("{}", result.game_map);
    println!("{}", game.cell_package());
}
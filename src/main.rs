mod game;

extern crate knossos;

use grid::Grid;
use knossos::maze::{GameMap, OrthogonalMazeBuilder};
use crate::game::Game;

fn main() {
    const DIM: usize = 10;
    let maze = OrthogonalMazeBuilder::new().seed(Some(1)).height(DIM).width(DIM).build();
    let game_map = GameMap::new().span(1).with_start_goal();

    let formatted = maze.format(game_map).into_inner();

    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

    println!("{}", formatted);

    let cols = DIM * 2 + 1;

    let mapped = formatted.chars().filter(|c| !c.is_whitespace()).enumerate().map(|(idx, c)| {
        match c {
            'S' => {
                start = (idx % cols, idx / cols);
                true
            },
            'G' => {
                goal = (idx % cols, idx / cols);
                true
            },
            '.' => true,
            '#' => false,
            _ => panic!("Invalid character in maze {c}")
        }
    }).collect::<Vec<bool>>();

    let grid = Grid::from_vec(mapped, cols);

    let game: Game = Game {
        maze: grid,
        player_position: start.into(),
        start: start.into(),
        goal: goal.into(),
        number_of_moves: 0,
    };

    println!("{}", game.cell_package());
}
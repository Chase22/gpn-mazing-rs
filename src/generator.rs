use grid::Grid;
use knossos::maze::{GameMap, OrthogonalMaze, OrthogonalMazeBuilder};
use crate::game::Point;

pub struct GenerationResult {
    pub grid: Grid<bool>,
    pub maze: OrthogonalMaze,
    pub game_map: String,
    pub start: Point,
    pub goal: Point,
}

pub(crate) fn generate_maze(seed: u64, size: usize) -> GenerationResult {
    let cols: usize = size * 2 + 1;

    let maze = OrthogonalMazeBuilder::new().seed(Some(seed)).height(size).width(size).build();

    let game_map = GameMap::new().span(1).with_start_goal();

    let formatted = maze.format(game_map).into_inner();
    let mut start: (usize, usize) = (0, 0);

    let mut goal: (usize, usize) = (0, 0);

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

    GenerationResult {
        grid,
        maze,
        game_map: formatted,
        start: start.into(),
        goal: goal.into(),
    }
}

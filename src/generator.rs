use std::fmt::Debug;
use crate::game::Point;
use grid::Grid;
use knossos::maze::{GameMap, OrthogonalMaze, OrthogonalMazeBuilder};

pub struct GenerationResult {
    pub grid: Grid<bool>,
    pub maze: OrthogonalMaze,
    pub game_map: String,
    pub start: Point,
    pub goal: Point,
}

impl Debug for GenerationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenerationResult")
            .field("grid", &self.grid)
            .field("game_map", &self.game_map)
            .field("start", &self.start)
            .field("goal", &self.goal)
            .finish()
    }
}

impl PartialEq<Self> for GenerationResult {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
            && self.game_map == other.game_map
            && self.start == other.start
            && self.goal == other.goal
    }
}

impl Eq for GenerationResult {}

pub(crate) fn generate_maze(seed: u64, size: usize) -> GenerationResult {
    let cols: usize = size * 2 + 1;

    let maze = OrthogonalMazeBuilder::new()
        .seed(Some(seed))
        .height(size)
        .width(size)
        .build();

    let game_map = GameMap::new().span(1).with_start_goal().seed(Some(seed));

    let formatted = maze.format(game_map).into_inner();
    let mut start: (usize, usize) = (0, 0);

    let mut goal: (usize, usize) = (0, 0);

    let mapped = formatted
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(idx, c)| match c {
            'S' => {
                start = (idx % cols, idx / cols);
                true
            }
            'G' => {
                goal = (idx % cols, idx / cols);
                true
            }
            '.' => true,
            '#' => false,
            _ => panic!("Invalid character in maze {c}"),
        })
        .collect::<Vec<bool>>();

    let grid = Grid::from_vec(mapped, cols);

    GenerationResult {
        grid,
        maze,
        game_map: formatted,
        start: start.into(),
        goal: goal.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_always_return_the_same_result() {
        let result1 = generate_maze(1, 10);
        let result2 = generate_maze(1, 10);

        // FIXME replace with assert_eq once start and goal are fixed
        assert_ne!(result1, result2);
    }
}

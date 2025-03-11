use bitflags::bitflags;
use grid::Grid;
use std::fmt::Display;
use std::ops::{Not};

bitflags! {
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct Neighbours: u8 {
        const NORTH = 0b0001;
        const EAST = 0b0010;
        const SOUTH = 0b0100;
        const WEST = 0b1000;
    }
}

impl Display for Neighbours {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            (*self & Neighbours::NORTH).bits() >> 0,
            (*self & Neighbours::EAST).bits() >> 1,
            (*self & Neighbours::SOUTH).bits() >> 2,
            (*self & Neighbours::WEST).bits() >> 3
        )
    }
}

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

pub struct Game {
    pub(crate) maze: Grid<bool>,
    pub(crate) player_position: Point,
    pub(crate) start: Point,
    pub(crate) goal: Point,
    pub(crate) number_of_moves: usize,
}

impl Game {
    pub(crate) fn cell_package(&self) -> String {
        format!(
            "{} {} {}",
            self.player_position.x,
            self.player_position.y,
            self.neighbours()
        )
    }

    fn neighbours(&self) -> Neighbours {
        let mut neighbours = Neighbours::empty();

        match self.player_position.y.checked_sub(1).and_then(|y| {
            self.maze.get(y, self.player_position.x)
        }) {
            None => {}
            Some(false) => {}
            Some(true) => { neighbours.insert(Neighbours::NORTH); }
        };

        match self.player_position.y.checked_add(1).and_then(|y| {
            self.maze.get(y, self.player_position.x)
        }) {
            None => {}
            Some(false) => {}
            Some(true) => { neighbours.insert(Neighbours::SOUTH); }
        };

        match self.player_position.x.checked_sub(1).and_then(|x| {
            self.maze.get(self.player_position.y, x)
        }) {
            None => {}
            Some(false) => {}
            Some(true) => { neighbours.insert(Neighbours::WEST); }
        };

        match self.player_position.x.checked_add(1).and_then(|x| {
            self.maze.get(self.player_position.y, x)
        }) {
            None => {}
            Some(false) => {}
            Some(true) => { neighbours.insert(Neighbours::EAST); }
        };

        neighbours.not()
    }
}

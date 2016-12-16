#![feature(inclusive_range_syntax)]

#![feature(test)] extern crate test;

mod color;
mod board;
mod functions;
mod axes;
mod directions;

pub use color::Color;
pub use board::{Board, PlacementError, PlacementInfo};
pub use functions::*;
pub use axes::*;
pub use directions::*;

/// Width and Height of the Gomoku board
pub const GRID_LEN: usize = 19;

#[derive(Debug, Copy, Clone)]
pub struct Axis { // TODO change this ugly name !
    pub x: usize,
    pub y: usize,
}

/// Unit value of the Grid, indicate state of each Grid tile
pub type Tile = Option<color::Color>;

/// The type used to represent the Gomoku grid
pub type Grid = [[Tile; GRID_LEN]; GRID_LEN];

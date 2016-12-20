#![feature(inclusive_range_syntax)]

#![feature(test)] extern crate test;

pub mod board;
mod color;
mod functions;
mod axes;
mod directions;

pub use board::{Board, PlaceInfo, PlaceError, PlaceResult};
pub use color::Color;
pub use functions::*;
pub use axes::*;
pub use directions::*;

/// Width and Height of the Gomoku board
pub const GRID_LEN: usize = 19;

pub type Position = (usize, usize);

/// Unit value of the Grid, indicate state of each Grid tile
pub type Tile = Option<color::Color>;

/// The type used to represent the Gomoku grid
pub type Grid = [[Tile; GRID_LEN]; GRID_LEN];

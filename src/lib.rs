#![feature(inclusive_range_syntax)]

#![feature(test)] extern crate test;

/// Main module, contains basics `Structures`.
pub mod board;
mod color;
mod functions;
mod axes;
mod directions;

pub use color::*;
pub use functions::*;
pub use axes::*;
pub use directions::*;

/// Represent the `width` and `height` of the `Grid`.
pub const GRID_LEN: usize = 19;

/// Represent a position on the `Grid`.
pub type Position = (usize, usize);

/// Represent a `Grid` tile, `None` indicates no stone.
pub type Tile = Option<color::Color>;

/// Represent the Gomoku `grid` (19x19).
pub type Grid = [[Tile; GRID_LEN]; GRID_LEN];

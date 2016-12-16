#![feature(inclusive_range_syntax)]

#![feature(test)] extern crate test;

pub mod color;
pub mod board;
pub mod functions;

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

pub const HORIZONTAL: usize = 0;
pub const DIAGONAL_UP: usize  = 1;
pub const VERTICAL: usize  = 2;
pub const DIAGONAL_DOWN: usize  = 3;

pub const TOP: usize = 0;
pub const TOP_RIGHT: usize = 1;
pub const RIGHT: usize = 2;
pub const BOT_RIGHT: usize = 3;
pub const BOT: usize = 4;
pub const BOT_LEFT: usize = 5;
pub const LEFT: usize = 6;
pub const TOP_LEFT: usize = 7;

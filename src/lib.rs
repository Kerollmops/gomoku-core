#![feature(inclusive_range_syntax)]

#![feature(test)] extern crate test;

pub mod color;
pub mod tile;
pub mod board;
pub mod functions;

const GRID_LEN: usize = 19;

pub type Grid = [[tile::Tile; GRID_LEN]; GRID_LEN];

#[derive(Debug, Copy, Clone)]
pub struct Axis { // TODO change this ugly name !
    pub x: usize,
    pub y: usize,
}

#![feature(test)] extern crate test;

mod color;
mod tile;
mod board;
mod functions;

const GRID_LEN: usize = 19;

#[derive(Debug, Copy, Clone)]
pub struct Axis {
    pub x: usize,
    pub y: usize,
}

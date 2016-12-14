#![feature(inclusive_range_syntax)]

#![feature(test)] extern crate test;

mod color;
mod tile;
mod board;
mod functions;

const GRID_LEN: usize = 19;

#[derive(Debug, Copy, Clone)]
pub struct Axis { // TODO change this ugly name !
    pub x: usize,
    pub y: usize,
}

use std::collections::btree_set::BTreeSet;
use ::{ Position, Color, Grid };
use ::axes::*;

mod alignments;
mod free_threes;
mod captures;

pub use self::alignments::*;
pub use self::free_threes::*;
pub use self::captures::*;

pub fn captures_on_horizontal(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if x >= 2 && x < ::GRID_LEN - 1 {
        for y in y - backward..y + forward {
            if grid[x + 1][y] == None && grid[x - 1][y] == Some(color)
               && grid[x - 2][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x + 1, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x - 2][y] == None && grid[x - 1][y] == Some(color)
                    && grid[x + 1][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x - 2, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
    }
    if x < ::GRID_LEN - 2 && x >= 1 {
        for y in y - backward..y + forward {
            if grid[x - 1][y] == None && grid[x + 1][y] == Some(color)
               && grid[x + 2][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x - 1, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x + 2][y] == None && grid[x + 1][y] == Some(color)
                    && grid[x - 1][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x + 2, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
    }
    captures
}

pub fn captures_on_diagonal_up(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, back, forw, _) = align;
    for (x, y) in (x - back..x + forw).zip(y - back..y + forw) {
        if x >= 2 && x < ::GRID_LEN - 1 && y >= 2 && y < ::GRID_LEN - 1 {
            if grid[x + 1][y + 1] == None && grid[x - 1][y - 1] == Some(color)
               && grid[x - 2][y - 2] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x + 1, y + 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x - 2][y - 2] == None && grid[x - 1][y - 1] == Some(color)
                    && grid[x + 1][y + 1] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x - 2, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
        if x >= 1 && x < ::GRID_LEN - 2 && y >= 1 && y < ::GRID_LEN - 2 {
            if grid[x - 1][y - 1] == None && grid[x + 1][y + 1] == Some(color)
               && grid[x + 2][y + 2] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x - 1, y - 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x + 2][y + 2] == None && grid[x + 1][y + 1] == Some(color)
                    && grid[x - 1][y - 1] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x - 2, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
    }
   captures
}

pub fn captures_on_vertical(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if y >= 2 && y < ::GRID_LEN - 1 {
        for x in x - backward..x + forward {
            if grid[x][y - 2] == None && grid[x][y - 1] == Some(color)
               && grid[x][y + 1] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x][y + 1] == None && grid[x][y - 1] == Some(color)
                    && grid[x][y - 2] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y + 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
    }
    if y < ::GRID_LEN - 2 && y >= 1 {
        for x in x - backward..x + forward {
            if grid[x][y + 2] == None && grid[x][y + 1] == Some(color)
               && grid[x][y - 1] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y + 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x][y - 1] == None && grid[x][y + 1] == Some(color)
                    && grid[x][y + 2] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y - 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
    }
    captures
}

pub fn captures_on_diagonal_down(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, back, forw, _) = align;
    for (x, y) in (x - back..x + forw).zip(y - back..y + forw) {
        if x >= 2 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 2 {
            if grid[x - 2][y + 2] == None && grid[x - 1][y + 1] == Some(color)
               && grid[x + 1][y - 1] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x - 2, y + 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x + 1][y - 1] == None && grid[x - 1][y + 1] == Some(color)
                    && grid[x - 2][y + 2] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x + 1, y - 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
        if x >= 1 && x < ::GRID_LEN - 2 && y >= 2 && y < ::GRID_LEN - 1 {
            if grid[x + 2][y - 2] == None && grid[x + 1][y - 1] == Some(color)
               && grid[x - 1][y + 1] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x + 2, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
            else if grid[x - 1][y + 1] == None && grid[x + 1][y - 1] == Some(color)
                    && grid[x + 2][y - 2] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x - 1, y + 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures.insert(pos);
                }
            }
        }
    }
    captures
}

use ::{ Position, Color, Grid };
use ::axes::*;

mod alignments;
mod free_threes;
mod captures;

pub use self::alignments::*;
pub use self::free_threes::*;
pub use self::captures::*;

#[derive(Debug)]
pub struct AlignmentCaptures {
    alignment: Alignment,
    captures: [Option<Position>; ::GRID_LEN]
}

fn captures_on_horizontal(grid: &Grid, pos: Position, color: Color, align: Alignment) -> AlignmentCaptures {
    let mut captures = [None; ::GRID_LEN];
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if x >= 2 && x < ::GRID_LEN - 1 {
        for (i, y) in (y - backward..y + forward).enumerate() {
            if grid[x + 1][y] == None && grid[x - 1][y] == Some(color)
               && grid[x - 2][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x + 1, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x - 2][y] == None && grid[x - 1][y] == Some(color)
                    && grid[x + 1][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x - 2, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
    }
    if x < ::GRID_LEN - 2 && x >= 1 {
        for (i, y) in (y - backward..y + forward).enumerate() {
            if grid[x - 1][y] == None && grid[x + 1][y] == Some(color)
               && grid[x + 2][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x - 1, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x + 2][y] == None && grid[x + 1][y] == Some(color)
                    && grid[x - 1][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x + 2, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
    }
    AlignmentCaptures {
        alignment: align,
        captures: captures
    }
}

fn captures_on_diagonal_up(grid: &Grid, pos: Position, color: Color, align: Alignment) -> AlignmentCaptures {
    let mut captures = [None; ::GRID_LEN];
    let (x, y) = pos;
    let Alignment(_, back, forw, _) = align;
    for (i, (x, y)) in (x - back..x + forw).zip(y - back..y + forw).enumerate() {
        if x >= 2 && x < ::GRID_LEN - 1 && y >= 2 && y < ::GRID_LEN - 1 {

            if grid[x + 1][y + 1] == None && grid[x - 1][y - 1] == Some(color)
               && grid[x - 2][y - 2] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x + 1, y + 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x - 2][y - 2] == None && grid[x - 1][y - 1] == Some(color)
                    && grid[x + 1][y + 1] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x - 2, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
        if x >= 1 && x < ::GRID_LEN - 2 && y >= 1 && y < ::GRID_LEN - 2 {
            if grid[x - 1][y - 1] == None && grid[x + 1][y + 1] == Some(color)
               && grid[x + 2][y + 2] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x - 1, y - 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x + 2][y + 2] == None && grid[x + 1][y + 1] == Some(color)
                    && grid[x - 1][y - 1] == Some(-color) {
                let aligns = Axes::new_diagonal_up(align);
                let pos = (x - 2, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
    }
    AlignmentCaptures {
        alignment: align,
        captures: captures
    }
}

fn captures_on_vertical(grid: &Grid, pos: Position, color: Color, align: Alignment) -> AlignmentCaptures {
    let mut captures = [None; ::GRID_LEN];
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if y >= 2 && y < ::GRID_LEN - 1 {
        for (i, x) in (x - backward..x + forward).enumerate() {
            if grid[x][y - 2] == None && grid[x][y - 1] == Some(color)
               && grid[x][y + 1] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x][y + 1] == None && grid[x][y - 1] == Some(color)
                    && grid[x][y - 2] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y + 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
    }
    if y < ::GRID_LEN - 2 && y >= 1 {
        for (i, x) in (x - backward..x + forward).enumerate() {
            if grid[x][y + 2] == None && grid[x][y + 1] == Some(color)
               && grid[x][y - 1] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y + 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x][y - 1] == None && grid[x][y + 1] == Some(color)
                    && grid[x][y + 2] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y - 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
    }
    AlignmentCaptures {
        alignment: align,
        captures: captures
    }
}

fn captures_on_diagonal_down(grid: &Grid, pos: Position, color: Color, align: Alignment) -> AlignmentCaptures {
    let mut captures = [None; ::GRID_LEN];
    let (x, y) = pos;
    let Alignment(_, back, forw, _) = align;
    for (i, (x, y)) in (x - back..x + forw).zip(y - back..y + forw).enumerate() {
        if x >= 2 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 2 {
            if grid[x - 2][y + 2] == None && grid[x - 1][y + 1] == Some(color)
               && grid[x + 1][y - 1] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x - 2, y + 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x + 1][y - 1] == None && grid[x - 1][y + 1] == Some(color)
                    && grid[x - 2][y + 2] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x + 1, y - 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
        if x >= 1 && x < ::GRID_LEN - 2 && y >= 2 && y < ::GRID_LEN - 1 {
            if grid[x + 2][y - 2] == None && grid[x + 1][y - 1] == Some(color)
               && grid[x - 1][y + 1] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x + 2, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
            else if grid[x - 1][y + 1] == None && grid[x + 1][y - 1] == Some(color)
                    && grid[x + 2][y - 2] == Some(-color) {
                let aligns = Axes::new_diagonal_down(align);
                let pos = (x - 1, y + 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some(pos);
                }
            }
        }
    }
    AlignmentCaptures {
        alignment: align,
        captures: captures
    }
}

// FIXME move `if` inside `for` loop
pub fn captures_on_alignment(grid: &Grid, pos: Position, color: Color, align: Alignment, axis: Axis) -> AlignmentCaptures {
    match axis {
        HORIZONTAL => captures_on_horizontal(grid, pos, color, align),
        DIAGONAL_UP => captures_on_diagonal_up(grid, pos, color, align),
        VERTICAL => captures_on_vertical(grid, pos, color, align),
        DIAGONAL_DOWN => captures_on_diagonal_down(grid, pos, color, align),
        _ => unreachable!()
    }
}

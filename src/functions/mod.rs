use ::{ Position, Color, Grid, Direction };
use ::axes::*;
use ::directions::*;

mod alignments;
mod free_threes;
mod captures;

pub use self::alignments::*;
pub use self::free_threes::*;
pub use self::captures::*;

fn captures_on_horizontal(grid: &Grid, pos: Position, color: Color, align: Alignment)
                          -> Vec<(Position, Direction)> { // TODO [Tile; 19] !!!
    let mut captures = Vec::with_capacity(align.len());
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if x >= 2 {
        for y in y - backward..y + forward {
            if grid[x - 1][y] == Some(color) && grid[x - 2][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) == 2 {
                    captures.push(((x - 2, y), BOT));
                }
            }
        }
    }
    captures
}

fn captures_on_diagonal_up(grid: &Grid, pos: Position, color: Color, align: Alignment)
                           -> Vec<(Position, Direction)> {
    let mut captures = Vec::with_capacity(align.len());

    //

    captures
}

fn captures_on_vertical(grid: &Grid, pos: Position, color: Color, align: Alignment)
                        -> Vec<(Position, Direction)> {
    let mut captures = Vec::with_capacity(align.len());

    //

    captures
}

fn captures_on_diagonal_down(grid: &Grid, pos: Position, color: Color, align: Alignment)
                             -> Vec<(Position, Direction)> {
    let mut captures = Vec::with_capacity(align.len());

    //

    captures
}

pub fn captures_on_alignment(grid: &Grid, pos: Position, color: Color, align: Alignment,
                             axis: Axis) -> Vec<(Position, Direction)> {

    // check if it's not a free_threes !!!
    match axis {
        HORIZONTAL => captures_on_horizontal(grid, pos, color, align),
        DIAGONAL_UP => captures_on_diagonal_up(grid, pos, color, align),
        VERTICAL => captures_on_vertical(grid, pos, color, align),
        DIAGONAL_DOWN => captures_on_diagonal_down(grid, pos, color, align),
        _ => unreachable!()
    }
}

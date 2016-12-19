use ::{ Position, Color, Grid, Direction };
use ::axes::*;
use ::directions::*;

mod alignments;
mod free_threes;
mod captures;

pub use self::alignments::*;
pub use self::free_threes::*;
pub use self::captures::*;

#[derive(Debug)]
pub struct CapturesAlignment {
    alignment: Alignment,
    captures: [Option<(Position, Direction)>; ::GRID_LEN]
}

fn captures_on_horizontal(grid: &Grid,
                          pos: Position,
                          color: Color,
                          align: Alignment)
                          -> CapturesAlignment {

    let mut captures = [None; ::GRID_LEN];
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if x >= 2 && x < ::GRID_LEN - 1 {
        for (i, y) in (y - backward..y + forward).enumerate() {
            if grid[x - 2][y] == Some(-color) && grid[x - 1][y] == Some(color)
               && grid[x + 1][y] == None {
                let aligns = Axes::new_horizontal(align);
                let pos = (x - 2, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, BOT));
                }
            }
            else if grid[x - 2][y] == None && grid[x - 1][y] == Some(color)
                    && grid[x + 1][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x + 1, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, BOT));
                }
            }
        }
    }
    if x < ::GRID_LEN - 2 && x >= 1 {
        for (i, y) in (y - backward..y + forward).enumerate() {
            if grid[x + 2][y] == Some(-color) && grid[x + 1][y] == Some(color)
               && grid[x - 1][y] == None {
                let aligns = Axes::new_horizontal(align);
                let pos = (x + 2, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, TOP));
                }
            }
            else if grid[x + 2][y] == None && grid[x + 1][y] == Some(color)
                    && grid[x - 1][y] == Some(-color) {
                let aligns = Axes::new_horizontal(align);
                let pos = (x - 1, y);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, TOP));
                }
            }
        }
    }
    CapturesAlignment {
        alignment: align,
        captures: captures
    }
}

fn captures_on_diagonal_up(grid: &Grid,
                           pos: Position,
                           color: Color,
                           align: Alignment)
                           -> CapturesAlignment {
    let mut captures = [None; ::GRID_LEN];
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if x >= 2 && y >= 2 {
        for (i, (x, y)) in (x - backward..x + forward)
                           .zip(y - backward..x + forward).enumerate() {
            if grid[x - 2][y - 2] == Some(-color) && grid[x - 1][y - 1] == Some(color)
               && grid[x + 1][y + 1] == None {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, TOP_LEFT));
                }
            }
        }
    }
    if y < ::GRID_LEN - 2 {
        for (i, x) in (x - backward..x + forward).enumerate() {
            if grid[x][y + 2] == Some(-color) && grid[x][y + 1] == Some(color)
               && grid[x][y - 1] == None {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y + 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, RIGHT));
                }
            }
        }
    }
    CapturesAlignment {
        alignment: align,
        captures: captures
    }
}

fn captures_on_vertical(grid: &Grid,
                        pos: Position,
                        color: Color,
                        align: Alignment)
                        -> CapturesAlignment {

    let mut captures = [None; ::GRID_LEN];
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if y >= 2 && y < ::GRID_LEN - 1 {
        for (i, x) in (x - backward..x + forward).enumerate() {
            if grid[x][y - 2] == Some(-color) && grid[x][y - 1] == Some(color)
               && grid[x][y + 1] == None {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y - 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, LEFT));
                }
            }
            else if grid[x][y - 2] == None && grid[x][y - 1] == Some(color)
                    && grid[x][y + 1] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y + 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, LEFT));
                }
            }
        }
    }
    if y < ::GRID_LEN - 2 && y >= 1 {
        for (i, x) in (x - backward..x + forward).enumerate() {
            if grid[x][y + 2] == Some(-color) && grid[x][y + 1] == Some(color)
               && grid[x][y - 1] == None {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y + 2);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, RIGHT));
                }
            }
            else if grid[x][y + 2] == None && grid[x][y + 1] == Some(color)
                    && grid[x][y - 1] == Some(-color) {
                let aligns = Axes::new_vertical(align);
                let pos = (x, y - 1);
                if get_free_threes(grid, pos, -color, &aligns).count(|x| *x) != 2 {
                    captures[i] = Some((pos, RIGHT));
                }
            }
        }
    }
    CapturesAlignment {
        alignment: align,
        captures: captures
    }
}

fn captures_on_diagonal_down(grid: &Grid,
                             pos: Position,
                             color: Color,
                             align: Alignment)
                             -> CapturesAlignment {
    let mut captures = [None; ::GRID_LEN];
    CapturesAlignment {
        alignment: align,
        captures: captures
    }
}

// TODO take Axes<Alignments> and return Axes<CapturesAlignment> ???
// TODO use alignment functions horizontal_alignment
pub fn captures_on_alignment(grid: &Grid,
                             pos: Position,
                             color: Color,
                             align: Alignment,
                             axis: Axis) -> CapturesAlignment {

    // TODO make thsi accessible
    // alignments::horizontal_alignment(grid, pos, color);

    match axis {
        HORIZONTAL => captures_on_horizontal(grid, pos, color, align),
        DIAGONAL_UP => captures_on_diagonal_up(grid, pos, color, align),
        VERTICAL => captures_on_vertical(grid, pos, color, align),
        DIAGONAL_DOWN => captures_on_diagonal_down(grid, pos, color, align),
        _ => unreachable!()
    }
}

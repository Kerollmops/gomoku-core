use std::collections::btree_set::BTreeSet;
use ::{Grid, Position, Color, Alignment};
use axes::*;
use functions::alignments::BoundState::*;
use ::get_alignments;

#[inline]
fn resolve_capture_position((x, y): Position, color: Color, axis: Axis, align: Alignment) -> Option<Position> {
    match align {
        Alignment(Tile(Some(c)), 1, 0, Tile(None)) if c == -color => {
            match axis {
                HORIZONTAL => Some((x, y + 1)),
                DIAGONAL_UP => Some((x - 1, y + 1)),
                VERTICAL => Some((x + 1, y)),
                DIAGONAL_DOWN => Some((x + 1, y + 1)),
                _ => unreachable!("Wrong axis!")
            }
        },
        Alignment(Tile(None), 1, 0, Tile(Some(c))) if c == -color => {
            match axis {
                HORIZONTAL => Some((x, y - 2)),
                DIAGONAL_UP => Some((x + 2, y - 2)),
                VERTICAL => Some((x - 2, y)),
                DIAGONAL_DOWN => Some((x - 2, y - 2)),
                _ => unreachable!("Wrong axis!")
            }
        },
        Alignment(Tile(Some(c)), 0, 1, Tile(None)) if c == -color => {
            match axis {
                HORIZONTAL => Some((x, y + 2)),
                DIAGONAL_UP => Some((x - 2, y + 2)),
                VERTICAL => Some((x + 2, y)),
                DIAGONAL_DOWN => Some((x + 2, y + 2)),
                _ => unreachable!("Wrong axis!")
            }
        },
        Alignment(Tile(None), 0, 1, Tile(Some(c))) if c == -color => {
            match axis {
                HORIZONTAL => Some((x, y - 1)),
                DIAGONAL_UP => Some((x + 1, y - 1)),
                VERTICAL => Some((x - 1, y)),
                DIAGONAL_DOWN => Some((x - 1, y - 1)),
                _ => unreachable!("Wrong axis!")
            }
        },
        _ => None
    }
}

/// Returns a `BTreeSet` of possible positions for captures on an horizontal `Alignment`.
pub fn captures_on_horizontal(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if x >= 2 && x < ::GRID_LEN - 1 {
        for y in y - backward..y + forward {
            let pos = (x, y);
            for (axis, align) in get_alignments(grid, pos, color).iter().enumerate() {
                if axis != HORIZONTAL {
                    if let Some(pos) = resolve_capture_position(pos, color, axis, *align) {
                        captures.insert(pos);
                    }
                }
            }
        }
    }
    if x < ::GRID_LEN - 2 && x >= 1 {
        for y in y - backward..y + forward {
            let pos = (x, y);
            for (axis, align) in get_alignments(grid, pos, color).iter().enumerate() {
                if axis != HORIZONTAL {
                    if let Some(pos) = resolve_capture_position(pos, color, axis, *align) {
                        captures.insert(pos);
                    }
                }
            }
        }
    }
    captures
}

/// Returns a `BTreeSet` of possible positions for captures on a diagonal up `Alignment`.
pub fn captures_on_diagonal_up(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, back, forw, _) = align;
    for (x, y) in (x - forw...x + back).rev().zip(y - back...y + forw) {
        let pos = (x, y);
        for (axis, align) in get_alignments(grid, pos, color).iter().enumerate() {
            if axis != DIAGONAL_UP {
                if let Some(pos) = resolve_capture_position(pos, color, axis, *align) {
                    captures.insert(pos);
                }
            }
        }
    }
    captures
}

/// Returns a `BTreeSet` of possible positions for captures on a vertical `Alignment`.
pub fn captures_on_vertical(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, backward, forward, _) = align;
    if y >= 2 && y < ::GRID_LEN - 1 {
        for x in x - backward..x + forward {
            let pos = (x, y);
            for (axis, align) in get_alignments(grid, pos, color).iter().enumerate() {
                if axis != VERTICAL {
                    if let Some(pos) = resolve_capture_position(pos, color, axis, *align) {
                        captures.insert(pos);
                    }
                }
            }
        }
    }
    if y < ::GRID_LEN - 2 && y >= 1 {
        for x in y - backward..y + forward {
            let pos = (x, y);
            for (axis, align) in get_alignments(grid, pos, color).iter().enumerate() {
                if axis != VERTICAL {
                    if let Some(pos) = resolve_capture_position(pos, color, axis, *align) {
                        captures.insert(pos);
                    }
                }
            }
        }
    }
    captures
}

/// Returns a `BTreeSet` of possible positions for captures on a diagonal down `Alignment`.
pub fn captures_on_diagonal_down(grid: &Grid, pos: Position, color: Color, align: Alignment) -> BTreeSet<Position> {
    let mut captures = BTreeSet::new();
    let (x, y) = pos;
    let Alignment(_, back, forw, _) = align;
    for (x, y) in (x - back...x + forw).zip(y - back...y + forw) {
        let pos = (x, y);
        for (axis, align) in get_alignments(grid, pos, color).iter().enumerate() {
            if axis != DIAGONAL_DOWN {
                if let Some(pos) = resolve_capture_position(pos, color, axis, *align) {
                    captures.insert(pos);
                }
            }
        }
    }
    captures
}

/// Returns a `BTreeSet` of possible positions for captures on the givens `Axis` and `Alignment`.
pub fn captures_on_axis(grid: &Grid, pos: Position, color: Color, align: Alignment, axis: Axis) -> BTreeSet<Position> {
    match axis {
        HORIZONTAL => captures_on_horizontal(grid, pos, color, align),
        DIAGONAL_UP => captures_on_diagonal_up(grid, pos, color, align),
        VERTICAL => captures_on_vertical(grid, pos, color, align),
        DIAGONAL_DOWN => captures_on_diagonal_down(grid, pos, color, align),
        _ => unreachable!("Wrong axis!")
    }
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use std::collections::btree_set::BTreeSet;
    use color::Color;
    use functions::alignments::*;
    use functions::captures_on_alignement::*;

    #[bench]
    fn captures_on_alignement_horizontal(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, b, b, n, b, b, w, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut captures = BTreeSet::new();
        captures.insert((3, 3));
        captures.insert((0, 1));

        bencher.iter(|| {
            let pos = (1, 3);
            let align = *get_alignments(&grid, pos, Color::Black).horizontal();
            assert_eq!(captures_on_horizontal(&grid, pos, Color::Black, align), captures)
        });
    }

    #[bench]
    fn captures_on_alignement_diagonal_up(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, b, w, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut captures = BTreeSet::new();
        captures.insert((5, 3));
        captures.insert((1, 5));

        bencher.iter(|| {
            let pos = (2, 5);
            let align = *get_alignments(&grid, pos, Color::Black).diagonal_up();
            assert_eq!(captures_on_diagonal_up(&grid, pos, Color::Black, align), captures)
        });
    }

    #[bench]
    fn captures_on_alignement_vertical(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [w, n, b, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, b, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut captures = BTreeSet::new();
        captures.insert((0, 1));
        captures.insert((4, 0));
        captures.insert((2, 1));
        captures.insert((6, 3));

        bencher.iter(|| {
            let pos = (2, 2);
            let align = *get_alignments(&grid, pos, Color::Black).vertical();
            assert_eq!(captures_on_vertical(&grid, pos, Color::Black, align), captures)
        });
    }

    #[bench]
    fn captures_on_alignement_diagonal_down(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[w, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, b, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, w, b, b, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, w, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut captures = BTreeSet::new();
        captures.insert((1, 0));
        captures.insert((3, 2));
        captures.insert((6, 8));
        captures.insert((7, 9));

        bencher.iter(|| {
            let pos = (4, 4);
            let align = *get_alignments(&grid, pos, Color::Black).diagonal_down();
            assert_eq!(captures_on_diagonal_down(&grid, pos, Color::Black, align), captures)
        });
    }
}

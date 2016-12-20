use std::collections::btree_set::BTreeSet;
use ::{Grid, Position, Color, Axes, Alignment};
use ::get_free_threes;

/// Returns a `BTreeSet` of possible positions for captures for an horizontal `Alignment`.
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

/// Returns a `BTreeSet` of possible positions for captures for a diagonal up `Alignment`.
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

/// Returns a `BTreeSet` of possible positions for captures for a vertical `Alignment`.
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

/// Returns a `BTreeSet` of possible positions for captures for a diagonal down `Alignment`.
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
                    [b, b, b, b, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut captures = BTreeSet::new();
        captures.insert((3, 3));

        bencher.iter(|| {
            let pos = (1, 3);
            let hori_align = *get_alignments(&grid, pos, Color::Black).horizontal();
            assert_eq!(captures_on_horizontal(&grid, pos, Color::Black, hori_align), captures)
        });
    }
}

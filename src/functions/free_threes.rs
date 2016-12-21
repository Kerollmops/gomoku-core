use ::{ Position, Grid, Axes, Color };
use functions::alignments::Alignment;
use functions::alignments::BoundState::*;

fn complete_vertical(grid: &Grid, (x, y): Position, color: Color, align: Alignment) -> bool {
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            (x >= 2 && grid[x - 2][y] == None)
            || (x < ::GRID_LEN - 4 && grid[x + 4][y] == None)
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            (x >= 3 && grid[x - 3][y] == None)
            || (x < ::GRID_LEN - 3 && grid[x + 3][y] == None)
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            (x >= 4 && grid[x - 4][y] == None)
            || (x < ::GRID_LEN - 2 && grid[x + 2][y] == None)
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            (x >= 4 && x < ::GRID_LEN - 1 && grid[x - 4][y] == None
             && grid[x - 3][y] == Some(color))
            || (x >= 2 && x < ::GRID_LEN - 3 && grid[x + 2][y] == Some(color)
                && grid[x + 3][y] == None)
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            (x >= 3 && x < ::GRID_LEN - 2
             && grid[x - 3][y] == None && grid[x - 2][y] == Some(color))
            || (x >= 1 && x < ::GRID_LEN - 4 && grid[x + 3][y] == Some(color)
                && grid[x + 4][y] == None)
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            (x >= 1 && x < ::GRID_LEN - 4 && grid[x + 2][y] == Some(color)
             && grid[x + 3][y] == Some(color) && grid[x + 4][y] == None)
            || (x >= 4 && x < ::GRID_LEN - 1 && grid[x - 4][y] == None
                && grid[x - 3][y] == Some(color) && grid[x - 2][y] == Some(color))
        }
        _ => false,
    }
}

fn complete_diagonal_up(grid: &Grid, (x, y): Position, color: Color, align: Alignment) -> bool {
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            (x < ::GRID_LEN - 2 && y >= 2 && grid[x + 2][y - 2] == None)
            || (x >= 4 && y < ::GRID_LEN - 3 && grid[x - 4][y + 4] == None)
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            (x < ::GRID_LEN - 3 && y >= 3 && grid[x + 3][y - 3] == None)
            || (x >= 3 && y < ::GRID_LEN - 3 && grid[x - 3][y + 3] == None)
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            (x < ::GRID_LEN - 4 && y >= 4 && grid[x + 4][y - 4] == None)
            || (x >= 2 && y < ::GRID_LEN - 2 && grid[x - 2][y + 2] == None)
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            (x >= 1 && x < ::GRID_LEN - 4 && y >= 4 && y < ::GRID_LEN - 1
             && grid[x + 4][y - 4] == None && grid[x + 3][y - 3] == Some(color))
            || (x >= 3 && x < ::GRID_LEN - 2 && y >= 2 && y < ::GRID_LEN - 3
                && grid[x - 2][y + 2] == Some(color) && grid[x - 3][y + 3] == None)
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            (x >= 2 && x < ::GRID_LEN - 3 && y >= 3 && y < ::GRID_LEN - 2
             && grid[x + 3][y - 3] == None && grid[x + 2][y - 2] == Some(color))
            || (x >= 4 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 4
                && grid[x - 3][y + 3] == Some(color) && grid[x - 4][y + 4] == None)
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            (x >= 4 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 4
             && grid[x - 2][y + 2] == Some(color) && grid[x - 3][y + 3] == Some(color)
             && grid[x - 4][y + 4] == None)
            || (x >= 1 && x < ::GRID_LEN - 4 && y >= 4 && y < ::GRID_LEN - 1
                && grid[x + 2][y - 2] == Some(color) && grid[x + 3][y - 3] == Some(color)
                && grid[x + 4][y - 4] == None)
        },
        _ => false,
    }
}

fn complete_horizontal(grid: &Grid, (x, y): Position, color: Color, align: Alignment) -> bool {
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            (y >= 2 && grid[x][y - 2] == None)
            || (y < ::GRID_LEN - 4 && grid[x][y + 4] == None)
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            (y >= 3 && grid[x][y - 3] == None)
            || (y < ::GRID_LEN - 3 && grid[x][y + 3] == None)
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            (y >= 4 && grid[x][y - 4] == None)
            || (y < ::GRID_LEN - 2 && grid[x][y + 2] == None)
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            (y >= 4 && y < ::GRID_LEN - 1
             && grid[x][y - 3] == Some(color) && grid[x][y - 4] == None)
            || (y >= 2 && y < ::GRID_LEN - 3 && grid[x][y + 2] == Some(color)
                && grid[x][y + 3] == None)
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            (y >= 3 && y < ::GRID_LEN - 2
             && grid[x][y - 3] == None && grid[x][y - 2] == Some(color))
            || (y >= 1 && y < ::GRID_LEN - 4 && grid[x][y + 3] == Some(color)
                && grid[x][y + 4] == None)
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            (y >= 1 && y < ::GRID_LEN - 4 && grid[x][y + 2] == Some(color)
             && grid[x][y + 3] == Some(color) && grid[x][y + 4] == None)
            || (y >= 4 && y < ::GRID_LEN - 1 && grid[x][y - 4] == None
                && grid[x][y - 3] == Some(color) && grid[x][y - 2] == Some(color))
        },
        _ => false,
    }
}

fn complete_diagonal_down(grid: &Grid, (x, y): Position, color: Color, align: Alignment) -> bool {
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            (x >= 2 && y >= 2 && grid[x - 2][y - 2] == None)
            || (x < ::GRID_LEN - 4 && y < ::GRID_LEN - 4
                && grid[x + 4][y + 4] == None)
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            (x >= 3 && y >= 3 && grid[x - 3][y - 3] == None)
            || (x < ::GRID_LEN - 3 && y < ::GRID_LEN - 3
                && grid[x + 3][y + 3] == None)
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            (x >= 4 && y >= 4 && grid[x - 4][y - 4] == None)
            || (x < ::GRID_LEN - 2 && y < ::GRID_LEN - 2
                && grid[x + 2][y + 2] == None)
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            (x >= 4 && x < ::GRID_LEN - 1 && y >= 4 && y < ::GRID_LEN - 1
             && grid[x - 4][y - 4] == None && grid[x - 3][y - 3] == Some(color))
            || (x >= 2 && x < ::GRID_LEN - 3 && y >= 2 && y < ::GRID_LEN - 3
                && grid[x + 2][y + 2] == Some(color) && grid[x + 3][y + 3] == None)
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            (x >= 3 && x < ::GRID_LEN - 2 && y >= 3 && y < ::GRID_LEN - 2
             && grid[x - 3][y - 3] == None && grid[x - 2][y - 2] == Some(color))
            || (x >= 1 && x < ::GRID_LEN - 4 && y >= 1 && y < ::GRID_LEN - 4
                && grid[x + 3][y + 3] == Some(color) && grid[x + 4][y + 4] == None)
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            (x >= 1 && x < ::GRID_LEN - 4 && y >= 1 && y < ::GRID_LEN - 4
             && grid[x + 2][y + 2] == Some(color) && grid[x + 3][y + 3] == Some(color)
             && grid[x + 4][y + 4] == None)
            || (x >= 4 && x < ::GRID_LEN - 1 && y >= 4 && y < ::GRID_LEN - 1
                && grid[x - 2][y - 2] == Some(color) && grid[x - 3][y - 3] == Some(color)
                && grid[x - 4][y - 4] == None)
        },
        _ => false,
    }
}

/// Returns the free-threes at `pos` for the `color`.
pub fn get_free_threes(grid: &Grid, pos: Position, color: Color, aligns: &Axes<Alignment>) -> Axes<bool> {
    let vert = complete_vertical(grid, pos, color, *aligns.vertical());
    let diag_up = complete_diagonal_up(grid, pos, color, *aligns.diagonal_up());
    let hori = complete_horizontal(grid, pos, color, *aligns.horizontal());
    let diag_down = complete_diagonal_down(grid, pos, color, *aligns.diagonal_down());

    Axes::new(vert, diag_up, hori, diag_down)
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use ::axes::*;
    use ::{get_alignments, get_free_threes, Axes};
    use ::color::Color;

    impl Default for Axes<bool> {
        fn default() -> Axes<bool> {
            Axes::new(false, false, false, false)
        }
    }

    #[bench]
    fn six_free_three_horizontal(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, b, b, n, n, n, n, n, n, n, n, n, n, n, b, b, b, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, b, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, b, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut free_threes = Axes::default();
        (*free_threes)[HORIZONTAL] = true;

        bencher.iter(|| {
            let pos = (1, 1);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 17);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (3, 5);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (5, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (7, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (9, 5);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn not_six_free_three_horizontal(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [w, b, b, b, n, n, n, n, n, n, n, n, n, n, w, b, b, b, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, b, w, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, b, n, b, w, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, w, b, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, w, b, n, b, b, w, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let free_threes = Axes::default();

        bencher.iter(|| {
            let pos = (1, 1);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 17);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (3, 5);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (5, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (7, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (9, 5);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn six_free_three_diagonal_up(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, b, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, b, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, b, n, n, n, n],
                    [n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, b, b, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, b, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, b, b, n, n, n],
                    [n, b, n, n, n, n, n, n, n, n, n, n, n, b, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut free_threes = Axes::default();
        (*free_threes)[DIAGONAL_UP] = true;

        bencher.iter(|| {
            let pos = (4, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (15, 16);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (14, 16);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (15, 3);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (4, 14);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (10, 8);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn not_six_free_three_diagonal_up(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, b, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, w, n, n],
                    [n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, b, n, n, n],
                    [b, n, n, n, n, n, n, n, n, n, n, n, n, n, b, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, w, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, b, n, w],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, w, n, b, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, b, n, b, n, n],
                    [n, b, n, n, n, n, n, n, n, n, n, n, n, b, n, b, n, n, n],
                    [w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let free_threes = Axes::default();

        bencher.iter(|| {
            let pos = (2, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (16, 16);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (14, 16);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (15, 3);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (4, 14);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (10, 8);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn six_free_three_vertical(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, n, b, n, b, n, n, n, b, n, n, b, n, n, n, b, n, n, n],
                    [b, n, b, n, b, n, n, n, b, n, n, b, n, n, n, n, n, n, n],
                    [b, n, b, n, n, n, n, n, n, n, n, n, n, n, n, b, n, n, n],
                    [n, n, n, n, b, n, n, n, b, n, n, b, n, n, n, b, n, n, n],
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
                    [b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut free_threes = Axes::default();
        (*free_threes)[VERTICAL] = true;

        bencher.iter(|| {
            let pos = (3, 0);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (15, 0);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (3, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (2, 4);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 8);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (4, 11);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 15);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn not_six_free_three_vertical(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[w, n, n, n, n, n, n, n, w, n, n, w, n, n, n, n, n, n, n],
                    [b, n, b, n, b, n, n, n, b, n, n, b, n, n, n, b, n, n, n],
                    [b, n, b, n, b, n, n, n, b, n, n, b, n, n, n, n, n, n, n],
                    [b, n, b, n, w, n, n, n, w, n, n, n, n, n, n, b, n, n, n],
                    [n, n, w, n, b, n, n, n, b, n, n, b, n, n, n, b, n, n, n],
                    [n, n, n, n, n, n, n, n, w, n, n, n, n, n, n, w, n, n, n],
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
                    [b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let free_threes = Axes::default();

        bencher.iter(|| {
            let pos = (3, 0);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (16, 0);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (3, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (2, 4);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 8);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (4, 11);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 15);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn six_free_three_diagonal_down(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, n, n, n, n, n, b, n, n, n, n, n, n, b, n, n, n, n],
                    [n, n, b, n, n, n, n, n, b, n, n, n, n, n, n, b, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, b, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, b, n, n, n, n, n, n, n, b, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, b, n, n],
                    [n, n, n, n, b, n, n, n, n, b, n, n, n, n, n, n, n, b, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut free_threes = Axes::default();
        (*free_threes)[DIAGONAL_DOWN] = true;

        bencher.iter(|| {
            let pos = (2, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (17, 17);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (14, 1);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (4, 17);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 7);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (14, 6);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn not_six_free_three_diagonal_down(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[b, n, n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, n, n, n, n, n, b, n, n, n, n, n, n, b, n, n, n, n],
                    [n, n, b, n, n, n, n, n, b, n, n, n, n, n, n, b, n, n, n],
                    [n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, w, n, n],
                    [n, n, n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, b, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, w, n, n, n, n, n, n, n, b, n, n],
                    [n, n, n, n, b, n, n, n, n, b, n, n, n, n, n, n, n, b, n],
                    [n, n, n, n, n, n, n, n, n, n, w, n, n, n, n, n, n, n, b]];

        let free_threes = Axes::default();

        bencher.iter(|| {
            let pos = (2, 2);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (17, 17);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);


            let pos = (14, 1);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (4, 17);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (1, 7);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);

            let pos = (14, 6);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes);
        });
    }

    #[bench]
    fn double_free_three_horizontal_and_vertical(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut free_threes = Axes::default();
        (*free_threes)[HORIZONTAL] = true;
        (*free_threes)[VERTICAL] = true;

        bencher.iter(|| {
            let pos = (3, 3);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes)
        });
    }

    #[bench]
    fn double_free_three_horizontal_and_diagonal_down(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
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

        let mut free_threes = Axes::default();
        (*free_threes)[HORIZONTAL] = true;
        (*free_threes)[DIAGONAL_DOWN] = true;

        bencher.iter(|| {
            let pos = (3, 3);
            let alignments = get_alignments(&grid, pos, Color::Black);
            assert_eq!(get_free_threes(&grid, pos, Color::Black, &alignments), free_threes)
        });
    }
}

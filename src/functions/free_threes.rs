use ::{ Position, Grid, Axes };
use functions::alignments::Alignment;
use functions::alignments::BoundState::*;

fn complete_horizontal(grid: &Grid, (x, y): Position, align: Alignment) -> bool {
    let tile = grid[x][y].expect("Tile is empty!"); // TODO don't do this in each function
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None];
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if y >= 2 && grid[x][y - 2] == None { true }
            else if y < ::GRID_LEN - 4 && grid[x][y + 4] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if y >= 3 && grid[x][y - 3] == None { true }
            else if y < ::GRID_LEN - 3 && grid[x][y + 3] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if y >= 4 && grid[x][y - 4] == None { true }
            else if y < ::GRID_LEN - 2 && grid[x][y + 2] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            // TODO don't need to test some bounds
            if y >= 4 && y < ::GRID_LEN - 1 && grid[x][y - 4...y + 1] == ft_cut { true }
            else if y >= 2 && y < ::GRID_LEN - 3
                    && (y - 2...y + 3).zip(ft_cut.into_iter().rev())
                                      .all(|(y, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            if y >= 3 && y < ::GRID_LEN - 2 && grid[x][y - 3...y + 2] == ft_cut { true }
            else if y >= 1 && y < ::GRID_LEN - 4
                    && (y - 1...y + 4).zip(ft_cut.into_iter().rev())
                                      .all(|(y, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            if y >= 1 && y < ::GRID_LEN - 4 && grid[x][y - 1...y + 4] == ft_cut { true }
            else if y >= 4 && y < ::GRID_LEN - 1 {
                (y - 4...y + 1).zip(ft_cut.into_iter().rev())
                               .all(|(y, p)| grid[x][y] == *p)
            }
            else { false }
        },
        _ => false,
    }
}

fn complete_diagonal_up(grid: &Grid, (x, y): Position, align: Alignment) -> bool {
    let tile = grid[x][y].expect("Tile is empty!");
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None];
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if x < ::GRID_LEN - 2 && y >= 2 && grid[x + 2][y - 2] == None { true }
            else if x >= 4 && y < ::GRID_LEN - 3 && grid[x - 4][y + 4] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if x < ::GRID_LEN - 3 && y >= 3 && grid[x + 3][y - 3] == None { true }
            else if x >= 3 && y < ::GRID_LEN - 3 && grid[x - 3][y + 3] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if x < ::GRID_LEN - 4 && y >= 4 && grid[x + 4][y - 4] == None { true }
            else if x >= 2 && y < ::GRID_LEN - 2 && grid[x - 2][y + 2] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            if x >= 1 && x < ::GRID_LEN - 4 && y >= 4 && y < ::GRID_LEN - 1
               && (x - 1...x + 4).rev().zip(y - 4...y + 1)
                  .zip(ft_cut.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 3 && x < ::GRID_LEN - 2 && y >= 2 && y < ::GRID_LEN - 3
                    && (x - 3...x + 2).rev().zip(y - 2...y + 3)
                       .zip(ft_cut.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            if x >= 2 && x < ::GRID_LEN - 3 && y >= 3 && y < ::GRID_LEN - 2
               && (x - 2...x + 3).rev().zip(y - 3...y + 2)
                  .zip(ft_cut.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 4 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 4
                    && (x - 4...x + 1).rev().zip(y - 1...y + 4)
                       .zip(ft_cut.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            if x >= 4 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 4
               && (x - 4...x + 1).rev().zip(y - 1...y + 4)
                  .zip(ft_cut.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 1 && x < ::GRID_LEN - 4 && y >= 4 && y < ::GRID_LEN - 1
                    && (x - 1...x + 4).rev().zip(y - 4...y + 1)
                       .zip(ft_cut.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        _ => false,
    }
}

fn complete_vertical(grid: &Grid, (x, y): Position, align: Alignment) -> bool {
    let tile = grid[x][y].expect("Tile is empty!");
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None];
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if x >= 2 && grid[x - 2][y] == None { true }
            else if x < ::GRID_LEN - 4 && grid[x + 4][y] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if x >= 3 && grid[x - 3][y] == None { true }
            else if x < ::GRID_LEN - 3 && grid[x + 3][y] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if x >= 4 && grid[x - 4][y] == None { true }
            else if x < ::GRID_LEN - 2 && grid[x + 2][y] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            if x >= 4 && x < ::GRID_LEN - 1
               && (x - 4...x + 1).zip(ft_cut.into_iter())
                                  .all(|(x, p)| grid[x][y] == *p) { true }
            else if x >= 2 && x < ::GRID_LEN - 3
                    && (x - 2...x + 3).zip(ft_cut.into_iter().rev())
                                      .all(|(x, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            if x >= 3 && x < ::GRID_LEN - 2
               && (x - 2...x + 2).zip(ft_cut.into_iter())
                                 .all(|(x, p)| grid[x][y] == *p) { true }
            else if x >= 1 && x < ::GRID_LEN - 4
                    && (x - 1...x + 4).zip(ft_cut.into_iter().rev())
                                      .all(|(x, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            if x >= 1 && x < ::GRID_LEN - 4
               && (x - 1...x + 4).zip(ft_cut.into_iter())
                                 .all(|(x, p)| grid[x][y] == *p) { true }
            else if x >= 4 && x < ::GRID_LEN - 1
                    && (x - 4...x + 1).zip(ft_cut.into_iter().rev())
                                      .all(|(x, p)| grid[x][y] == *p) { true }
            else { false }
        }
        _ => false,
    }
}

fn complete_diagonal_down(grid: &Grid, (x, y): Position, align: Alignment) -> bool {
    let tile = grid[x][y].expect("Tile is empty!");
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None];
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if x >= 2 && y >= 2 && grid[x - 2][y - 2] == None { true }
            else if x < ::GRID_LEN - 4 && y < ::GRID_LEN - 4
                    && grid[x + 4][y + 4] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if x >= 3 && y >= 3 && grid[x - 3][y - 3] == None { true }
            else if x < ::GRID_LEN - 3 && x < ::GRID_LEN - 3
                    && grid[x + 3][y + 3] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if x >= 4 && y >= 4 && grid[x - 4][y - 4] == None { true }
            else if x < ::GRID_LEN - 2 && y < ::GRID_LEN - 2
                    && grid[x + 2][y + 2] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
            if x >= 4 && x < ::GRID_LEN - 1 && y >= 4 && y < ::GRID_LEN - 1
               && (x - 4...x + 1).zip(y - 4...y + 1)
                  .zip(ft_cut.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
               else if x >= 2 && x < ::GRID_LEN - 3 && y >= 2 && y < ::GRID_LEN - 3
                       && (x - 2...x + 3).zip(y - 2...y + 3)
                          .zip(ft_cut.into_iter().rev())
                          .all(|((x, y), p)| grid[x][y] == *p) { true }
               else { false }
        },
        Alignment(Tile(None), 0, 1, Tile(None)) => {
            if x >= 3 && x < ::GRID_LEN - 2 && y >= 3 && y < ::GRID_LEN - 2
               && (x - 3...x + 2).zip(y - 3...y + 2)
                  .zip(ft_cut.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 1 && x < ::GRID_LEN - 4 && y >= 1 && y < ::GRID_LEN - 4
                    && (x - 1...x + 4).zip(y - 1...y + 4)
                       .zip(ft_cut.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment(Tile(None), 0, 0, Tile(None)) => {
            if x >= 1 && x < ::GRID_LEN - 4 && y >= 1 && y < ::GRID_LEN - 4
               && (x - 1...x + 4).zip(y - 1...y + 4)
                  .zip(ft_cut.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 4 && x < ::GRID_LEN - 1 && y >= 4 && y < ::GRID_LEN - 1
                    && (x - 4...x + 1).zip(y - 4...y + 1)
                       .zip(ft_cut.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        _ => false,
    }
}

/// Returns a list of free-threes for the tile at `pos`
pub fn list_free_threes(grid: &Grid, pos: Position, aligns: &Axes<Alignment>) -> Axes<bool> {
    let hori = complete_horizontal(grid, pos, *aligns.horizontal());
    let diag_up = complete_diagonal_up(grid, pos, *aligns.diagonal_up());
    let vert = complete_vertical(grid, pos, *aligns.vertical());
    let diag_down = complete_diagonal_down(grid, pos, *aligns.diagonal_down());

    Axes::new(hori, diag_up, vert, diag_down)
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use ::free_threes::*;
    use ::alignments::*;
    use ::axes::*;
    use ::color::Color;

    impl Default for Axes<bool> {
        fn default() -> Axes<bool> {
            Axes::new(false, false, false, false)
        }
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (15, 0);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (3, 2);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (2, 4);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 8);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (4, 11);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 15);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (16, 0);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (3, 2);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (2, 4);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 8);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (4, 11);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 15);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
        });
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 17);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (3, 5);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (5, 2);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (7, 2);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (9, 5);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 17);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (3, 5);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (5, 2);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (7, 2);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (9, 5);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (15, 16);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (14, 16);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (15, 3);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (4, 14);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (10, 8);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (16, 16);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (14, 16);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (15, 3);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (4, 14);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (10, 8);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (17, 17);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (14, 1);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (4, 17);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 7);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (14, 6);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (17, 17);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);


            let pos = (14, 1);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (4, 17);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (1, 7);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);

            let pos = (14, 6);
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes);
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes)
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
            let alignments = list_alignments(&grid, pos);
            assert_eq!(list_free_threes(&grid, pos, &alignments), free_threes)
        });
    }
}

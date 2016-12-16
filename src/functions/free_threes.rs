use ::{ Axis, Grid, HORIZONTAL, DIAGONAL_UP, VERTICAL, DIAGONAL_DOWN };
use functions::alignments::{list_alignments, Alignment};
use functions::alignments::BoundState::*;

fn complete_horizontal(grid: &Grid, pos: Axis, align: Alignment) -> bool {
    let Axis{ x, y } = pos;
    let tile = grid[x][y].unwrap();
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None];
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if y >= 2 && y < ::GRID_LEN - 3 && grid[x][y - 2] == None { true } // TODO doesn't need to test some bounds
            else if y >= 1 && y < ::GRID_LEN - 4 && grid[x][y + 4] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if y >= 3 && y < ::GRID_LEN - 2 && grid[x][y - 3] == None { true }
            else if y >= 2 && y < ::GRID_LEN - 3 && grid[x][y + 3] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if y >= 4 && y < ::GRID_LEN - 1 && grid[x][y - 4] == None { true }
            else if y >= 3 && y < ::GRID_LEN - 2 && grid[x][y + 2] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 0, Tile(None)) => {
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

fn complete_diagonal_up(grid: &Grid, pos: Axis, align: Alignment) -> bool {
    let Axis{ x, y } = pos;
    let tile = grid[x][y].unwrap();
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None]; // TODO doesn't need to test None bounds
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if x >= 3 && x < ::GRID_LEN - 2 && y >= 2 && y < ::GRID_LEN - 3
               && grid[x + 2][y - 2] == None { true }
            else if x >= 4 && x < ::GRID_LEN - 1 && y >= 2 && y < ::GRID_LEN - 3
                    && grid[x - 4][y + 4] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if x >= 2 && x < ::GRID_LEN - 3 && y >= 3 && y < ::GRID_LEN - 2
               && grid[x + 3][y - 3] == None { true }
            else if x >= 3 && x < ::GRID_LEN - 2 && y >= 2 && y < ::GRID_LEN - 3
                    && grid[x - 3][y + 3] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if x >= 1 && x < ::GRID_LEN - 4 && y >= 3 && y < ::GRID_LEN - 2
               && grid[x + 4][y - 4] == None { true }
            else if x >= 2 && x < ::GRID_LEN - 3 && y >= 3 && y < ::GRID_LEN - 2
                    && grid[x - 2][y + 2] == None { true }
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

fn complete_vertical(grid: &Grid, pos: Axis, align: Alignment) -> bool {
    let Axis{ x, y } = pos;
    let tile = grid[x][y].unwrap();
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None];
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if x >= 2 && x < ::GRID_LEN - 3 && grid[x - 2][y] == None { true }
            else if x >= 1 && x < ::GRID_LEN - 4 && grid[x + 4][y] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if x >= 3 && x < ::GRID_LEN - 2 && grid[x - 3][y] == None { true }
            else if x >= 2 && x < ::GRID_LEN - 3 && grid[x + 3][y] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if x >= 4 && x < ::GRID_LEN - 1 && grid[x - 4][y] == None { true }
            else if x >= 3 && x < ::GRID_LEN - 2 && grid[x + 2][y] == None { true }
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

fn complete_diagonal_down(grid: &Grid, pos: Axis, align: Alignment) -> bool {
    let Axis{ x, y } = pos;
    let tile = grid[x][y].unwrap();
    let ft_cut = [None, Some(tile), None, Some(tile), Some(tile), None];
    match align {
        Alignment(Tile(None), 0, 2, Tile(None)) => {
            if x >= 2 && x < ::GRID_LEN - 3 && y >= 2 && y < ::GRID_LEN - 3
               && grid[x - 2][y - 2] == None { true }
            else if x >= 1 && x < ::GRID_LEN - 4 && y >= 1 && y < ::GRID_LEN - 4
                    && grid[x + 4][y + 4] == None { true }
            else { false }
        },
        Alignment(Tile(None), 1, 1, Tile(None)) => {
            if x >= 3 && x < ::GRID_LEN - 2 && y >= 3 && y < ::GRID_LEN - 2
               && grid[x - 3][y - 3] == None { true }
            else if x >= 2 && x < ::GRID_LEN - 3 && x >= 2 && x < ::GRID_LEN - 3
                    && grid[x + 3][y + 3] == None { true }
            else { false }
        },
        Alignment(Tile(None), 2, 0, Tile(None)) => {
            if x >= 4 && x < ::GRID_LEN - 1 && y >= 4 && y < ::GRID_LEN - 1
               && grid[x - 4][y - 4] == None { true }
            else if x >= 3 && x < ::GRID_LEN - 2 && y >= 3 && y < ::GRID_LEN - 2
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

pub fn list_free_threes(grid: &Grid, pos: Axis) -> [bool; 4] {
    grid[pos.x][pos.y].expect(&format!("Tile at {:?} is empty!", pos));
    let aligns = list_alignments(grid, pos);

    let mut free_threes = [false; 4];
    free_threes[HORIZONTAL] = complete_horizontal(grid, pos, aligns[HORIZONTAL]);
    free_threes[DIAGONAL_UP] = complete_diagonal_up(grid, pos, aligns[DIAGONAL_UP]);
    free_threes[VERTICAL] = complete_vertical(grid, pos, aligns[VERTICAL]);
    free_threes[DIAGONAL_DOWN] = complete_diagonal_down(grid, pos, aligns[DIAGONAL_DOWN]);
    free_threes
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use functions::free_threes::*;
    use ::{ HORIZONTAL, DIAGONAL_UP, VERTICAL, DIAGONAL_DOWN };
    use color::Color;

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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[VERTICAL] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 0 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 15, y: 0 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 2 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 2, y: 4 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 8 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 11 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 15 }), free_threes);
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

        bencher.iter(|| {
            let free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 0 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 16, y: 0 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 2 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 2, y: 4 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 8 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 11 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 15 }), free_threes);
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[HORIZONTAL] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 1 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 17 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 5 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 5, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 7, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 9, y: 5 }), free_threes);
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

        bencher.iter(|| {
            let free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 1 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 17 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 5 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 5, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 7, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 9, y: 5 }), free_threes);
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[DIAGONAL_UP] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 15, y: 16 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 14, y: 16 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 15, y: 3 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 14 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 10, y: 8 }), free_threes);
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

        bencher.iter(|| {
            let free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 2, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 16, y: 16 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 14, y: 16 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 15, y: 3 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 14 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 10, y: 8 }), free_threes);
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[DIAGONAL_DOWN] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 2, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 17, y: 17 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 14, y: 1 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 17 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 7 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 14, y: 6 }), free_threes);
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

        bencher.iter(|| {
            let free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 2, y: 2 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 17, y: 17 }), free_threes);

            assert_eq!(list_free_threes(&grid, Axis { x: 14, y: 1 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 17 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 7 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 14, y: 6 }), free_threes);
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[HORIZONTAL] = true;
            free_threes[VERTICAL] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 3 }), free_threes)
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[HORIZONTAL] = true;
            free_threes[DIAGONAL_DOWN] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 3 }), free_threes)
        });
    }
}

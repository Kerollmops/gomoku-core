use color::Color;
use tile::Tile;
use ::{Axis, Grid};
use functions::alignments::{list_alignments, Alignment, BoundState};
use functions::alignments::{ HORIZONTAL, DIAGONAL_UP, VERTICAL, DIAGONAL_DOWN }; // TODO move this elsewhere

fn complete_horizontal(grid: &Grid, pos: Axis, align: Alignment) -> bool {
    let Axis{ x, y } = pos;
    let tile = grid[x][y].unwrap();
    let ft = [Some(tile), None, Some(tile), Some(tile)];
    match align {
        a @ Alignment {
            first_bound: BoundState::Tile(None),
            second_bound: BoundState::Tile(None),
            ..
        } if a.alignment_len() == 3 => true,
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 1,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if y >= 4 && y < ::GRID_LEN - 1 && grid[x][y - 3...y] == ft { true }
            else if y >= 2 && y < ::GRID_LEN - 3
                    && (y - 1...y + 2).zip(ft.into_iter().rev())
                                      .all(|(y, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 1,
            second_bound: BoundState::Tile(None),
        } => {
            if y >= 3 && y < ::GRID_LEN - 2 && grid[x][y - 2...y + 1] == ft { true }
            else if y >= 1 && y < ::GRID_LEN - 4
                    && (y...y + 3).zip(ft.into_iter().rev())
                                  .all(|(y, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if y >= 1 && y < ::GRID_LEN - 4 && grid[x][y...y + 3] == ft { true }
            else if y >= 4 && y < ::GRID_LEN - 1 {
                (y - 3...y).zip(ft.into_iter().rev())
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
    let ft = [Some(tile), None, Some(tile), Some(tile)]; // TODO doesn't need to test None bounds
    match align {
        a @ Alignment {
            first_bound: BoundState::Tile(None),
            second_bound: BoundState::Tile(None),
            ..
        } if a.alignment_len() == 3 => true,
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 1,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 1 && x < ::GRID_LEN - 4 && y >= 4 && y < ::GRID_LEN - 1
               && (x...x + 3).rev().zip(y - 3...y)
                  .zip(ft.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 3 && x < ::GRID_LEN - 2 && y >= 2 && y < ::GRID_LEN - 3
                    && (x - 2...x + 1).rev().zip(y - 1...y + 2)
                       .zip(ft.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 1,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 2 && x < ::GRID_LEN - 3 && y >= 3 && y < ::GRID_LEN - 2
               && (x - 1...x + 2).zip(y - 2...y + 1)
                  .zip(ft.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 4 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 4
                    && (x - 3...x).zip(y...y + 3)
                       .zip(ft.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 4 && x < ::GRID_LEN - 1 && y >= 1 && y < ::GRID_LEN - 4
               && (x - 3...x).rev().zip(y...y + 3)
                  .zip(ft.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 1 && x < ::GRID_LEN - 4 && y >= 4 && y < ::GRID_LEN - 1
                    && (x...x + 3).rev().zip(y - 3...y)
                       .zip(ft.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        _ => false,
    }
}

fn complete_vertical(grid: &Grid, pos: Axis, align: Alignment) -> bool {
    let Axis{ x, y } = pos;
    let tile = grid[x][y].unwrap();
    let ft = [Some(tile), None, Some(tile), Some(tile)];
    match align {
        a @ Alignment {
            first_bound: BoundState::Tile(None),
            second_bound: BoundState::Tile(None),
            ..
        } if a.alignment_len() == 3 => true,
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 1,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 4 && x < ::GRID_LEN - 1
                && (x - 3...x).zip(ft.into_iter())
                              .all(|(x, p)| grid[x][y] == *p) { true }
            else if x >= 2 && x < ::GRID_LEN - 3
                    && (x - 1...x + 2).zip(ft.into_iter().rev())
                                      .all(|(x, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 1,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 3 && x < ::GRID_LEN - 2
               && (x - 1...x + 1).zip(ft.into_iter())
                                 .all(|(x, p)| grid[x][y] == *p) { true }
            else if x >= 1 && x < ::GRID_LEN - 4
                    && (x...x + 3).zip(ft.into_iter().rev())
                                  .all(|(x, p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 1 && x < ::GRID_LEN - 4
               && (x...x + 3).zip(ft.into_iter())
                             .all(|(x, p)| grid[x][y] == *p) { true }
            else if x >= 4 && x < ::GRID_LEN - 1
                    && (x - 3...x).zip(ft.into_iter().rev())
                                  .all(|(x, p)| grid[x][y] == *p) { true }
            else { false }
        }
        _ => false,
    }
}

fn complete_diagonal_down(grid: &Grid, pos: Axis, align: Alignment) -> bool {
    let Axis{ x, y } = pos;
    let tile = grid[x][y].unwrap();
    let ft = [Some(tile), None, Some(tile), Some(tile)];
    match align {
        a @ Alignment {
            first_bound: BoundState::Tile(None),
            second_bound: BoundState::Tile(None),
            ..
        } if a.alignment_len() == 3 => true,
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 1,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 4 && x < ::GRID_LEN - 1 && y >= 4 && y < ::GRID_LEN - 1
               && (x - 3...x).zip(y - 3...y)
                  .zip(ft.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
               else if x >= 2 && x < ::GRID_LEN - 3 && y >= 2 && y < ::GRID_LEN - 3
                       && (x - 1...x + 2).zip(y - 1...y + 2)
                          .zip(ft.into_iter().rev())
                          .all(|((x, y), p)| grid[x][y] == *p) { true }
               else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 1,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 3 && x < ::GRID_LEN - 2 && y >= 3 && y < ::GRID_LEN - 2
               && (x - 2...x + 1).zip(y - 2...y + 1)
                  .zip(ft.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 1 && x < ::GRID_LEN - 4 && y >= 1 && y < ::GRID_LEN - 4
                    && (x...x + 3).zip(y...y + 3)
                       .zip(ft.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        Alignment {
            first_bound: BoundState::Tile(None),
            backward: 0,
            forward: 0,
            second_bound: BoundState::Tile(None),
        } => {
            if x >= 1 && x < ::GRID_LEN - 4 && y >= 1 && y < ::GRID_LEN - 4
               && (x...x + 3).zip(y...y + 3)
                  .zip(ft.into_iter())
                  .all(|((x, y), p)| grid[x][y] == *p) { true }
            else if x >= 4 && x < ::GRID_LEN - 1 && y >= 4 && y < ::GRID_LEN - 1
                    && (x - 3...x).zip(y - 3...y)
                       .zip(ft.into_iter().rev())
                       .all(|((x, y), p)| grid[x][y] == *p) { true }
            else { false }
        },
        _ => false,
    }
}

pub fn list_free_threes(grid: &Grid, pos: Axis) -> [bool; 4] {
    let tile = grid[pos.x][pos.y].unwrap();
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
    use functions::alignments::{ HORIZONTAL, DIAGONAL_UP, VERTICAL, DIAGONAL_DOWN };
    use color::Color;

    #[bench]
    fn one_free_three_vertical(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[VERTICAL] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn one_free_three_vertical_cut(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, b, n, n, b, n, n, n, b, n, n, n],
                    [n, n, n, n, b, n, n, n, b, n, n, b, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, b, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[VERTICAL] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 2, y: 4 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 8 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 11 }), free_threes);
            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 15 }), free_threes);
        });
    }

    #[bench]
    fn one_free_three_horizontal(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[HORIZONTAL] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn one_free_three_horizontal_cut(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, b, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[HORIZONTAL] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 5 }), free_threes)
        });
    }

    #[bench]
    fn one_free_three_diagonal_up(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[DIAGONAL_UP] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn one_free_three_diagonal_up_cut(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[DIAGONAL_UP] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn one_free_three_diagonal_down(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[DIAGONAL_DOWN] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn one_free_three_diagonal_down_cut(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];
            free_threes[DIAGONAL_DOWN] = true;

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn not_free_three_vertical(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 4, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn not_free_three_horizontal(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, w, b, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn not_free_three_diagonal_up(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        bencher.iter(|| {
            let mut free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 1, y: 4 }), free_threes)
        });
    }


    #[bench]
    fn not_free_three_diagonal_down(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        bencher.iter(|| {
            let mut free_threes = [false; 4];

            assert_eq!(list_free_threes(&grid, Axis { x: 3, y: 4 }), free_threes)
        });
    }

    #[bench]
    fn double_free_three_horizontal_and_vertical(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
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
        let w = Some(Color::White);
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

use std::collections::btree_set::BTreeSet;
use std::ops::BitAnd;
use ::{Position, Color, Grid, Axes, Directions, Alignment};
use ::{get_alignments, get_free_threes, get_captures};
use functions::captures_on_alignement::captures_on_axis;
use ::directions::*;

/// Number of stones to take to win.
pub const STONES_COUNT_TO_WIN: usize = 10;

/// The main structure: allow you to play on the `Grid` with Gomoku rules.
#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid,
    stones_black_takes: usize,
    stones_white_takes: usize,
}

/// Indicates the error of placement you get
/// when you missplace a stone on the `Board`.
#[derive(Debug)]
pub enum Error {
    TileNotEmpty,
    DoubleFreeThrees(Axes<bool>),
}

/// Indicates the victory condition under the one
/// you win by placing a stone on the `Board`.
#[derive(Debug)]
pub enum Condition {
    CapturedStones { total: usize, captures: Directions<bool> },
    FiveStonesAligned(Axes<Alignment>),
}

/// Gives information on the last successful stone placement on the `Board`.
#[derive(Debug)]
pub enum Info {
    Nothing,
    Captures(Directions<bool>),
    FiveStonesAligned { counteract: Vec<Position> }
}

/// The type returned by the board when placing a stone.
#[derive(Debug)]
pub enum PlaceResult {
    Ok(Info),
    Victory(Condition),
    Err(Error),
}

use self::PlaceResult::*;

impl Board {
    /// Returns the default `Board`.
    pub fn new() -> Board {
        Board {
            grid: [[None; ::GRID_LEN]; ::GRID_LEN],
            stones_black_takes: 0,
            stones_white_takes: 0
        }
    }

    /// Simply puts a stone on the board
    pub fn raw_place_stone(&mut self, (x, y): Position, color: Color) {
        self.grid[x][y] = Some(color)
    }

    /// Simply removes a stone from the board
    pub fn raw_remove_stone(&mut self, (x, y): Position) {
        self.grid[x][y] = None;
    }

    // TODO another solution ?
    fn stones_taken(&self, color: Color) -> usize {
        match color {
            Color::Black => self.stones_black_takes,
            Color::White => self.stones_white_takes,
        }
    }

    fn mut_stones_taken(&mut self, color: Color) -> &mut usize {
        match color {
            Color::Black => &mut self.stones_black_takes,
            Color::White => &mut self.stones_white_takes,
        }
    }

    fn remove_captured_stones(&mut self, (x, y): Position, captures: &Directions<bool>) {
        if captures[TOP] {
            self.grid[x - 1][y] = None;
            self.grid[x - 2][y] = None;
        }
        if captures[TOP_RIGHT] {
            self.grid[x - 1][y + 1] = None;
            self.grid[x - 2][y + 2] = None;
        }
        if captures[RIGHT] {
            self.grid[x][y + 1] = None;
            self.grid[x][y + 2] = None;
        }
        if captures[BOT_RIGHT] {
            self.grid[x + 1][y + 1] = None;
            self.grid[x + 2][y + 2] = None;
        }
        if captures[BOT] {
            self.grid[x + 1][y] = None;
            self.grid[x + 2][y] = None;
        }
        if captures[BOT_LEFT] {
            self.grid[x + 1][y - 1] = None;
            self.grid[x + 2][y - 2] = None;
        }
        if captures[LEFT] {
            self.grid[x][y - 1] = None;
            self.grid[x][y - 2] = None;
        }
        if captures[TOP_LEFT] {
            self.grid[x - 1][y - 1] = None;
            self.grid[x - 2][y - 2] = None;
        }
    }

    fn update_captures(&mut self, pos: Position, color: Color, captures: &Directions<bool>) -> usize {
        self.remove_captured_stones(pos, captures);
        let nb_captures = captures.count(|x| *x);
        *self.mut_stones_taken(color) += nb_captures * 2;
        nb_captures
    }

    fn get_all_possible_captures(&self, color: Color) -> Vec<Position> {
        let mut captures = Vec::new();
        for x in 0..::GRID_LEN {
            for y in 0..::GRID_LEN {
                let pos = (x, y);
                if let None = self.grid[x][y] {
                    for _ in get_captures(&self.grid, pos, color).iter().filter(|x| **x) {
                        let aligns = get_alignments(&self.grid, pos, color);
                        if get_free_threes(&self.grid, pos, color, &aligns).count(|x| *x) != 2 {
                            captures.push(pos);
                        }
                    }
                }
            }
        }
        captures
    }

    // TODO much better way to do this (remove ugly boolean)
    fn get_counter_alignments(&self, pos: Position, color: Color, alignments: &Axes<Alignment>) -> BTreeSet<Position> {
        let mut is_first_insert = true;
        let mut captures = BTreeSet::new();
        for (axis, alignment) in alignments.iter().enumerate()
                                           .filter(|&(_, x)| x.len() >= 5) {
            let capts = captures_on_axis(&self.grid, pos, color, *alignment, axis);
            if is_first_insert == true { captures = capts; }
            else { captures = captures.bitand(&capts); }
            is_first_insert = false;
        }
        captures
    }

    /// Try placing a stone on board respecting rules
    pub fn try_place_stone(&mut self, pos: Position, color: Color) -> PlaceResult {
        let (x, y) = pos;
        if self.grid[x][y].is_some() {
            return Err(Error::TileNotEmpty)
        }

        let alignments = get_alignments(&self.grid, pos, color);
        let free_threes = get_free_threes(&self.grid, pos, color, &alignments);
        let captures = get_captures(&self.grid, pos, color);

        if free_threes.count(|x| *x) == 2 {
            Err(Error::DoubleFreeThrees(free_threes))
        }
        else {
            self.raw_place_stone(pos, color);
            let stones_taken = self.update_captures(pos, color, &captures);

            if alignments.any(|x| x.len() >= 5) {
                if self.stones_taken(-color) + 2 == STONES_COUNT_TO_WIN {
                    let captures = self.get_all_possible_captures(-color);
                    if !captures.is_empty() {
                        return Ok(Info::FiveStonesAligned {
                            counteract: captures
                        })
                    }
                }
                else {
                    let captures = self.get_counter_alignments(pos, color, &alignments);
                    if !captures.is_empty() {
                        return Ok(Info::FiveStonesAligned {
                            counteract: captures.iter().cloned().collect()
                        })
                    }
                }
                Victory(Condition::FiveStonesAligned(alignments))
            }
            else if stones_taken > 0 {
                if self.stones_taken(color) >= STONES_COUNT_TO_WIN {
                    Victory(Condition::CapturedStones {
                        total: self.stones_taken(color),
                        captures: captures
                    })
                }
                else {
                    Ok(Info::Captures(captures))
                }
            }
            else {
                Ok(Info::Nothing)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use color::Color;
    use ::{Alignment, BoundState};
    use super::{Board, PlaceResult};
    use super::{Condition, Error};
    use super::Info::*;

    #[bench]
    fn counteract_alignment_by_breaking(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, w, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        let mut board = Board {
            grid: grid,
            stones_black_takes: 0,
            stones_white_takes: 0,
        };

        let captures = vec![(0, 1), (3, 2), (3, 3)];
        bencher.iter(|| {
            let pos = (1, 3);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Ok(FiveStonesAligned { counteract }) => assert_eq!(counteract, captures),
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn counteract_mulitple_alignments_by_breaking(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, w, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, b, b, n, b, b, w, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut board = Board {
            grid: grid,
            stones_black_takes: 0,
            stones_white_takes: 0,
        };

        let captures = vec![(0, 1), (3, 2)];
        bencher.iter(|| {
            let pos = (1, 3);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Ok(FiveStonesAligned { counteract }) => assert_eq!(counteract, captures),
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn counteract_alignment_with_last_capture(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [b, b, b, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, b, b, w],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut board = Board {
            grid: grid,
            stones_black_takes: 0,
            stones_white_takes: 8,
        };

        let captures = vec![(14, 15)];
        bencher.iter(|| {
            let pos = (1, 3);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Ok(FiveStonesAligned { counteract }) => assert_eq!(counteract, captures),
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn victory_with_captures(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, w, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, w, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        let mut board = Board {
            grid: grid,
            stones_black_takes: 8,
            stones_white_takes: 0,
        };

        bencher.iter(|| {
            let pos = (2, 5);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Victory(Condition::CapturedStones {
                    total: 14,
                    captures
                }) => assert_eq!(captures, [false, false, false, false, true, true, true, false].into()),
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn placement_do_nothing(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        let mut board = Board {
            grid: grid,
            stones_black_takes: 8,
            stones_white_takes: 8,
        };

        bencher.iter(|| {
            let pos = (2, 3);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Ok(Nothing) => (),
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn placement_do_nothing_but_captures(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, w, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, w, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        let mut board = Board {
            grid: grid,
            stones_black_takes: 2,
            stones_white_takes: 0,
        };

        bencher.iter(|| {
            let pos = (2, 5);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Ok(Captures(captures)) => assert_eq!(captures, [false, false, false, false, true, true, true, false].into()),
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn double_free_threes_cant_counteract_alignment(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, w, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, b, b, n, b, b, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, w, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut board = Board {
            grid: grid,
            stones_black_takes: 8,
            stones_white_takes: 8,
        };

        bencher.iter(|| {
            let pos = (4, 6);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Victory(Condition::FiveStonesAligned(aligns)) => {
                    let alignment = Alignment(BoundState::Tile(None), 3, 2, BoundState::Tile(None));
                    assert_eq!(*aligns.horizontal(), alignment)
                },
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn cant_place_on_double_free_threes(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        let mut board = Board {
            grid: grid,
            stones_black_takes: 8,
            stones_white_takes: 0,
        };

        bencher.iter(|| {
            let pos = (4, 3);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Err(Error::DoubleFreeThrees(axes)) => assert_eq!(axes, [true, false, true, false].into()),
                x => panic!("{:?}", x),
            }
        });
    }

    #[bench]
    fn cant_place_on_non_empty_tile(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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

        let mut board = Board {
            grid: grid,
            stones_black_takes: 8,
            stones_white_takes: 0,
        };

        bencher.iter(|| {
            let pos = (2, 3);
            match board.try_place_stone(pos, Color::Black) {
                PlaceResult::Err(Error::TileNotEmpty) => (),
                x => panic!("{:?}", x),
            }
        });
    }
}

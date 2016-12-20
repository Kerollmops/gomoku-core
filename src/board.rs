use std::collections::btree_set::BTreeSet;
use std::ops::BitAnd;
use ::{Position, Color, Grid, Axes, Directions, Alignment};
use ::{get_alignments, get_free_threes, get_captures};
use functions::captures_on_alignement::captures_on_axis;
use ::directions::*;

#[derive(Debug, Clone)]
struct StonesStats {
    rest_stones: usize,
    stones_taken: usize
}

/// The main structure: allow you to play on the `Grid` with Gomoku rules.
#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid,
    to_take_stones: usize, // TODO remove this ?
    black_stones: StonesStats,
    white_stones: StonesStats,
}

/// Indicates the error of placement you get
/// when you missplace a stone on the `Board`.
#[derive(Debug)]
pub enum PlaceError {
    TileNotEmpty(Position),
    DoubleFreeThrees(Axes<bool>),
}

/// Indicates the victory condition under the one
/// you win by placing a stone on the `Board`.
#[derive(Debug)]
pub enum VictoryCondition {
    CapturedStones(usize, Directions<bool>), // TODO document this !!!
    FiveStonesAligned(Axes<Alignment>),
}

/// Gives information on the last successful stone placement on the `Board`.
#[derive(Debug)]
pub enum PlaceInfo {
    Nothing,
    Captures(Directions<bool>),
    FiveStonesAligned{ mandatory: Vec<Position> }, // TODO save state in Board struct
    Victory(VictoryCondition)
}

/// The type returned by the board when placing a stone.
pub type PlaceResult = Result<PlaceInfo, PlaceError>;

impl Board {
    /// Returns the default `Board`.
    pub fn new() -> Board {
        Board {
            grid: [[None; ::GRID_LEN]; ::GRID_LEN],
            to_take_stones: 10,
            black_stones: StonesStats { rest_stones: 20, stones_taken: 0 },
            white_stones: StonesStats { rest_stones: 20, stones_taken: 0 }
        }
    }

    /// create a board with a limited number of stones for players
    pub fn with_stone_limit(limit: usize) -> Board { // TODO remove this ?
        Board {
            grid: [[None; ::GRID_LEN]; ::GRID_LEN],
            to_take_stones: 10,
            black_stones: StonesStats { rest_stones: limit, stones_taken: 0 },
            white_stones: StonesStats { rest_stones: limit, stones_taken: 0 }
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
    fn stone_stats(&self, color: Color) -> &StonesStats {
        match color {
            Color::Black => &self.black_stones,
            Color::White => &self.white_stones,
        }
    }

    fn mut_stone_stats(&mut self, color: Color) -> &mut StonesStats {
        match color {
            Color::Black => &mut self.black_stones,
            Color::White => &mut self.white_stones,
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
        self.remove_captured_stones(pos, &captures);
        let nb_captures = captures.count(|x| *x == true);
        let mut stone_stats = self.mut_stone_stats(color);
        stone_stats.stones_taken += nb_captures;
        stone_stats.stones_taken
    }

    fn get_all_possible_captures(&self, color: Color) -> Vec<Position> {
        let mut captures = Vec::new();
        for x in 0..::GRID_LEN {
            for y in 0..::GRID_LEN {
                let pos = (x, y);
                for _ in get_captures(&self.grid, pos, color).iter().filter(|x| **x) {
                    let aligns = get_alignments(&self.grid, pos, color);
                    if get_free_threes(&self.grid, pos, color, &aligns).count(|x| *x) != 2 {
                        captures.push(pos);
                    }
                }
            }
        }
        captures
    }

    /// Try placing a stone on board, respecting rules
    pub fn try_place_stone(&mut self, color: Color, pos: Position) -> PlaceResult {
        let (x, y) = pos;
        if self.grid[x][y].is_some() {
            return Err(PlaceError::TileNotEmpty(pos))
        }
        let alignements = get_alignments(&self.grid, pos, color);
        let free_threes = get_free_threes(&self.grid, pos, color, &alignements);
        let captures = get_captures(&self.grid, pos, color);

        if free_threes.count(|x| *x == true) == 2 {
            Err(PlaceError::DoubleFreeThrees(free_threes))
        }
        else {
            let stones_taken = self.update_captures(pos, color, &captures);
            if alignements.any(|x| x.len() >= 5) {
                self.raw_place_stone(pos, color);
                if self.stone_stats(-color).stones_taken + 2 == self.to_take_stones {
                    Ok(PlaceInfo::FiveStonesAligned {
                        mandatory: self.get_all_possible_captures(-color)
                    })
                }
                else {
                    let mut captures = BTreeSet::new();
                    for (axis, alignment) in alignements.iter().enumerate()
                                                        .filter(|&(_, x)| x.len() >= 5) {
                        let capts = captures_on_axis(&self.grid, pos, -color, *alignment, axis);
                        captures = captures.bitand(&capts);
                    }
                    if !captures.is_empty() {
                        Ok(PlaceInfo::FiveStonesAligned {
                            mandatory: captures.iter().cloned().collect()
                        })
                    }
                    else {
                        Ok(PlaceInfo::Victory(VictoryCondition::FiveStonesAligned(alignements)))
                    }
                }
            }
            else if stones_taken >= self.to_take_stones {
                self.raw_place_stone(pos, color);
                Ok(PlaceInfo::Victory(VictoryCondition::CapturedStones(stones_taken, captures)))
            }
            else {
                Ok(PlaceInfo::Nothing)
            }
        }
    }
}

use std::default::Default;
use color::Color;
use tile::Tile;
use ::{Axis, Grid};

#[derive(Debug, Clone)]
struct StonesStats {
    rest_stones: usize,
    taken_stones: usize
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid,
    to_take_stones: usize, // TODO remove this ?
    black_stones: StonesStats,
    white_stones: StonesStats,
}

#[derive(Debug)]
pub enum PlacementError {
    AnotherStoneAtEmplacement,
    DoubleFreeTree,
}

#[derive(Debug)]
pub enum PlacementInfo {
    Nothing,
    Variant2,
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[None; ::GRID_LEN]; ::GRID_LEN],
            to_take_stones: 10,
            black_stones: StonesStats { rest_stones: 20, taken_stones: 0 },
            white_stones: StonesStats { rest_stones: 20, taken_stones: 0 }
        }
    }

    /// create a board with a limited number of stones for players
    pub fn with_stone_limit(limit: usize) -> Board {
        Board {
            grid: [[None; ::GRID_LEN]; ::GRID_LEN],
            to_take_stones: 10,
            black_stones: StonesStats { rest_stones: limit, taken_stones: 0 },
            white_stones: StonesStats { rest_stones: limit, taken_stones: 0 }
        }
    }

    /// put a stone without checks of double-free-trees, alignements of stones
    /// and will not remove other stones
    pub fn raw_place_stone(&mut self, color: Color, pos: Axis) {
        self.grid[pos.x][pos.y] = Some(color)
    }

    /// put a stone and launch the game rules, check for alignements of stones,
    /// return errors if a stone is already present...
    pub fn place_stone(&mut self, color: Color, pos: Axis)
                       -> Result<PlacementInfo, PlacementError> {

        match self.grid[pos.x][pos.y] {
            None => {
                self.grid[pos.x][pos.y] = Some(color);
                // check alignements, validty...
                Ok(PlacementInfo::Nothing) // return check informations
            },
            _ => Err(PlacementError::AnotherStoneAtEmplacement)
        }
    }
}

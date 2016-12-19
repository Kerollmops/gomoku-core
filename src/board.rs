use ::{Position, Color, Grid, Axes, Directions, Alignment};
use ::{get_alignments, get_free_threes, get_captures};
use ::directions::*;

#[derive(Debug, Clone)]
struct StonesStats {
    rest_stones: usize,
    stones_taken: usize
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid,
    to_take_stones: usize, // TODO remove this ?
    black_stones: StonesStats,
    white_stones: StonesStats,
}

#[derive(Debug)]
pub enum PlaceError {
    TileNotEmpty(Position),
    DoubleFreeThrees(Axes<bool>),
}

#[derive(Debug)]
pub enum VictoryCondition {
    CapturedStones(usize, Directions<bool>), // TODO clear this ?
    FiveStonesAligned(Axes<Alignment>),
}

#[derive(Debug)]
pub enum PlaceInfo {
    Nothing,
    Captures(Directions<bool>),
    FiveStonesAligned, // TODO add capturable stones
    Victory(VictoryCondition)
}

/// The type returned by the board when placing a stone
pub type PlaceResult = Result<PlaceInfo, PlaceError>;

impl Board {
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
        match color {
            Color::Black => {
                self.black_stones.stones_taken += nb_captures;
                self.white_stones.stones_taken
            },
            Color::White => {
                self.white_stones.stones_taken += nb_captures;
                self.white_stones.stones_taken
            },
        }
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

                // Check if -color can win with the last capture
                // force him to play this or break color alignment

                // Check if an alignement of five stone is not blocked
                // by captures, allow victory in this case

                // if can_be_took(alignement_of_five_or_more) {
                //     Ok(PlaceInfo::FiveStonesAligned)
                // }
                // else {

                // TODO if opponent can capture don't win !

                self.raw_place_stone(pos, color);
                Ok(PlaceInfo::Victory(VictoryCondition::FiveStonesAligned(alignements)))
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

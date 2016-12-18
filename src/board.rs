use ::{Position, Color, Grid, Axes, Directions, Alignment};
use ::{list_alignments, list_free_threes, list_captures};

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
            black_stones: StonesStats { rest_stones: 20, taken_stones: 0 },
            white_stones: StonesStats { rest_stones: 20, taken_stones: 0 }
        }
    }

    /// create a board with a limited number of stones for players
    pub fn with_stone_limit(limit: usize) -> Board { // TODO remove this ?
        Board {
            grid: [[None; ::GRID_LEN]; ::GRID_LEN],
            to_take_stones: 10,
            black_stones: StonesStats { rest_stones: limit, taken_stones: 0 },
            white_stones: StonesStats { rest_stones: limit, taken_stones: 0 }
        }
    }

    /// Simply puts a stone on the board
    pub fn raw_place_stone(&mut self, color: Color, (x, y): Position) {
        self.grid[x][y] = Some(color)
    }

    /// Simply removes a stone from the board
    pub fn raw_remove_stone(&mut self, (x, y): Position) {
        self.grid[x][y] = None;
    }

    fn increase_captures(&mut self, color: Color, nb_captures: usize) -> usize {
        match color {
            Color::Black => {
                self.black_stones.taken_stones += nb_captures;
                self.white_stones.taken_stones
            },
            Color::White => {
                self.white_stones.taken_stones += nb_captures;
                self.white_stones.taken_stones
            },
        }
    }

    fn remove_captured_stones(&mut self, _captures: &Directions<bool>) {
        //
    }

    /// Try placing a stone on board, respecting rules,
    /// Return an error if a stone is already present...
    pub fn try_place_stone(&mut self, color: Color, pos: Position) -> PlaceResult {
        let (x, y) = pos;
        if self.grid[x][y].is_some() {
            return Err(PlaceError::TileNotEmpty(pos))
        }

        self.raw_place_stone(color, pos);

        let alignements = list_alignments(&self.grid, pos);
        let free_threes = list_free_threes(&self.grid, pos, &alignements);
        let captures = list_captures(&self.grid, pos);

        if free_threes.count(|x| *x == true) == 2 {
            self.raw_remove_stone(pos);
            Err(PlaceError::DoubleFreeThrees(free_threes))
        }
        else {
            self.remove_captured_stones(&captures);
            if alignements.any(|x| x.len() >= 5) {

                // Check if an alignement of five stone is not blocked
                // by captures, allow victory in this case

                // if can_be_took(alignement_of_five_or_more) {
                //     Ok(PlaceInfo::FiveStonesAligned)
                // }
                // else {

                use self::VictoryCondition::*;
                Ok(PlaceInfo::Victory(FiveStonesAligned(alignements)))
            }
            else {
                let nb_captures = captures.count(|x| *x == true);
                let taken_stones = self.increase_captures(color, nb_captures);
                if taken_stones >= self.to_take_stones {
                    use self::VictoryCondition::*;
                    Ok(PlaceInfo::Victory(CapturedStones(taken_stones, captures)))
                }
                else {
                    Ok(PlaceInfo::Nothing)
                }
            }
        }
    }
}

use color::Color;
use tile::Tile;
use ::Axis;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoundState { // TODO change name
    OutOfBound,
    Tile(Tile),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Alignement(pub BoundState, pub usize, pub BoundState);

pub const HORIZONTAL: usize = 0;
pub const DIAGONAL_UP: usize  = 1;
pub const VERTICAL: usize  = 2;
pub const DIAGONAL_DOWN: usize  = 3;

// TODO make types for clarity (grid, return)
pub fn horizontal_alignement(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> Alignement {
    let tile = grid[pos.x][pos.y].expect(&format!("Tile at {:?} is empty!", pos));
    let mut alignement = Alignement(BoundState::OutOfBound, 0, BoundState::OutOfBound);
    for y in (0...pos.y).rev() {
        match grid[pos.x][y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.0 = BoundState::Tile(tile); break },
        }
    }
    for y in pos.y + 1..::GRID_LEN {
        match grid[pos.x][y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.2 = BoundState::Tile(tile); break },
        }
    }
    alignement
}

pub fn diagonal_up_alignement(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> Alignement {
    let tile = grid[pos.x][pos.y].expect(&format!("Tile at {:?} is empty!", pos));
    let mut alignement = Alignement(BoundState::OutOfBound, 0, BoundState::OutOfBound);
    let Axis { mut x, mut y } = pos;
    while x < ::GRID_LEN && y < ::GRID_LEN { // x will underflow to usize::max()
        match grid[x][y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.0 = BoundState::Tile(tile); break },
        }
        x += 1;
        y = y.wrapping_sub(1);
    }
    let Axis { mut x, mut y } = pos;
    x = x.wrapping_sub(1);
    y += 1;
    while x < ::GRID_LEN && y < ::GRID_LEN {
        match grid[x][y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.2 = BoundState::Tile(tile); break },
        }
        x = x.wrapping_sub(1);
        y += 1;
    }
    alignement
}

pub fn vertical_alignement(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> Alignement {
    let tile = grid[pos.x][pos.y].expect(&format!("Tile at {:?} is empty!", pos));
    let mut alignement = Alignement(BoundState::OutOfBound, 0, BoundState::OutOfBound);
    for x in (0...pos.x).rev() {
        match grid[x][pos.y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.0 = BoundState::Tile(tile); break },
        }
    }
    for x in pos.x + 1..::GRID_LEN {
        match grid[x][pos.y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.2 = BoundState::Tile(tile); break },
        }
    }
    alignement
}

pub fn diagonal_down_alignement(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> Alignement {
    let tile = grid[pos.x][pos.y].expect(&format!("Tile at {:?} is empty!", pos));
    let mut alignement = Alignement(BoundState::OutOfBound, 0, BoundState::OutOfBound);
    let Axis { mut x, mut y } = pos;
    while x < ::GRID_LEN && y < ::GRID_LEN { // x and y will overflow to usize::max()
        match grid[x][y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.0 = BoundState::Tile(tile); break },
        }
        x = x.wrapping_sub(1);
        y = y.wrapping_sub(1);
    }
    let Axis { mut x, mut y } = pos;
    x += 1;
    y += 1;
    while x < ::GRID_LEN && y < ::GRID_LEN {
        match grid[x][y] {
            Some(c) if c == tile => alignement.1 += 1,
            tile => { alignement.2 = BoundState::Tile(tile); break },
        }
        x += 1;
        y += 1;
    }
    alignement
}

/// returns a list of alignements with the tile at `pos` position in Clockwise
/// (e.g. top_to_bot, top_right_to_bot_left, right_to_left, bot_right_to_top_left)
/// a None value means no alignement (e.g. less than 2 stones)
pub fn list_alignements(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> [Option<Alignement>; 4] {
    let mut alignements = [None; 4];
    alignements[HORIZONTAL] = match horizontal_alignement(grid, pos) {
        Alignement(_, 0, _) => unreachable!("horizontal_alignement cannot count zero tiles!"),
        Alignement(_, 1, _) => None,
        x => Some(x),
    };
    alignements[DIAGONAL_UP] = match diagonal_up_alignement(grid, pos) {
        Alignement(_, 0, _) => unreachable!("diagonal_up_alignement cannot count zero tiles!"),
        Alignement(_, 1, _) => None,
        x => Some(x),
    };
    alignements[VERTICAL] = match vertical_alignement(grid, pos) {
        Alignement(_, 0, _) => unreachable!("vertical_alignement cannot count zero tiles!"),
        Alignement(_, 1, _) => None,
        x => Some(x),
    };
    alignements[DIAGONAL_DOWN] = match diagonal_down_alignement(grid, pos) {
        Alignement(_, 0, _) => unreachable!("diagonal_down_alignement cannot count zero tiles!"),
        Alignement(_, 1, _) => None,
        x => Some(x),
    };
    alignements
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use functions::alignements::*;
    use ::Axis;
    use color::Color;

    #[bench]
    fn alignements_horizontal_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[b, b, b, b, b, b, b, b, b, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::OutOfBound, 9, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(horizontal_alignement(&grid, Axis { x: 0, y: 0 }), alignement)
        );
    }

    #[bench]
    fn alignements_horizontal_backward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[b, b, b, b, b, b, n, b, b, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::OutOfBound, 6, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(horizontal_alignement(&grid, Axis { x: 0, y: 5 }), alignement)
        );
    }

    #[bench]
    fn alignements_horizontal_backward_and_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let n = None;

        let grid = [[b, b, b, b, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::OutOfBound, 6, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(horizontal_alignement(&grid, Axis { x: 0, y: 3 }), alignement)
        );
    }

    #[bench]
    fn alignements_diagonal_up_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::Tile(w), 6, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(diagonal_up_alignement(&grid, Axis { x: 8, y: 2 }), alignement)
        );
    }

    #[bench]
    fn alignements_diagonal_up_backward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::Tile(w), 6, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(diagonal_up_alignement(&grid, Axis { x: 3, y: 7 }), alignement)
        );
    }

    #[bench]
    fn alignements_diagonal_up_backward_and_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, w, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::Tile(n), 6, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(diagonal_up_alignement(&grid, Axis { x: 5, y: 5 }), alignement)
        );
    }

    #[bench]
    fn alignements_vertical_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::Tile(w), 6, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(vertical_alignement(&grid, Axis { x: 3, y: 4 }), alignement)
        );
    }

    #[bench]
    fn alignements_vertical_backward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::Tile(w), 6, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(vertical_alignement(&grid, Axis { x: 8, y: 4 }), alignement)
        );
    }

    #[bench]
    fn alignements_vertical_backward_and_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let alignement = Alignement(BoundState::Tile(w), 6, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(vertical_alignement(&grid, Axis { x: 6, y: 4 }), alignement)
        );
    }

    #[bench]
    fn alignements_diagonal_down_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
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

        let alignement = Alignement(BoundState::Tile(w), 6, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(diagonal_down_alignement(&grid, Axis { x: 2, y: 2 }), alignement)
        );
    }

    #[bench]
    fn alignements_diagonal_down_backward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, w, n, n, n, n, n, n, n, n, n, n],
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

        let alignement = Alignement(BoundState::Tile(n), 6, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(diagonal_down_alignement(&grid, Axis { x: 7, y: 7 }), alignement)
        );
    }

    #[bench]
    fn alignements_diagonal_down_backward_and_forward(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, n, n, n, n, n, w, n, n, n, n, n, n, n, n, n, n],
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

        let alignement = Alignement(BoundState::Tile(n), 6, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(diagonal_down_alignement(&grid, Axis { x: 5, y: 5 }), alignement)
        );
    }

    #[bench]
    fn alignements_list_all(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n],
                    [n, w, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, b, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, b, b, b, w, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut alignements = [None; 4];

        alignements[HORIZONTAL] = Some(Alignement(BoundState::Tile(n), 4, BoundState::Tile(w)));
        alignements[DIAGONAL_UP] = Some(Alignement(BoundState::Tile(n), 7, BoundState::OutOfBound));
        alignements[VERTICAL] = Some(Alignement(BoundState::Tile(n), 3, BoundState::Tile(n)));
        alignements[DIAGONAL_DOWN] = Some(Alignement(BoundState::Tile(w), 4, BoundState::Tile(n)));

        bencher.iter(||
            assert_eq!(list_alignements(&grid, Axis { x: 4, y: 4 }), alignements)
        );
    }

    #[bench]
    fn alignements_list_all_too_small(bencher: &mut Bencher) {
        let b = Some(Color::Black);
        let w = Some(Color::White);
        let n = None;

        let grid = [[n, n, n, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n],
                    [n, w, n, n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, b, n, b, n, b, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, b, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, w, b, w, n, n, n, n, n, n, n, n, n, n, n, n, n],
                    [n, n, n, b, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
                    [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

        let mut alignements = [None; 4];

        alignements[HORIZONTAL] = None;
        alignements[DIAGONAL_UP] = Some(Alignement(BoundState::Tile(n), 7, BoundState::OutOfBound));
        alignements[VERTICAL] = Some(Alignement(BoundState::Tile(n), 3, BoundState::Tile(n)));
        alignements[DIAGONAL_DOWN] = Some(Alignement(BoundState::Tile(w), 4, BoundState::Tile(n)));

        bencher.iter(||
            assert_eq!(list_alignements(&grid, Axis { x: 4, y: 4 }), alignements)
        );
    }
}

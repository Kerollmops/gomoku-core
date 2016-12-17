use std::default::Default;
use ::{ Position, Tile, Grid, Axes };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoundState {
    OutOfBound,
    Tile(Tile),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Alignment(pub BoundState, pub usize, pub usize, pub BoundState);

impl Alignment {
    pub fn len(&self) -> usize {
        let &Alignment(_, backward, forward, _) = self;
        backward + forward + 1
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment(BoundState::OutOfBound, 0, 0, BoundState::OutOfBound)
    }
}

// TODO make types for clarity (grid, return)
fn horizontal_alignment(grid: &Grid, (x, y): Position) -> Alignment {
    let tile = grid[x][y].expect("Tile is empty!");
    let mut alignment = Alignment::default();
    for y in (0...y).rev() {
        match grid[x][y] {
            Some(c) if c == tile => alignment.1 += 1,
            tile => { alignment.0 = BoundState::Tile(tile); break },
        }
    }
    alignment.1 -= 1;
    for y in y + 1..::GRID_LEN {
        match grid[x][y] {
            Some(c) if c == tile => alignment.2 += 1,
            tile => { alignment.3 = BoundState::Tile(tile); break },
        }
    }
    alignment
}

fn diagonal_up_alignment(grid: &Grid, pos: Position) -> Alignment {
    let (mut x, mut y) = pos;
    let tile = grid[x][y].expect("Tile is empty!");
    let mut alignment = Alignment::default();
    while x < ::GRID_LEN && y < ::GRID_LEN { // x will underflow to usize::max()
        match grid[x][y] {
            Some(c) if c == tile => alignment.1 += 1,
            tile => { alignment.0 = BoundState::Tile(tile); break },
        }
        x += 1;
        y = y.wrapping_sub(1);
    }
    alignment.1 -= 1;
    let (mut x, mut y) = pos;
    x = x.wrapping_sub(1);
    y += 1;
    while x < ::GRID_LEN && y < ::GRID_LEN {
        match grid[x][y] {
            Some(c) if c == tile => alignment.2 += 1,
            tile => { alignment.3 = BoundState::Tile(tile); break },
        }
        x = x.wrapping_sub(1);
        y += 1;
    }
    alignment
}

fn vertical_alignment(grid: &Grid, (x, y): Position) -> Alignment {
    let tile = grid[x][y].expect("Tile is empty!");
    let mut alignment = Alignment::default();
    for x in (0...x).rev() {
        match grid[x][y] {
            Some(c) if c == tile => alignment.1 += 1,
            tile => { alignment.0 = BoundState::Tile(tile); break },
        }
    }
    alignment.1 -= 1;
    for x in x + 1..::GRID_LEN {
        match grid[x][y] {
            Some(c) if c == tile => alignment.2 += 1,
            tile => { alignment.3 = BoundState::Tile(tile); break },
        }
    }
    alignment
}

fn diagonal_down_alignment(grid: &Grid, pos: Position) -> Alignment {
    let (mut x, mut y) = pos;
    let tile = grid[x][y].expect("Tile is empty!");
    let mut alignment = Alignment::default();
    while x < ::GRID_LEN && y < ::GRID_LEN { // x and y will overflow to usize::max()
        match grid[x][y] {
            Some(c) if c == tile => alignment.1 += 1,
            tile => { alignment.0 = BoundState::Tile(tile); break },
        }
        x = x.wrapping_sub(1);
        y = y.wrapping_sub(1);
    }
    alignment.1 -= 1;
    let (mut x, mut y) = pos;
    x += 1;
    y += 1;
    while x < ::GRID_LEN && y < ::GRID_LEN {
        match grid[x][y] {
            Some(c) if c == tile => alignment.2 += 1,
            tile => { alignment.3 = BoundState::Tile(tile); break },
        }
        x += 1;
        y += 1;
    }
    alignment
}

/// returns a list of alignments with the tile at `pos` position Clockwise
/// (e.g. top_to_bot, top_right_to_bot_left, right_to_left, bot_right_to_top_left)
/// a None value means no alignment (e.g. less than 2 stones)
pub fn list_alignments(grid: &Grid, pos: Position) -> Axes<Alignment> {
    let hori = horizontal_alignment(grid, pos);
    let diag_up = diagonal_up_alignment(grid, pos);
    let vert = vertical_alignment(grid, pos);
    let diag_down = diagonal_down_alignment(grid, pos);
    Axes::new(hori, diag_up, vert, diag_down)
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use ::alignments::*;
    use ::Axes;
    use color::Color;

    #[bench]
    fn alignments_horizontal_forward(bencher: &mut Bencher) {
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

        let expected_align = Alignment(BoundState::OutOfBound, 0, 8, BoundState::Tile(n));

        bencher.iter(|| {
            let alignment = horizontal_alignment(&grid, (0, 0));
            assert_eq!(alignment, expected_align);
            assert_eq!(alignment.len(), 9);
        });
    }

    #[bench]
    fn alignments_horizontal_backward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::OutOfBound, 5, 0, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(horizontal_alignment(&grid, (0, 5)), alignment)
        );
    }

    #[bench]
    fn alignments_horizontal_backward_and_forward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::OutOfBound, 3, 2, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(horizontal_alignment(&grid, (0, 3)), alignment)
        );
    }

    #[bench]
    fn alignments_diagonal_up_forward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(w), 0, 5, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(diagonal_up_alignment(&grid, (8, 2)), alignment)
        );
    }

    #[bench]
    fn alignments_diagonal_up_backward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(w), 5, 0, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(diagonal_up_alignment(&grid, (3, 7)), alignment)
        );
    }

    #[bench]
    fn alignments_diagonal_up_backward_and_forward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(n), 3, 2, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(diagonal_up_alignment(&grid, (5, 5)), alignment)
        );
    }

    #[bench]
    fn alignments_vertical_forward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(w), 0, 5, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(vertical_alignment(&grid, (3, 4)), alignment)
        );
    }

    #[bench]
    fn alignments_vertical_backward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(w), 5, 0, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(vertical_alignment(&grid, (8, 4)), alignment)
        );
    }

    #[bench]
    fn alignments_vertical_backward_and_forward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(w), 3, 2, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(vertical_alignment(&grid, (6, 4)), alignment)
        );
    }

    #[bench]
    fn alignments_diagonal_down_forward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(w), 0, 5, BoundState::Tile(n));

        bencher.iter(||
            assert_eq!(diagonal_down_alignment(&grid, (2, 2)), alignment)
        );
    }

    #[bench]
    fn alignments_diagonal_down_backward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(n), 5, 0, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(diagonal_down_alignment(&grid, (7, 7)), alignment)
        );
    }

    #[bench]
    fn alignments_diagonal_down_backward_and_forward(bencher: &mut Bencher) {
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

        let alignment = Alignment(BoundState::Tile(n), 3, 2, BoundState::Tile(w));

        bencher.iter(||
            assert_eq!(diagonal_down_alignment(&grid, (5, 5)), alignment)
        );
    }

    #[bench]
    fn alignments_list_all(bencher: &mut Bencher) {
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

        let alignments = Axes::new(
            Alignment(BoundState::Tile(n), 2, 1, BoundState::Tile(w)),
            Alignment(BoundState::Tile(n), 2, 4, BoundState::OutOfBound),
            Alignment(BoundState::Tile(n), 2, 0, BoundState::Tile(n)),
            Alignment(BoundState::Tile(w), 2, 1, BoundState::Tile(n))
        );

        bencher.iter(||
            assert_eq!(list_alignments(&grid, (4, 4)), alignments)
        );
    }

    #[bench]
    fn alignments_list_all_too_small(bencher: &mut Bencher) {
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

        let alignments = Axes::new(
            Alignment(BoundState::Tile(w), 0, 0, BoundState::Tile(w)),
            Alignment(BoundState::Tile(n), 2, 4, BoundState::OutOfBound),
            Alignment(BoundState::Tile(n), 2, 0, BoundState::Tile(n)),
            Alignment(BoundState::Tile(w), 2, 1, BoundState::Tile(n))
        );

        bencher.iter(||
            assert_eq!(list_alignments(&grid, (4, 4)), alignments)
        );
    }
}

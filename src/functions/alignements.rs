use color::Color;
use tile::Tile;
use ::Axis;

pub const ALIGN_HORIZONTAL: usize = 0;
pub const ALIGN_DIAGONAL_UP: usize  = 1;
pub const ALIGN_VERTICAL: usize  = 2;
pub const ALIGN_DIAGONAL_DOWN: usize  = 3;

pub fn alignement_horizontal(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> usize {
    let tile = grid[pos.x][pos.y].unwrap();
    let mut count = 0;
    for y in (pos.y...0).rev() {
        match grid[pos.x][y] {
            Some(c) if c == tile => count += 1,
            _ => break,
        }
    }
    for y in pos.y + 1..::GRID_LEN {
        match grid[pos.x][y] {
            Some(c) if c == tile => count += 1,
            _ => break,
        }
    }
    count
}

pub fn alignement_diagonal_up(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> usize {
    let tile = grid[pos.x][pos.y].unwrap();
    let mut count = 0;
    {
        let Axis { mut x, mut y } = pos;
        while x < ::GRID_LEN && y < ::GRID_LEN { // y will overflow to usize::max()
            match grid[x][y] {
                Some(c) if c == tile => count += 1,
                _ => break,
            }
            x += 1;
            y = y.wrapping_sub(1);
        }
    }
    {
        let Axis { mut x, mut y } = pos;
        x = x.wrapping_sub(1);
        y += 1;
        while x < ::GRID_LEN && y < ::GRID_LEN {
            match grid[x][y] {
                Some(c) if c == tile => count += 1,
                _ => break,
            }
            x = x.wrapping_sub(1);
            y += 1;
        }
    }
    count
}

pub fn alignement_vertical(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> usize {
    let tile = grid[pos.x][pos.y].unwrap();
    let mut count = 0;
    for x in (pos.x...0).rev() {
        match grid[x][pos.y] {
            Some(c) if c == tile => count += 1,
            _ => break,
        }
    }
    for x in pos.x + 1..::GRID_LEN {
        match grid[x][pos.y] {
            Some(c) if c == tile => count += 1,
            _ => break,
        }
    }
    count
}

pub fn alignement_diagonal_down(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> usize {
    let tile = grid[pos.x][pos.y].unwrap();
    let mut count = 0;
    {
        let Axis { mut x, mut y } = pos;
        while x < ::GRID_LEN && y < ::GRID_LEN { // x and y will overflow to usize::max()
            match grid[x][y] {
                Some(c) if c == tile => count += 1,
                _ => break,
            }
            x = x.wrapping_sub(1);
            y = y.wrapping_sub(1);
        }
    }
    {
        let Axis { mut x, mut y } = pos;
        x += 1;
        y += 1;
        while x < ::GRID_LEN && y < ::GRID_LEN {
            match grid[x][y] {
                Some(c) if c == tile => count += 1,
                _ => break,
            }
            x += 1;
            y += 1;
        }
    }
    count
}

/// returns a list of alignements with the tile at `pos` position in Clockwise
/// (e.g. top_to_bot, top_right_to_bot_left, right_to_left, bot_right_to_top_left)
/// a None value means no alignement (e.g. less than 2 stones)
pub fn list_alignements(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> [Option<usize>; 4] {
    let mut alignements = [None; 4];
    alignements[ALIGN_HORIZONTAL] = match alignement_horizontal(grid, pos) {
        0 => unreachable!("alignement_horizontal cannot count zero tiles!"),
        1 => None,
        x => Some(x),
    };
    alignements[ALIGN_DIAGONAL_UP] = match alignement_diagonal_up(grid, pos) {
        0 => unreachable!("alignement_diagonal_up cannot count zero tiles!"),
        1 => None,
        x => Some(x),
    };
    alignements[ALIGN_VERTICAL] = match alignement_vertical(grid, pos) {
        0 => unreachable!("alignement_vertical cannot count zero tiles!"),
        1 => None,
        x => Some(x),
    };
    alignements[ALIGN_DIAGONAL_DOWN] = match alignement_diagonal_down(grid, pos) {
        0 => unreachable!("alignement_diagonal_down cannot count zero tiles!"),
        1 => None,
        x => Some(x),
    };
    alignements
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use functions::alignements::*;
    use color::Color;

    // #[bench]
    // fn diagonal_up_top_bounds(bencher: &mut Bencher) {
    //     let b = Some(Color::Black);
    //     let n = None;

    //     let grid = [[n, n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

    //     bencher.iter(||
    //         assert_eq!(check_diagonal_up_alignement(&grid, Color::Black), true)
    //     );
    // }
}

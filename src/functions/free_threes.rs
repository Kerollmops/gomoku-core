use color::Color;
use tile::Tile;
use ::Axis;
use functions::alignements::{list_alignements, Alignement, BoundState};

pub fn list_free_threes(grid: &[[Tile; ::GRID_LEN]; ::GRID_LEN], pos: Axis) -> [bool; 8] {
    let mut free_threes = [false; 8];
    for (i, x) in list_alignements(grid, pos).iter().enumerate() {
        free_threes[i] = match *x {
            Some(Alignement(BoundState::Tile(None), _, BoundState::Tile(None))) => true,
            _ => false,
        };
    }
    free_threes
}

#[cfg(test)]
mod tests {

    use test::Bencher;
    use functions::free_threes::*;
    use color::Color;

    // #[bench]
    // fn captures_top(bencher: &mut Bencher) {
    //     let b = Some(Color::Black);
    //     let w = Some(Color::White);
    //     let n = None;

    //     let grid = [[n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, b, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
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
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n],
    //                 [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n]];

    //     bencher.iter(|| {
    //         let mut captures = [false; 8];
    //         captures[TOP] = true;

    //         assert_eq!(list_captures(&grid, Axis { x: 3, y: 3 }), captures)
    //     });
    // }
}

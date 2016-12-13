use color::Color;

fn check_diagonal_up_alignement(grid: &[&[Option<Color>]]) {
    for x in (0..grid.len() - 4).rev() {
        unimplemented!()
    }
}

fn check_diagonal_down_alignement(grid: &[&[Option<Color>]], color: Color) -> bool {
    for x in 0..grid.len() - 4 {
        let mut x = x;
        let mut y = 0;
        let mut count = 0;
        while x < grid.len() && y < grid[0].len() {
            match grid[x][y] {
                Some(c) if c == color => {
                    count += 1;
                    if count == 5 { return true }
                },
                _ => count = 0,
            }
            x += 1;
            y += 1;
        }
    }
    for y in 0..grid[0].len() - 4 { // TODO create function to test this
        let mut x = 0;
        let mut y = y;
        let mut count = 0;
        while x < grid.len() && y < grid[0].len() {
            match grid[x][y] {
                Some(c) if c == color => {
                    count += 1;
                    if count == 5 { return true }
                },
                _ => count = 0,
            }
            x += 1;
            y += 1;
        }
    }
    false
}

fn check_horizontal_alignement(grid: &[&[Option<Color>]], color: Color) -> bool {
    for x in 0..grid.len() {
        let mut count = 0;
        for y in 0..grid[0].len() - 4 {
            match grid[x][y] {
                Some(c) if c == color => {
                    count += 1;
                    if count == 5 { return true }
                },
                _ => count = 0,
            }
        }
    }
    false
}

fn check_vertical_alignement(grid: &[&[Option<Color>]], color: Color) -> bool {
    for y in 0..grid.len() {
        let mut count = 0;
        for x in 0..grid[0].len() - 4 {
            match grid[x][y] {
                Some(c) if c == color => {
                    count += 1;
                    if count == 5 { return true }
                },
                _ => count = 0,
            }
        }
    }
    false
}

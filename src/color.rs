use std::ops::Neg;

/// Represent stone possible colors (Black and White).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

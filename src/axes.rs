use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

pub const HORIZONTAL: usize = 0;
pub const DIAGONAL_UP: usize  = 1;
pub const VERTICAL: usize  = 2;
pub const DIAGONAL_DOWN: usize  = 3;

/// Represent all the 4 axes
/// (`horizontal`, `diagonal_up`, `vertical`, `diagonal_down`)
pub struct Axes<T>([T; 4]);

impl<T> Axes<T> {
    pub fn new(hori: T, diag_up: T, vert: T, diag_down: T) -> Axes<T> {
        Axes([hori, diag_up, vert, diag_down])
    }

    pub fn horizontal(&self) -> &T { &self.0[HORIZONTAL] }

    pub fn diagonal_up(&self) -> &T { &self.0[DIAGONAL_UP] }

    pub fn vertical(&self) -> &T { &self.0[VERTICAL] }

    pub fn diagonal_down(&self) -> &T { &self.0[DIAGONAL_DOWN] }
}

impl<T: Debug> Debug for Axes<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Axes({:?})", self.0)
    }
}

impl<T: PartialEq> PartialEq for Axes<T> {
    fn eq(&self, other: &Axes<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> From<[T; 4]> for Axes<T> {
    fn from(array: [T; 4]) -> Self {
        Axes(array)
    }
}

impl<T> Deref for Axes<T> {
    type Target = [T; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Axes<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

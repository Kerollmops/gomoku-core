use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

/// Type used to represent an axis.
pub type Axis = usize;

/// Horizontal axis.
pub const HORIZONTAL: Axis = 0;
/// Diagonal up axis.
pub const DIAGONAL_UP: Axis  = 1;
/// Vertical axis.
pub const VERTICAL: Axis  = 2;
/// Diagonal down axis.
pub const DIAGONAL_DOWN: Axis  = 3;

/// Represent all the 4 axes.
pub struct Axes<T>([T; 4]);

impl<T> Axes<T> {
    pub fn new(hori: T, diag_up: T, vert: T, diag_down: T) -> Axes<T> {
        Axes([hori, diag_up, vert, diag_down])
    }

    pub fn horizontal(&self) -> &T { &self.0[HORIZONTAL] }

    pub fn diagonal_up(&self) -> &T { &self.0[DIAGONAL_UP] }

    pub fn vertical(&self) -> &T { &self.0[VERTICAL] }

    pub fn diagonal_down(&self) -> &T { &self.0[DIAGONAL_DOWN] }

    pub fn count<P>(&self, mut predicate: P) -> usize where P: FnMut(&T) -> bool {
        let mut count = 0;
        for x in &self.0 {
            if predicate(x) { count += 1; }
        }
        count
    }

    pub fn any<P>(&self, predicate: P) -> bool where P: FnMut(&T) -> bool {
        self.0.iter().any(predicate)
    }
}

impl<T: Default> Axes<T> {
    pub fn new_horizontal(axis: T) -> Axes<T> {
        Axes([axis, T::default(), T::default(), T::default()])
    }

    pub fn new_diagonal_up(axis: T) -> Axes<T> {
        Axes([T::default(), axis, T::default(), T::default()])
    }

    pub fn new_vertical(axis: T) -> Axes<T> {
        Axes([T::default(), T::default(), axis, T::default()])
    }

    pub fn new_diagonal_down(axis: T) -> Axes<T> {
        Axes([T::default(), T::default(), T::default(), axis])
    }
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

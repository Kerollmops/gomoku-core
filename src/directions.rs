use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

use std::ops::FnMut;

/// Top direction.
pub const TOP: usize = 0;
/// Top combined with right direction.
pub const TOP_RIGHT: usize = 1;
/// Right direction.
pub const RIGHT: usize = 2;
/// Bottom combined with right direction.
pub const BOT_RIGHT: usize = 3;
/// Bottom direction.
pub const BOT: usize = 4;
/// Bottom combined with left direction.
pub const BOT_LEFT: usize = 5;
/// Left direction.
pub const LEFT: usize = 6;
/// Top combined with left direction.
pub const TOP_LEFT: usize = 7;

/// Represent all the 8 directions.
/// The only way to create a Direction is from a `[T; 8]`.
pub struct Directions<T>([T; 8]);

impl<T> Directions<T> {
    pub fn top(&self) -> &T { &self.0[TOP] }

    pub fn top_right(&self) -> &T { &self.0[TOP_RIGHT] }

    pub fn right(&self) -> &T { &self.0[RIGHT] }

    pub fn bot_right(&self) -> &T { &self.0[BOT_RIGHT] }

    pub fn bot(&self) -> &T { &self.0[BOT] }

    pub fn bot_left(&self) -> &T { &self.0[BOT_LEFT] }

    pub fn left(&self) -> &T { &self.0[LEFT] }

    pub fn top_left(&self) -> &T { &self.0[TOP_LEFT] }

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

impl<T: Debug> Debug for Directions<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Directions({:?})", self.0)
    }
}

impl<T: PartialEq> PartialEq for Directions<T> {
    fn eq(&self, other: &Directions<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> From<[T; 8]> for Directions<T> {
    fn from(array: [T; 8]) -> Self {
        Directions(array)
    }
}

impl<T> Deref for Directions<T> {
    type Target = [T; 8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Directions<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use std::cmp::PartialEq;
use std::convert::From;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

pub const TOP: usize = 0;
pub const TOP_RIGHT: usize = 1;
pub const RIGHT: usize = 2;
pub const BOT_RIGHT: usize = 3;
pub const BOT: usize = 4;
pub const BOT_LEFT: usize = 5;
pub const LEFT: usize = 6;
pub const TOP_LEFT: usize = 7;

/// Represent all the 8 possible directions
/// (top, top_right, right, bot_right, bot, bot_left, left, top_left)
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

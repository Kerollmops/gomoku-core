mod alignments;
mod free_threes;
mod captures;
/// Returns possible positions for captures for all `Alignments`.
pub mod captures_on_alignement;

pub use self::alignments::*;
pub use self::free_threes::*;
pub use self::captures::*;

mod bitboard;
mod square;
mod filerank;

pub mod prelude {
    pub use crate::filerank::*;
    pub use crate::bitboard::Bitboard;
    pub use crate::square::Square;
}

// If we want to use PEXT instructions
// Sometimes we don't even if it's available because it
// can be slow
#[cfg(all(feature = "pext", any(target_arch = "x86", target_arch = "x86_64")))]
pub const HAS_PEXT: bool = true;
#[cfg(any(not(feature = "pext"), not(any(target_arch = "x86", target_arch = "x86_64"))))]
pub const HAS_PEXT: bool = false;

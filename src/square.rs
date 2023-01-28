use crate::filerank::{File, Rank};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Square(u8);

impl Square {
    pub const NULL: Self = Self(64);

    pub const fn is_ok(self) -> bool {
        self.0 < 64
    }
    pub const unsafe fn new(value: u8) -> Self {
        // Unsafe because this can create an invalid square
        Self(value)
    }
    pub const fn create(file: File, rank: Rank) -> Self {
        unsafe { Self::new(file as u8 + 8 * rank as u8) }
    }
    pub const fn inner(self) -> u8 {
        self.0
    }

    pub const fn file(self) -> File {
        unsafe { std::mem::transmute(self.0 & 7) }
    }
    pub const fn rank(self) -> Rank {
        unsafe { std::mem::transmute((self.0 >> 3) & 7) }
    }

    pub const fn in_line(self, other: Self) -> bool {
        // Uses a lot of `as u8` to make it `const`-ified
        if self.file() as u8 == other.file() as u8 {
            true
        } else if self.rank() as u8 == other.rank() as u8 {
            true
        } else if (self.file() as u8).abs_diff(other.file() as u8) == (self.rank() as u8).abs_diff(other.rank() as u8) {
            true
        } else {
            false
        }
    }

    pub fn dist(self, other: Self) -> u32 {
        // Unsigned distance (king moves)
        let fd = (self.file() as u32).abs_diff(other.file() as u32);
        let rd = (self.rank() as u32).abs_diff(other.rank() as u32);
        fd.max(rd)
    }

    pub fn safe_move(self, jmp: i32) -> bool {
        if jmp < 0 && -jmp > self.inner() as i32 { return false; }
        let to = Self((self.inner() as i32 + jmp) as u8);
        if to.is_ok() && self.dist(to) <= 2 {
            true
        } else {
            false
        }
    }
}

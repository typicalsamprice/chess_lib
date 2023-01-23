#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Square(u8);

impl Square {
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

    /*
    pub const fn file(self) -> File {
        todo!();
    }
    pub const fn rank(self) -> Rank {
        todo!();
    }
    */

    pub const fn in_line(self, other: Self) -> bool {
        todo!();
    }
}

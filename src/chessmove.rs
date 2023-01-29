use crate::square::Square;
use crate::piece::PType;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move(u32);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum MType {
    #[default]
    Normal,
    EnPassant,
    Promotion,
    Castle
}

impl Move {
    pub const fn from(self) -> Square {
        unsafe { Square::new(self.0 as u8 & 7) }
    }
    pub const fn to(self) -> Square {
        unsafe { Square::new(((self.0 as u8) >> 6) & 7) }
    }
    pub const fn kind(self) -> MType {
        unsafe { std::mem::transmute(((self.0 as u8) >> 12) & 3) }
    }
    pub const fn promo(self) -> PType  {
        unsafe { std::mem::transmute((self.0 as u8) >> 14) }
    }

    pub fn new(from: Square, to: Square) -> Self {
        let f6 = from.inner() as u32;
        let s6 = (to.inner() as u32) << 6;
        Self(f6 | s6)
    }

    pub fn add_type(self, ty: MType) -> Self {
        Self(self.0 | ((ty as u32) << 12))
    }

    pub fn add_promo(self, ty: PType) -> Self {
        debug_assert_eq!(self.kind(), MType::Promotion);
        Self(self.0 | ((ty as u32) << 14))
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece(u8);

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PType {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
use chess_lib::prelude::*;

fn main() {
    init_comp();
    initalize_magics();

    let s1 = Square::create(File::E, Rank::Four);

    //print!("{}", rook_moves(s1, Bitboard::ZERO));
}

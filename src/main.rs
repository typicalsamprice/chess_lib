use chess_lib::prelude::*;

fn main() {
    init_comp();
    initalize_magics();

    let s1 = Square::create(File::E, Rank::Four);
    let oc = Bitboard::new(4536693952512);
    println!("{s1:?}");
    println!("{}", rook_moves(s1, oc));
}

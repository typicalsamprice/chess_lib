use chess_lib::prelude::*;
use chess_lib::prelude::individual_squares::*;

fn main() {
    init_comp();
    initalize_magics();
    let p = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse::<Position>();
    let _p = "rnbqkbnr/ppp3pp/8/5p2/8/5N2/PPPPQ1PP/RNB1KB1R b KQkq - 0 6".parse::<Position>();
    let mut p = p.unwrap();
    println!("{p}");
    println!("{}", p.fen());

    // b2b3: Missing 2 moves
    p.do_move(Move::new(B2, B3));
    let u = p.perft::<true>(3);
    println!("Nodes searched: {u}");
}

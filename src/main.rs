use chess_lib::prelude::individual_squares::*;
use chess_lib::prelude::*;

fn main() {
    init_comp();
    initalize_magics();
    let p = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse::<Position>();
    let _p = "rnbqkbnr/ppp3pp/8/5p2/8/5N2/PPPPQ1PP/RNB1KB1R b KQkq - 0 6".parse::<Position>();
    let mut p = p.unwrap();

    // p.do_move(Move::new(B2, B3));
    // p.do_move(Move::new(E7, E6));
    // p.do_move(Move::new(C1, A3));

    // p.do_move(Move::new(C2, C4));
    // p.do_move(Move::new(D7, D5));
    // p.do_move(Move::new(D1, A4));
    println!("{p}");
    println!("{}", p.fen());
    let u = p.perft::<true>(5);
    println!("Nodes searched: {u}");
}

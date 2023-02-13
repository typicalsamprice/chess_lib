use chess_lib::prelude::individual_squares::*;
use chess_lib::prelude::*;

macro_rules! do_moves {
    ($P:ident $CNT:ident; $($F:ident $T:ident)*) => {
        $(
            $CNT -= 1;
            $P.do_move(Move::new($F, $T));
        )*
    }
}

// FIXME Position::do_move is slow
// FIXME Position::attacks_to_occ may be slow
// FIXME Position::compute_state and Position::generate_all are possibly slow

fn main() {
    initalize_magics();
    init();
    chess_lib::zobrist::init_zobrist();
    println!("Done with initialization");

   // println!("{}", between::<true>(E8, B5));

    //let f = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    //let f = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    let mut p = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        .parse::<Position>()
        .unwrap();
    let u = p.perft::<true>(7);
    println!("Nodes: {}", u); // */
}

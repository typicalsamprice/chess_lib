use chess_lib::prelude::individual_squares::*;
use chess_lib::{evaluate, prelude::*, diagnostics};

use chess_lib::debug;

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
    debug!("Done with initialization");

    //let f = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    //let f = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";
    let f = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    let mut p = f.parse::<Position>().unwrap();
    //let mut p = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    //    .parse::<Position>().unwrap();
    let mut m = Move::new(A1, A1);
    let e = evaluate::alpha_beta(&mut p, &mut m, 3);
    debug!("Beta cutoffs: {}", diagnostics::get_beta_cutoffs());
    debug!("Leaf nodes: {}", diagnostics::get_alphabeta_leaf_nodes());
    println!("{p}");
    println!("Best Move: {m} (E: {e})"); // */
}

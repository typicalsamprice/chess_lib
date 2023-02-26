use std::str::FromStr;

use chess_lib::evaluate;
use chess_lib::prelude::*;
use chess_lib::prelude::individual_squares::*;

use chess_lib::debug;
use chess_lib::search;

// FIXME Position::do_move is slow
// FIXME Position::attacks_to_occ may be slow
// FIXME Position::compute_state and Position::generate_all are possibly slow

fn main() {
    chess_lib::initialize();
    debug!("Done with initialization");

    //let mut p = Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    //println!("Nodes searched: {}", p.perft::<true>(7));
    let mut p = Position::from_str("2r1k3/4ppp1/8/2q4b/8/3Q4/1r6/3RK3 w - - 0 1").unwrap();
    println!("{p}");
    let (l, e) = search::ab_with_pv(&mut p, 6);
    println!("Eval: {e}");
    for m in l.as_slice() {
        println!("{m}");
    }
}

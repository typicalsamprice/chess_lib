use chess_lib::evaluate::alpha_beta;
use chess_lib::prelude::*;
use chess_lib::search;
use chess_lib::diagnostics;

use chess_lib::debug;

// FIXME Position::do_move is slow
// FIXME Position::attacks_to_occ may be slow
// FIXME Position::compute_state and Position::generate_all are possibly slow

fn main() {
    chess_lib::initialize();
    debug!("Done with initialization");

    let mut p = Position::STARTPOS.parse::<Position>().unwrap();

    let (l, e) = search::ab_with_pv(&mut p, 6);
    println!("Eval: {e}\n");
    for m in l.as_slice() {
        println!("{m}");
    }
    debug!(pv.len());
}

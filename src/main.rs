use std::str::FromStr;

use chess_lib::prelude::*;

use chess_lib::debug;

// FIXME Position::do_move is slow
// FIXME Position::attacks_to_occ may be slow
// FIXME Position::compute_state and Position::generate_all are possibly slow

fn main() {
    chess_lib::initialize();
    debug!("Done with initialization");

    let mut p = Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    println!("Nodes searched: {}", p.perft::<true>(7))
}

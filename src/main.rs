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

fn main() {
    init_comp();
    initalize_magics();

    let mut p = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        .parse::<Position>()
        .unwrap();
}

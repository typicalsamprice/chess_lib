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
    let _p = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse::<Position>();
    let _p = "rnbqkbnr/ppp3pp/8/5p2/8/5N2/PPPPQ1PP/RNB1KB1R b KQkq - 0 6".parse::<Position>();
    let p = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -".parse::<Position>();
    let mut p = p.unwrap();

    let mut i = 5;
    // STARTINGPOS
    //do_moves! { p i; H2 H4 E7 E6 F2 F3 F8 B4 }
    //do_moves! { p i; D2 D3 A7 A6 E1 D2 A6 A5 }
    // do_moves! { p i; A2 A3 E8 C8 } /* KIWIPETE */
    // do_moves! { p i; H1 G1 E8 C8 } /* KIWIPETE */
    println!("{p}");
    println!("{}", p.fen());
    let u = p.perft::<true>(dbg!(i));
    println!("Nodes searched: {u}");
}

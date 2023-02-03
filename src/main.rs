use chess_lib::prelude::*;

fn main() {
    init_comp();
    initalize_magics();
    let p = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse::<Position>();
    let _p = "rnbqkbnr/ppp3pp/8/5p2/8/5N2/PPPPQ1PP/RNB1KB1R b KQkq - 0 6".parse::<Position>();
    let mut p = p.unwrap();
    println!("{p}");
    println!("{}", p.fen());

    let u = p.perft::<true>(3);
    println!("Nodes searched: {u}");
}

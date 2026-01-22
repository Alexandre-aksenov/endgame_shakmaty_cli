use shakmaty::{Chess, Position, fen::Fen, CastlingMode, uci::UciMove};
// see: https://docs.rs/shakmaty/latest/shakmaty/fen/index.html

use std::io; // to query the Player's moves.


// fn query_players_move(mut pos: &mut dyn Position)
//->
// fn <T> query_players_move(mut pos: T)
// where T: Sized + Position
//->
// fn query_players_move(mut play_board: Chess)
//->
/// Query the player's move. To-CALL, To-refactor 
fn query_players_move<T: Sized + Position>(pos : &mut T)
{
    loop {
        println!("Enter UCI move:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // let success = play_board.apply_uci_move(input.trim());
        // awaiting_player_move = success;
        // ->
        // let uci: UciMove = input.trim().parse()?;
        let mut success = true;
        
        // try to parse the move: https://docs.rs/shakmaty/latest/shakmaty/uci/index.html
        let uci: UciMove = match input.parse() {
            Ok(mv) => mv,
            Err(_) => { continue; }
        };

        // Try Converting to a legal move in the context of a position:
        // let m = uci.to_move(pos)?;
        // -> Requires trait Sized + Position
        //->
        let m = match uci.to_move(pos) {
            Ok(mv) => mv,
            Err(_) => { continue; }
        };
        
        
        if success { break; }

    }
}


fn main() {
    
    println!("Hello, world!");

    // Position by Barbieri-Saavedra
    /*
    W: Kb6, c6.
    B: Ka1, Rd5.
    White to move.

    Winning move: c6-c7!
     */

    let fen: Fen = "8/8/1KP5/3r4/8/8/8/k7 w - - 0 0".parse().unwrap();
    let mut study: Chess = fen.into_position(CastlingMode::Standard).unwrap();
    
    // Print the pos of study
    println!("Init position");
    // println!("{:?}", study); // Too much printed info!
    // ->
    println!("{}", study.board());
}

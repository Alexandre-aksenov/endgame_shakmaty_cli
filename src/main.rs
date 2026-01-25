use shakmaty::{Chess, Position, fen::Fen, CastlingMode};
// see: https://docs.rs/shakmaty/latest/shakmaty/fen/index.html

use shakmaty_syzygy::{Tablebase};


// use std::io; // to query the Player's moves.

extern crate endgame_shakmaty_cli;
use endgame_shakmaty_cli::{query_players_move, query_tablebase_move};


fn main() {

    println!("Position by Barbieri-Saavedra. White to move!");

    // Position by Barbieri-Saavedra
    /*
    W: Kb6, c6.
    B: Ka1, Rd5.
    White to move.

    Winning move: c6-c7!
     */

    let fen: Fen = "8/8/1KP5/3r4/8/8/8/k7 w - - 0 0".parse().unwrap();
    let mut study: Chess = fen.into_position(CastlingMode::Standard).unwrap();

    // import the tablebase.
    let mut tables = Tablebase::new();
    tables.add_directory("tables").expect("Could not add tablebase directory");
    
    // Print the pos of study
    println!("Init position");
    println!("{}", study.board()); // FEN. Small output for end user, but enough for dev.

    // Main loop
    let mut awaiting_player_move = true;
    while !study.is_game_over()
    {
        if awaiting_player_move {
            // Query the player's move.
            query_players_move(&mut study);
            println!("{}", study.board());
        }
        else 
        { 
            let opponent_reply = query_tablebase_move(&study, &tables);
            study.play_unchecked(opponent_reply) ;
            println!("Position after opponent's move:");
            println!("{}", study.board()); // 8/2P5/1K1r4/8/8/8/8/k7 -> main line!
        }
        
        awaiting_player_move = !awaiting_player_move;
    }
    
    println!("Game over. Result: {}", study.outcome().as_str());
    // 1-0 in case of good play by W.
}



use shakmaty::{Chess, Position, uci::UciMove, Move};
use shakmaty_syzygy::Tablebase;
use std::io; // to query the Player's moves.


/// Best move from the tablebase. Next step: Result<Move, String>
pub fn query_tablebase_move(pos :  &Chess, tables: &Tablebase<Chess>) -> Move
{
    let tup_move = tables
        .best_move(pos)
        .expect("Position was not found.")
        .expect("Could not find the best move.");

    return tup_move.0;
}

/// Query the player's move and return it to the main loop.
pub fn query_player_wait<T: Sized + Position>(pos : &mut T) -> Move
{
    let mut candidate_move = None;

    while candidate_move.is_none() {
        println!("Enter UCI move:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Debug:
        println!("Raw input: {:?}", input);
        let input = input.trim(); // <-- trim the user input from '/n'

        // try to parse the move: https://docs.rs/shakmaty/latest/shakmaty/uci/index.html
        let uci: UciMove = match input.parse() {
            Ok(mv) => mv,
            Err(_) => { println!("Failed to parse the move."); continue; }
        };

        // Try Converting to a legal move in the context of a position:
        candidate_move = match uci.to_move(pos) {
            Ok(mv) => Some(mv),
            Err(_) => { println!("Illegal move."); continue; }
        };

    }
    
    candidate_move.expect("The loop ended in an unexpected state.")
}

/*
/// Play the move.
pub fn play_opt_move<T: Sized + Position>(pos : &mut T, opt_mv: Option<Move>)
{
    match opt_mv {
        Some(mv) => pos.play_unchecked(mv),
        None => {}
    }
}
*/

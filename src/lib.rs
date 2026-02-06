use shakmaty::{Chess, Position, uci::UciMove, Move};
use shakmaty_syzygy::Tablebase;
use std::io; // to query the Player's moves.


/// Best move from the tablebase.
fn query_tablebase_move(pos :  &Chess, tables: &Tablebase<Chess>) -> Move
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
        // println!("Raw input: {:?}", input);
        let input = input.trim(); // <-- trim the user input from '/n'
        
        candidate_move = match check_uci_to_move(pos, input) {
            Ok(mv) => Some(mv),
            Err(err) => {println!("Encountered problem: {}", err); None},
        }
        
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

/// Return Rd3-d4 (hardcoded) or the Tablebase's move.
pub fn query_opponent_move(pos :  &Chess, tables: &Tablebase<Chess>) -> Result<Move, String>
{
    let pos_fen = format!("{}", pos.board());
    let pos_pieces = str_chess_pieces(pos_fen.as_str());
    
    match pos_pieces == String::from("8/2P5/8/8/8/3r4/2K5/k7") {
        true => check_uci_to_move(pos, &String::from("d3d4")),
        // false => Ok(query_tablebase_move(pos, tables)) // -> 2nd check
        false => match pos_pieces == String::from("8/2P5/8/8/8/8/2K5/k2r4") {
            true => check_uci_to_move(pos, &String::from("d1d4")),
            false => Ok(query_tablebase_move(pos, tables))
        }
    }
}

/// Leave just the pieces part of the FEN.
/// Example: "8/8/1KP5/3r4/8/8/8/k7 w - - 0 0" -> "8/8/1KP5/3r4/8/8/8/k7"
fn str_chess_pieces(full_fen: &str) -> &str
{
    // first word
    full_fen.split_whitespace().next().unwrap()
}

/// try to parse the move: https://docs.rs/shakmaty/latest/shakmaty/uci/index.html
fn check_uci_to_move<T: Sized + Position>(pos : &T, input: &str) -> Result<Move, String>
{
    // Debug:
    // println!("In check_uci_to_move, parsing input: {:?}", input);
    
    let uci: UciMove = match input.parse() {
        Ok(mv) => mv,
        Err(_) => { // println!("Failed to parse the move."); continue;
            return Err(String::from("Failed to parse the move."));
            }
    };

    // Try Converting to a legal move in the context of a position:
    let candidate_move = match uci.to_move(pos) {
        Ok(mv) => mv,
        Err(_) => { // println!("Illegal move."); continue;
        return Err(String::from("Illegal move."))}
    };

    Ok(candidate_move)
}

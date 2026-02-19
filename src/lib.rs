use shakmaty::{Chess, Position, Move}; // uci::UciMove, 
// use shakmaty_syzygy::Tablebase; // Only needed in the main branch.
use std::io; // to query the Player's moves.
use str_move::check_uci_to_move;

use remote_tablebase::query_remote_tablebase_move; 

// Best move from the local tablebase.
/*
fn query_tablebase_move(pos :  &Chess, tables: &Tablebase<Chess>) -> Move
{
    let tup_move = tables
        .best_move(pos)
        .expect("Position was not found.")
        .expect("Could not find the best move.");

    return tup_move.0;
}

 */

/// Query the player's move and return it to the main loop.
pub fn query_player_wait<T: Sized + Position>(pos : &mut T) -> Move
{
    let mut candidate_move = None;

    while candidate_move.is_none() {
        println!("Enter UCI move:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        candidate_move = match check_uci_to_move(pos, input.trim()) {
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

/// Return Rd4 (hardcoded) or the Tablebase's move.
pub fn query_opponent_move(pos :  &Chess, ) -> Result<Move, String>
{
    let pos_fen = format!("{}", pos.board());
    let pos_pieces = str_chess_pieces(pos_fen.as_str());
    
    match pos_pieces == String::from("8/2P5/8/8/8/3r4/2K5/k7") {
        true => check_uci_to_move(pos, &String::from("d3d4")),
        // -> 2nd check
        false => match pos_pieces == String::from("8/2P5/8/8/8/8/2K5/k2r4") {
            true => check_uci_to_move(pos, &String::from("d1d4")),
            // false => Ok(query_tablebase_move(pos, tables))  // tables: &Tablebase<Chess>
            false => query_remote_tablebase_move(pos) // -> remote tablebase
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



/// "Pretty-print" position after the opponent's move.
/// Adding numbers of ranks & files
pub fn pretty_format<T: Sized + Position>(pos : &T) -> String
{
    /*
    Current layout:

8 . . . . . . . .
7 . . . . . . . .
6 . K P . . . . .
5 . . . r . . . .
4 . . . . . . . .
3 . . . . . . . .
2 . . . . . . . .
1 k . . . . . . .
  a b c d e f g h

    */

    let mut vec_str_result = vec![];

    // row of column names
    let cols = String::from("  a b c d e f g h");
    let ranks = vec!['8', '7', '6', '5', '4', '3', '2', '1',];

    let min_layout = format!("{:?}", pos.board());

    for (line, rnk,) in min_layout.lines().zip(ranks.iter())  {
        {
            // prepend rank , ' '
            let expanded_fmt = format!("{} {}", rnk, line);
            vec_str_result.push(expanded_fmt);
        }
    }

    vec_str_result.push(cols);
    return vec_str_result.join("\n");
}

/// Module for quering the remote tablebase
mod remote_tablebase{
    use serde_json;
    use shakmaty::{Chess, Move, fen::Fen, EnPassantMode, };
    use crate::str_move::check_uci_to_move;

    #[derive(serde::Deserialize)]
    struct Moves {
        moves: Vec<serde_json::Value>,
    }

    /// Useful part of the tablebase info about a move
    #[derive(serde::Deserialize)]
    struct InfoMove {
        uci: String,
    }

    impl InfoMove {
        fn new(full_info_move: &serde_json::Value) -> Self
        {
            let uci_move = full_info_move["uci"].as_str().expect("Could not parse tablebase info: missing uci field");
            Self { uci: uci_move.to_string() }
        }
    }

    /// Recommended move from tablebase on Lichess.
    pub fn query_remote_tablebase_move(pos :  &Chess) -> Result<Move, String>
    {
        let query_url = pos_to_url(&pos);
        // let remote_move_uci = query_lichess_tablebase_move(&query_url)?; // ->
        let remote_move_uci = query_lichess_tablebase_move(&query_url).unwrap_or(String::from("Failed to query remote tablebase"));

        check_uci_to_move(pos, &remote_move_uci)
    }

    /// Lichess tablebase URL from Position
    fn pos_to_url(pos :  &Chess) -> String
    {

        // Part 1
        let mut result = String::from("https://tablebase.lichess.ovh/standard?fen=");

        // Part 2
        let fen = Fen::from_position(pos, EnPassantMode::Always);
        fen.append_to_string(&mut result);

        // replace spaces with underscores for making an arg for 'curl'
        result = result.replace(" ", "_");

        // Part 3
        result
    }

    /// query Lichess tablebase, return the best move in UCI format
    fn query_lichess_tablebase_move(query : &str) -> Result<String, reqwest::Error>
    {
        let response = reqwest::blocking::get(query)?;

        if !response.status().is_success() {
            let status = response.status();
            eprintln!("API request failed with status: {}", status);
            // We could read the body here, but it would consume the response.
            // For debugging purposes, if it's not success, it will likely fail during JSON decoding anyway.
        }

        let response_json = response.json::<Moves>()?;

        // let read_uci = json_to_uci(&response_json.moves[0]).expect("Could not parse tablebase info");
        let read_uci = InfoMove::new(&response_json.moves[0]).uci;

        Ok(read_uci)
    }
}

/// Module for parsing the input move from the Player or the remote tablebase.
//pub mod str_move{
mod str_move{
    use shakmaty::{Move, Position};
    use shakmaty::uci::UciMove;

    /// try to parse the move: https://docs.rs/shakmaty/latest/shakmaty/uci/index.html
    pub fn check_uci_to_move<T: Sized + Position>(pos : &T, input: &str) -> Result<Move, String>
    {
        // Possible debug:
        // println!("In check_uci_to_move, parsing input: {:?}", input);

        let uci: UciMove = match input.parse() {
            Ok(mv) => mv,
            Err(_) => {
                return Err(String::from("Failed to parse the move."));
            }
        };

        // Try Converting to a legal move in the context of a position:
        let candidate_move = match uci.to_move(pos) {
            Ok(mv) => mv,
            Err(_) => {
                return Err(String::from("Illegal move."))}
        };

        Ok(candidate_move)
    }
}

/// Test module
#[cfg(test)]
mod tests {
    use shakmaty::CastlingMode;
    use shakmaty::fen::Fen;
    use super::*;

    #[test]
    fn test_pretty_print_v1() {
        let fen: Fen = "8/8/1KP5/3r4/8/8/8/k7 w - - 0 0".parse().unwrap();
        let study: Chess = fen.into_position(CastlingMode::Standard).unwrap();

        // Print the pos of study
        let formatted = pretty_format(&study);
        let expected = "8 . . . . . . . .\n7 . . . . . . . .\n6 . K P . . . . .\n5 . . . r . . . .\n4 . . . . . . . .\n3 . . . . . . . .\n2 . . . . . . . .\n1 k . . . . . . .";
        assert_ne!(formatted, expected );
    }

    #[test]
    fn test_pretty_print_v2() {
        let fen: Fen = "8/8/1KP5/3r4/8/8/8/k7 w - - 0 0".parse().unwrap();
        let study: Chess = fen.into_position(CastlingMode::Standard).unwrap();

        // Print the pos of study
        let formatted = pretty_format(&study);
        let expected = "8 . . . . . . . .\n7 . . . . . . . .\n6 . K P . . . . .\n5 . . . r . . . .\n4 . . . . . . . .\n3 . . . . . . . .\n2 . . . . . . . .\n1 k . . . . . . .\n  a b c d e f g h";
        assert_eq!(formatted, expected );
    }
}

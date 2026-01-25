use shakmaty::{Chess, Position, fen::Fen, CastlingMode, uci::UciMove, Move};
// see: https://docs.rs/shakmaty/latest/shakmaty/fen/index.html

use shakmaty_syzygy::{Tablebase, MaybeRounded, Wdl};


use std::io; // to query the Player's moves.


// fn query_players_move(mut pos: &mut dyn Position)
//->
// fn <T> query_players_move(mut pos: T)
// where T: Sized + Position
//->
// fn query_players_move(mut play_board: Chess)
//->
/// Query the player's move and make it. To-refactor
fn query_players_move<T: Sized + Position>(pos : &mut T)
{
    loop {
        println!("Enter UCI move:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Debug:
        println!("Raw input: {:?}", input);
        let input = input.trim(); // <-- critical addition

        // let success = play_board.apply_uci_move(input.trim());
        // awaiting_player_move = success;
        // ->
        let success = true;

        // try to parse the move: https://docs.rs/shakmaty/latest/shakmaty/uci/index.html
        let uci: UciMove = match input.parse() {
            Ok(mv) => mv,
            Err(_) => { println!("Failed to parse the move."); continue; }
        };

        // Try Converting to a legal move in the context of a position:
        // let m = uci.to_move(pos)?;
        // -> Requires trait Sized + Position
        //->
        let m = match uci.to_move(pos) {
            Ok(mv) => mv,
            Err(_) => { println!("Illegal move."); continue; }
        };

        // Play the move.
        pos.play_unchecked(m);

        if success { break; }

    }
}

/// Best move from the tablebase. Next step: Result<Move, String>
fn query_tablebase_move(pos :  &Chess, tables: &Tablebase<Chess>) -> Move
{
    let tup_move = tables
        .best_move(pos)
        .expect("Position was not found.")
        .expect("Could not find the best move.");
    
    return tup_move.0;
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
}



use shakmaty::{Chess, Position, fen::Fen, CastlingMode};
// see: https://docs.rs/shakmaty/latest/shakmaty/fen/index.html

// use shakmaty_syzygy::{Tablebase};
// Only needed in the main branch.

extern crate endgame_shakmaty_cli;
use endgame_shakmaty_cli::{pretty_format};
use endgame_shakmaty_cli::player::query_player_wait;
use endgame_shakmaty_cli::opponent::query_opponent_move;


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

    // import the tablebase : only needed in the main branch.
    // let mut tables = Tablebase::new();
    // tables.add_directory("tables").expect("Could not add tablebase directory");
    
    // Print the pos of study
    println!("Init position");
    println!("{}", pretty_format(&study));
    /*
. . . . . . . .
. . . . . . . .
. K P . . . . .
. . . r . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
k . . . . . . .
     */
    
    // Main loop
    let mut awaiting_player_move = true;
    while !study.is_game_over()
    {
        if awaiting_player_move {
            let players_play = query_player_wait(&mut study);
            study.play_unchecked(players_play);
            println!("FEN: {}", study.board());
        }
        else
        {
            let opponent_reply = query_opponent_move(&study).expect("Could not query opponent's move.");
            // 2nd parameter (&tables) is needed in the main branch with local tablebase.
            
            println!("Opponent's move: {}", opponent_reply); // Examples: "Rd6-d5", "Rd7xc7"
            study.play_unchecked(opponent_reply) ;
            println!("Position after opponent's move:");
            println!("{}", pretty_format(&study)); // -> main line!
        }

        awaiting_player_move = !awaiting_player_move;
    }

    // Print the final position and result 
    let result = study.outcome().as_str();

    println!("{:?}", study.board());
    println!("Game over. Result: {}", result);
    // 1-0 in case of good play by W.
    
    match result == String::from("1-0") { 
        true => println!("Congratulations! You are victorious!"),
        false => println!("You failed to achieve victory."),
    }
}



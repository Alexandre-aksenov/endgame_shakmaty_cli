# Endgame training for chess players.

This computer game simulates solving exercises from a book about chess endgames.
The player is presented with a position and is asked to play against a program.
The player is told at the end of the game whether the result corresponds to the expected one.
In this case, the player can progress to the next exercise.

The player is strongly encouraged to solve the position before playing it.
Breaking the program's defense should merely serve to validate the solution.

## The opponent.

The current opponent is a subset of Syzygy tablebase.
It selects the move with the best outcome and is expected to prefer the line with the highest number of moves until a pawn move or capture.
Its play is not the strongest one when several "equivalent" moves are possible.

## Usage.

To play the game, the user should have Rust installed, clone the repository, download the necessary tables from <code>http://tablebase.sesse.net/syzygy/3-4-5/</code>
to the folder <code>tables</code> and run the following in the root folder:
```bash
ls ./tables/
#-> {the list of files to download}
# KBvK.rtbw  KNvK.rtbw  KPvK.rtbw  KQvKR.rtbw  KQvK.rtbw  KRvKB.rtbw  KRvKN.rtbw  KRvKP.rtbw  KRvKR.rtbw  KRvK.rtbw
# KBvK.rtbz  KNvK.rtbz  KPvK.rtbz  KQvKR.rtbz  KQvK.rtbz  KRvKB.rtbz  KRvKN.rtbz  KRvKP.rtbz  KRvKR.rtbz  KRvK.rtbz

cargo build

cargo run
```

The game has been developed and tested on Rust 1.92.0 stable.

The player's moves should be entered in the UCI format:

<code>{square of departure}{square of arrival}[optional piece of promotion]</code>

For instance: <code>c6c7</code>, then <code>c7c8q</code>.

## Limitations and future work.

This is a work in progress. It currently contains a single position (position by Barebieri-Saavedra) and is limited to CLI interface.

All <b>suggestions</b> and <b>feedback</b> should be adressed to its author Alexandre Aksenov:
* GitHub: Alexandre-aksenov
* Email: alexander1aksenov@gmail.com

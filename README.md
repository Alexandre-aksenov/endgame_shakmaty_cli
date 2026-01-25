# Endgame training for chess players.

This computer game simulates solving exercises from a book about chess endgames.
The player is presented with a position and is asked to play it against a program.
The player is told at the end of the game whether the result corresponds to the expected one.
In this case, the player can progress to the next exercise.

The player is strongly encouraged to solve the position before playing it.
Breaking the program's defense should merely serve to validate the solution.

## The opponent.

The current opponent is a subset of Syzygy tablebase.
It selects the move with the best outcome and is expected to prefer the line with the highest number of moves until capture.
If the opponent's move does not follow this rule, the player is kindly asked to communicate this behaviour to the author of the repository.

To run the program, the user should have Rust installed and download the necessary tables from <code>http://tablebase.sesse.net/syzygy/3-4-5/</code>
to the folder <code>tables</code> and run the following in the root folder:
```bash
ls ./tables/
#-> {the list of files to download}
# KPvK.rtbw  KPvK.rtbz  KQvKR.rtbw  KQvKR.rtbz  KQvK.rtbw  KQvK.rtbz  KRvKP.rtbw  KRvKP.rtbz  KRvKR.rtbw  KRvKR.rtbz  KRvK.rtbw  KRvK.rtbz

cargo build

cargo run
```

## Limitations and future work.

This is a work in progress. It currently contains a single position (position by Barebieri-Saavedra) and is limited to CLI interface.

All <b>suggestions</b> and <b>feedback</b> should be adressed to its author Alexandre Aksenov:
* GitHub: Alexandre-aksenov
* Email: alexander1aksenov@gmail.com

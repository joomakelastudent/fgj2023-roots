/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

// Standard library imports
    use std::collections::HashMap;
    use std::io::{BufRead, BufReader};
    use std::fs::File;
    use std::io;
    use std::env;
    use std::fs;

use gamestate::{EnemyState, GameState};
// external dependencies
    use tui;

// our modules
    //use crate::gamestate::GameState;
    mod gamestate;
    mod render;


// Public constant declarations

/*
should level be mutable or not?
probably yes. it's easier to handle one state than to handle one state and then the exceptions.
*/

/* Where the preliminary setup for the game happens
 * Mostly does one-time work that the main loop requires to start
 * Locates and "loads" the level from the filesystem
 * Verifys that the terminal window has enough rows and columns
 * Starts up the main game loop
 * Takes no parameters
 * Returns an exit status to the OS
 */
fn main() {
    
    let map = load_map();
    // We need some basic info from the state to start/advance the main loop
    let mut game_state: GameState = initialize_game_state(&map);

    // If we get here, assume everything is good to go
    //gameloop(level);
}

fn load_map() -> String  /*[[char; 64]; 64]*/ {
    let contents = fs::read_to_string("placeholder.map")
        .expect("Error opening map.");
    
    contents
    /*
    let mut map: [[char; 64]; 64];

    Loop {
        let my_char = contents.next();

        for _y in 0..63 {
            for _x in 0..63 {
                map[_x][_y] = contents; // include newline in map    
                if (ch == '\n') { break; } // switch to new line
            }
        }
    }
    */
}


/* Where the main logic of the game happens
 * Organizes all of the separate systems of the game
 * Advances all systems by one step each cycle
 * One cycle equals one "tick" of the game
 * Takes the level data as a parameter
 * Returns nothing
 */
fn gameloop(mut game_state: gamestate::GameState) {
    loop {
        // Preliminary comment
    }
}

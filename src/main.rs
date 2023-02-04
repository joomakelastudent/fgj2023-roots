/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

// Standard library imports
use std::fs;

// External crates/dependencies
use tui;

// Our own modules
mod gamestate;
use gamestate::GameState;
mod render;

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
    let mut game_state: GameState = initialize_game_state(map);

    // If we get here, assume everything is good to go
    gameloop(game_state);
}

/* Loads a map-file from disk to memory
 * Takes no parameters
 * Returns a String
 */
fn load_map() -> Vec<char> {
    let contents = fs::read_to_string("../../assets/placeholder.map")
        .expect("Error opening map.");
    
    let mut map_vector: Vec<char> = Vec::new();

    let mut line_width = 0;

    //for (i, ch) in contents.chars().enumerate() {
    for ch in contents.chars() {   
        //pad the rest to 64 width when newline is found

        if ch == '\n' {
            while line_width <= 64 {
                map_vector.push(' ');
                line_width += 1;
            }
            line_width = 0;
        }

        //not pad
        map_vector.push(ch);
        line_width += 1;
    }

    map_vector
}

/* Creates a preliminary game starting state from scratch
 * Fills the state struct with default values
 * Takes a map as a parameter
 * Returns a usable GameState struct
 */
fn initialize_game_state(mapstring: Vec<char>) -> GameState {
    let mut list_of_enemies: Vec<gamestate::EnemyState>;

    // This is not pretty. We know
    let mut game_state = gamestate::GameState {
        map: mapstring,
        control_state: gamestate::ControlState::ACTION,
        player_state: gamestate::PlayerState {
            location: gamestate::Location {
                x: 5,
                y: 5,
            },
            health: gamestate::Health {
                current: 3,
                max: 5,
            },
            heading: 1,
            dash_cooldown: 0,
            attack_cooldown: 0,
            invis_frames: 0,
        },
        enemy_list: list_of_enemies,
    };

    return game_state;
}

/* Where the main logic of the game happens
 * Organizes all of the separate systems of the game
 * Advances all systems by one step each cycle
 * One cycle equals one "tick" of the game
 * Takes the level data as a parameter
 * Returns nothing
 */
fn gameloop(mut game_state: GameState) {
    loop {
        // Capture input state
        // 
        render::render(&game_state);
    }
}

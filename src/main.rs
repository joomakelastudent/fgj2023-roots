/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

// Standard library imports
    use std::fs;

// external dependencies
    use tui;

// our modules
    //use crate::gamestate::GameState;
    mod gamestate;
    use gamestate::GameState;
    mod render;

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
    let mut game_state: GameState = initialize_game_state(map);

    // If we get here, assume everything is good to go
    //gameloop(level);
}

fn load_map() -> String {
    let contents = fs::read_to_string("../../assets/placeholder.map")
        .expect("Error opening map.");
    contents
}

/* Creates a preliminary game starting state from scratch
 * Fills the state struct with default values
 * Takes a map as a parameter
 * Returns a usable GameState struct
 */
fn initialize_game_state(mapstring: String) -> GameState {
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
        
        // Preliminary comment
    }
}

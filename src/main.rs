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

fn main() {
    
    let map = load_map();
    // We need some basic info from the state to start/advance the main loop
    let mut game_state: GameState = initialize_game_state(map);

    gameloop(game_state);
}

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
            heading: gamestate::Heading::NORTH,
            dash_cooldown: 0,
            attack_cooldown: 0,
            invis_frames: 0,
        },
        enemy_list: list_of_enemies,
    };

    return game_state;
}

fn gameloop(mut game_state: GameState) {
    loop {
        // Start current loop timer
        // Capture input state
        // Resolve input (if any)
        // Do player actions and check for their legality
        // 
        render::render(&game_state);
        // Check loop timer and wait if we did things too fast
        // Basically limit fps via the system clock
    }
}

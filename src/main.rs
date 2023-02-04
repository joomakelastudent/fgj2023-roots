/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

// Standard library imports
use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};
use std::fs;
use std::thread;

// External crates/dependencies
use tui;
//use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};

// Our own modules
mod gameconsts;
mod gamestate;
use gamestate::GameState;
mod render;

fn main() {
    
    let map = load_map();
    // We need some basic info from the state to start/advance the main loop
    let mut game_state: GameState = initialize_game_state(map);

    set_up_terminal();

    gameloop(game_state);
}

fn load_map() -> Vec<char> {
    let contents = fs::read_to_string("./assets/placeholder.map")
        .expect("Error opening map.");
    
    let mut map_vector: Vec<char> = Vec::new();

    let mut line_width = 0;
    
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

        if line_width == 64 {
            map_vector.push('\n');
        }
    }

    map_vector
}

fn initialize_game_state(mapstring: Vec<char>) -> GameState {
    let mut list_of_enemies: Vec<gamestate::EnemyState> = vec![];

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
            facing: gamestate::Facing::NORTH,
            dash_cooldown: 0,
            attack_cooldown: 0,
            invis_frames: 0,
            movement_cooldown: 0,
            moving: false,
        },
        enemy_list: list_of_enemies,
    };

    return game_state;
}

fn set_up_terminal() {

}

fn gameloop(mut game_state: GameState) {
    loop {
    let tick_start = Instant::now();
    // Start current loop timer
    
    //limited to one input due to time constraints
    
    //if(pressed_keys.contains(Keycode.W)
    let pressed_keys = get_input();
    
    // do player actions based on code.
    // Do player actions and check for their legality
    // Run entity logic systems
    // Check if more enemies can be spawned

    //render::render(&game_state);
    limit_tickrate(&tick_start);
    }
}

//would do SOCD https://github.com/gilsrus/SOCD-Cleaner/blob/master/SOCDCleaner.ino
//not enough time though
fn get_input() -> Vec<Keycode> {
    let device_state = DeviceState::new();
    device_state.get_keys()
}

fn limit_tickrate(tick_start: &Instant) {
    let elapsed_time = tick_start.elapsed();
    let min_tick_duration = Duration::from_millis(1);
    if elapsed_time < min_tick_duration {
        let time_to_wait = min_tick_duration - elapsed_time;
        thread::sleep(time_to_wait);
    }
}

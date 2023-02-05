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
//use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};


//use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use rand::Rng;

// Our own modules
mod gameconsts;
mod gamestate;
use gamestate::{GameState, EnemyState};
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
        enemy_spawner: gamestate::EnemySpawner {
            cooldown: 100,
        },
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

    let device_state = DeviceState::new();
    let pressed_keys = device_state.get_keys();
    
    //let mouse: MouseState = device_state.get_mouse();
    //println!("Current Mouse Coordinates: {:?}", mouse.coords);

    if game_state.player_state.movement_cooldown <= 0 {
        if pressed_keys.contains(&Keycode::W) {
            game_state.player_state.location.y -= 1;
            game_state.player_state.movement_cooldown = 20;
        }
        else if pressed_keys.contains(&Keycode::S) {
            game_state.player_state.location.y += 1;
            game_state.player_state.movement_cooldown = 20;
        }
        if pressed_keys.contains(&Keycode::A) {
                game_state.player_state.location.x -= 2;
                game_state.player_state.movement_cooldown = 20;
        }
        else if pressed_keys.contains(&Keycode::D) {
            game_state.player_state.location.x += 2;
            game_state.player_state.movement_cooldown = 20;
        }
    }


    game_state.player_state.movement_cooldown -= 1;

    let mut x = game_state.player_state.location.x;
    let mut y = game_state.player_state.location.y;
    
    //limit player bounds
    x = x.max(3);
    x = x.min(39);

    y = y.max(1);
    y = y.min(15);

    //update player bounds to gamestate
    game_state.player_state.location.x = x;
    game_state.player_state.location.y = y;

    
    //spawn enemies on the right side of the screen for the player to dodge
    
    if game_state.enemy_spawner.cooldown < 1 {
        game_state.enemy_spawner.cooldown = 100;
        let mut new_enemy = vec![make_random_enemy_state()];
        game_state.enemy_list.append(&mut new_enemy);
    }
    
    game_state.enemy_spawner.cooldown -= 1;
    //update enemy state.
    
    for mut enemy in &mut game_state.enemy_list {
        
        if (enemy.movement_cooldown < 0) {
            if enemy.location.x == 2 {
                enemy.location.x = 1000;
            }
            else
            {
                enemy.location.x -= 1;
                enemy.movement_cooldown = 15;
            }
        }
        
        enemy.movement_cooldown -= 1;
    }
    

    //println!("{}",game_state.enemy_list.len());

    render::render(&game_state);

    //if player touches enemy, end game.
    /*
    for enemy in game_state.enemy_list {
        let player = game_state.player_state;

        if (enemy.location.x == player.location.x) & (enemy.location.y == player.location.x){
            end_game(&game_state);
        }
    }
    */

    limit_tickrate(&tick_start);
    }
}

fn make_random_enemy_state() -> EnemyState {
    EnemyState {
        location: gamestate::Location {
            x: 40,
            y: rand::thread_rng().gen_range(1..=15),// random y
        },
        movement_cooldown: 15,
    }
}

fn end_game(game_state: &GameState) {
    //todo joonas survived for some time
    std::process::exit(1);
}

fn limit_tickrate(tick_start: &Instant) {
    let elapsed_time = tick_start.elapsed();
    let min_tick_duration = Duration::from_millis(3);
    if elapsed_time < min_tick_duration {
        let time_to_wait = min_tick_duration - elapsed_time;
        thread::sleep(time_to_wait);
    }
    //todo joonas laita peli pyörimään aina nopeemmin kun aikaa on menny.
}

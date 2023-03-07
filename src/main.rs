/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Attribution-Noncommercial-Share Alike 3.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

// Standard library imports
use std::time::{Instant, Duration};
use std::fs;
use std::thread;

// External crates/dependencies
//use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};


//use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;

// Our own modules
mod gameconsts;
mod gamestate;
use gamestate::{GameState, MoverState};
mod render;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let map = load_map();
    // We need some basic info from the state to start/advance the main loop
    let gamestate: GameState = gamestate::initialize_gamestate(map);

    gameloop(gamestate);
}

// doesn't need to be fast cause it only happens once per run
fn load_map() -> Vec<char> {
    let contents = fs::read_to_string("./assets/placeholder.map")
        .expect("Error opening map."); 

    let mut map_vector: Vec<char> = Vec::new();
    
    //alternative faster data structure:
    //let mut map_array = [[false; gameconsts::MAX_WIDTH]; gameconsts::MAX_HEIGHT];

    let mut line_width = 0;
    
    //hard to read and might be error prone, but kinda works for now
    for ch in contents.chars() {

        if ch == '\n' {
            while line_width <= gameconsts::MAX_FRAME_WIDTH {
                map_vector.push(' ');
                line_width += 1;
            }
            line_width = 0;
        }

        map_vector.push(ch);
        line_width += 1;

        if line_width == gameconsts::MAX_FRAME_WIDTH {
            map_vector.push('\n');
        }
    }

    map_vector
}



/*
fn load_map_2() -> [[char; 64]; 64] {
    let contents = fs::read_to_string("./assets/placeholder.map")
        .expect("Error opening map.");

    //remove newlines -> pad and add newlines

    let something = [[' '; 64]; 64];

    let map_vector_2: Vec<String> = contents
        .lines()
        .map(|unpadded| format!("{unpadded:64}"))
        .join("\n");
        //.collect();

    for y in 0..=63 {
        for x in 0..=63 {
            let result = map_vector_2.get(y).chars()[x];
            something[y][x] = match
            
        }
        
    }

    array[['c'; 64]; 64]
}
*/

// +x and -x leads to no movement
// no movement takes no stamina
// always called for frame consistency reasons

fn update_player_location(pressed_keys: Vec<Keycode>, player: MoverState) {
    
    //used to check if the player moved
    let original_location = (player.x, player.y);
    
    struct Movement {x:i32, y:i32}
    let movement = Movement {x:0,y:0};

    //get wanted movement from pressed keys
    {
        if pressed_keys.contains(&Keycode::W) { movement.y -= 1; }
        if pressed_keys.contains(&Keycode::S) { movement.y += 1; }
        if pressed_keys.contains(&Keycode::A) { movement.x -= 2; }
        if pressed_keys.contains(&Keycode::D) { movement.x += 2; }
    }

    // try to move player
    {
        let player_wants_to_move = movement.x != 0 || movement.y != 0;
        let player_has_stamina = player.movement_cooldown <= 0;

        if player_wants_to_move && player_has_stamina {
            player.x += movement.x;
            player.y += movement.x;
        }    
    }

    //keep player within some box
    {
        player.x = player.x.max(3).min(39);
        player.y = player.y.max(1).min(15);
    }

    // put player on cooldown if it moved
    {
        let new_location= (player.x, player.y);
        let player_moved = new_location != original_location;

        if player_moved {
            player.movement_cooldown = 30;
        } 
    }
}

fn gameloop(mut gs: GameState) {
    loop {
    let tick_start = Instant::now();
    
    //limited to one input due to time constraints
    let device_state = DeviceState::new();
    let pressed_keys = device_state.get_keys();
    
    update_player_location(pressed_keys, gs.player);

    //spawn enemies on the right side of the screen for the player to dodge
    
    if gs.enemy_spawn_cooldown < 1 {
        gs.enemy_spawn_cooldown = 100;

        gs.enemy_list.push(
            MoverState {
                x: 40,
                y: rand::thread_rng().gen_range(1..=15),
                movement_cooldown: 15,
            }
        );
    }
    
    gs.enemy_spawn_cooldown -= 1;
    
    //update enemy locations
    for mut enemy in &mut gs.enemy_list {

        if enemy.movement_cooldown < 0 {
            if enemy.x == 2 {
                enemy.x = 40;
            }
            else
            {
                enemy.x -= 1;
                enemy.movement_cooldown = 15;
            }
        }

        //player dies when it stands on the same square as a snake
        if (enemy.x == gs.player.x) & (enemy.y == gs.player.y){
            std::process::exit(0);
        }
        
        enemy.movement_cooldown -= 1;
    }

    render::render(&gs);
    limit_tickrate(&tick_start);
    }
}

//basically a spinlock
fn limit_tickrate(tick_start: &Instant) {
    let elapsed_time = tick_start.elapsed();
    let min_tick_duration = Duration::from_millis(3);
    if elapsed_time < min_tick_duration {
        let time_to_wait = min_tick_duration - elapsed_time;
        thread::sleep(time_to_wait);
    }
}

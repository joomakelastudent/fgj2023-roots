/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

/*
assuming terminal ui

terminals are kinda garbage.
they're potentially thousands of times slower than their potential.

the main question with optimization is how much should be handled at a time.
considering that the assets will for now be in essentially an arbitrary format,
    doing a painter's algorithm is not a bad idea

slow code isolation would suggest that the main issue is feeding the terminal

i would assume that feeding it the entire frame.
    would be unlikely to be really really bad (system call problems)
    would produce the least stupid render bugs.
*/

use crate::gameconsts::*;
use crate::gamestate::GameState;

//mutability is not needed for rendering
pub fn render(game_state: &GameState) {
    
    //done, cause i couldn't copy it properly for some reason

    let mut framebuffer: Vec<char> = vec![];
    let player_location = &game_state.player_state.location;
    
    //todo joonas pushaa framebufferiin jotenkin selviydytty aika.

    let mut frame_x: usize = 0;
    let mut frame_y: usize = 0;
    for &ch in &game_state.map {
        frame_x += 1;
        if ch == '\n' {
            frame_x = 0;
            frame_y += 1;
        }

        //draw player
        if (frame_x == player_location.x) & (frame_y == player_location.y) {
            framebuffer.push('@');
            continue;
        }

        //else draw enemies
        let mut drew_enemy = false;
        for enemy in &game_state.enemy_list {
            if (frame_x == enemy.location.x) & (frame_y == enemy.location.y) {
                framebuffer.push('S');
                drew_enemy = true;
                break;
            }
        }

        if drew_enemy { continue; }

        framebuffer.push(ch);
    }
    
    /*
    reset_cursor();

    let player_location = &game_state.player_state.location;
    let player_sprite = '@';

    framebuffer[MAX_SIZE*player_location.y + player_location.x - 1] = player_sprite;
    */

    //
    reset_cursor();
    //print UI
    //println!("\tPLAYER at {}:{}", player_location.x, player_location.y);
    //print framebuffer
    println!("{}", framebuffer.iter().cloned().collect::<String>());
}

fn reset_cursor() {
    print!("{esc}[3J{esc}[1;1H", esc = 27 as char);
}

//27 desimaalina
//heksana 33

fn testcolours () {
    for i in 0..7{
        let index1 = 30+i;
        let index2 = 47-i;//perhaps bug?
        //let colour_string = f!("\x1b[{index1}m{index1} and {index2}\x1b[{index2}m").to_string();
        let colour_string = format!("\x1b[{}m{} and {}\x1b[{}m", index1,index1,index2,index2);
        
        println!("{}", colour_string);
    }
}
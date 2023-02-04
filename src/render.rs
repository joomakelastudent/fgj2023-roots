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

use game_consts;

use crate::gamestate::GameState;

//mutability is not needed for rendering
pub fn render(game_state: &GameState) {
    
    //background layer of framebuffer is the map 
    let framebuffer = &game_state.map;
    
    //draw player at location
    let player_location = &game_state.player_state.location;
    let player_sprite = '@';

    framebuffer[64*player_location.y + player_location.x] = player_sprite;

    
}

fn testcolours () {
    for i in 0..7{
        let index1 = 30+i;
        let index2 = 47-i;//perhaps bug?
        //let colour_string = f!("\x1b[{index1}m{index1} and {index2}\x1b[{index2}m").to_string();
        let colour_string = format!("\x1b[{}m{} and {}\x1b[{}m", index1,index1,index2,index2);
        
        println!("{}", colour_string);
    }
}
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

//mutability is not needed for rendering
fn render(&game_state) {
    let level = game_state.level;
    
}
use crate::gameconsts;

// structs that work to structure objects' internal data.
// the idea is to handle them through SOA for blazing speeds
pub struct GameState {
    pub map: Vec<char>,
    pub enemy_list: Vec<MoverState>,
    pub player: MoverState,
    pub enemy_spawn_cooldown: gameconsts::CooldownType
}

pub struct MoverState {
    pub movement_cooldown: gameconsts::CooldownType,
    pub x: gameconsts::CoordinateType,
    pub y: gameconsts::CoordinateType,
    
}

// much clearer to have this here instead of main.rs
// less "not pretty" initialization than before
pub fn initialize_gamestate(map: Vec<char>) -> GameState {
    GameState {
        map,
        enemy_list: vec![],
        player: MoverState {
            movement_cooldown: 0,
            x:5,
            y:5,
        },
        enemy_spawn_cooldown: 100,
    }
}


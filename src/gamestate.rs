/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

 /// List of (most) structs that are used in the game

pub struct GameState {
    pub map_state: MapState,
    pub control_state: ControlState,
    pub player_state: PlayerState,
    pub enemy_state: EnemyState,
}

pub struct MapState {
    pub data: Vec<char>,
}

pub enum ControlState {
    MENU,
    PAUSED,
    ACTION,
}

pub struct PlayerState {
    pub location: Location,
    pub health: Health,
    pub heading: i8, // North, south, east or west
    pub dash_cooldown: i32,
    pub attack_cooldown: i32,
}

pub struct Location {
    pub x: i32,
    pub y: i32,
}

pub struct Health {
    pub current: i32,
    pub max: i32,
}

pub struct EnemyState {
    pub action: EnemyAction,
    pub health: Health,
    pub species: EnemySpecies,
    pub location: Location,
    pub heading: i8,
}

pub enum EnemyAction {
    IDLE,
    MOVE_TO (Location),
    ATTACK,
    STUNNED,
}

pub enum EnemySpecies {
    MELEE,
    RANGED,
}
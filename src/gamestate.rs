/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

 // List of (most) structs that are used in the game

pub struct GameState {
    pub map: Vec<char>,
    pub control_state: ControlState,
    pub player_state: PlayerState,
    pub enemy_list: Vec<EnemyState>,
}

pub enum ControlState {
    MENU,
    PAUSED,
    ACTION,
}

pub struct PlayerState {
    pub location: Location,
    pub health: Health,
    pub heading: Heading,
    pub dash_cooldown: i32,
    pub attack_cooldown: i32,
    pub invis_frames: i32,
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
    pub heading: Heading,
}

pub struct AttackSquares {
    pub location: Location,
    pub heading: Heading,
    pub owner: Faction,
    pub damage: i8,
}

pub enum Faction {
    PLAYER,
    ENEMY,
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

pub enum Heading {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
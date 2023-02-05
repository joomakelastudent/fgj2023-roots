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
    pub enemy_spawner:EnemySpawner,
}

// Is the game paused, in a menu or playable normally?
pub enum ControlState {
    MENU,
    PAUSED,
    ACTION,
}

pub struct PlayerState {
    pub location: Location,
    pub health: Health,
    pub facing: Facing,
    pub moving: bool,
    pub movement_cooldown: i32,
    pub dash_cooldown: i32,
    pub attack_cooldown: i32,
    pub invis_frames: i32,
}

pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct Health {
    pub current: i32,
    pub max: i32,
}

pub struct EnemyState {
    //pub action: EnemyAction,
    //pub health: Health,
    //pub species: EnemySpecies,
    pub location: Location,
    //pub facing: Facing,
    //pub moving: bool,
    pub movement_cooldown: i32,
}

pub struct EnemySpawner {
    pub cooldown: usize,
}

pub struct AttackSquares {
    pub location: Location,
    pub facing: Facing,
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

pub enum Facing {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
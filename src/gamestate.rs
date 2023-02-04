/*******************************************************************************
 * Finnish Game Jam 2023-02-03 project of team Ryhmä Rämä
 * Project name: tty-throwdown
 * Authors: see the AUTHORS.md file
 * License: Apache 2.0 license
 * Repo: https://github.com/joomakelastudent/fgj2023-roots
 ******************************************************************************/

 /// List of (most) structs that are used in the game

struct GameState {
    map_state : MapState,
    player_state : PlayerState,
}

struct MapState {
    data: String,
}

struct PlayerState {
    location: Location,
    health: Health,
    dash_available: bool,
    dash_cooldown: i32,
}

struct Location {
    x: i32,
    y: i32,
}

struct EnemyState {
    behavior: EnemyBehavior,
    health: Health,
    species: EnemySpecies,
    location: Location,
}

enum EnemyBehavior {
    IDLE,
    MOVE_TO (Location),
    ATTACK,
}

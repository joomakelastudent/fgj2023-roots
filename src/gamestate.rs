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

struct GameState {
    map_state : MapState,
    player_state : PlayerState,
}

struct PlayerState {
    location : Location,
    health : Health,
    status : 
}

struct Location {
    x : i32,
    y : i32,
}

struct EnemyState {
    behavior : EnemyBehavior,
    health : Health,
    species : EnemySpecies,
    location : Location,
}

enum EnemyBehavior {
    IDLE,
    MOVE_TO (Location),
    ATTACK,
}

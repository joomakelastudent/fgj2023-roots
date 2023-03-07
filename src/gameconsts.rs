// terminal rendering maximum area that we can render
pub const MAX_FRAME_WIDTH: usize = 64;
pub const MAX_FRAME_HEIGHT: usize = 64;

// this is used a lot so size might matter for optimization
// also hard coding can be bad
pub type CoordinateType = i32;
pub type CooldownType = u32;
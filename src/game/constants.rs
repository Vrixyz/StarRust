use bevy::prelude::*;


// Bullet Constants
pub const BULLET_SPEED: f32 = 0.25;


// Wall Constants
pub const WALL_THICKNESS: f32 = 1.0;
// x coordinates_WALL
pub const LEFT_WALL: f32 = -30.;
pub const RIGHT_WALL: f32 = 30.;
// y coordinates
pub const BOTTOM_WALL: f32 = -17.;
pub const TOP_WALL: f32 = 17.;
pub const WALL_COLOR: Color = Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.03};
use serde::{Serialize, Deserialize};
use amethyst::core::math::Vector2;

/// Config struct for the arena settings read from a ron config file
// amethyst::config::Config is implemented for this automatically because
// it implements serde::{Serialize, Deserialize} which we derive
#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 100.0,
            width: 100.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BallConfig {
    pub velocity: Vector2<f32>,
    pub radius: f32,
    pub color: (f32, f32, f32, f32),
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            velocity: Vector2::new(75.0, 50.0),
            radius: 2.5,
            color: (1.0, 0.0, 0.0, 1.0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaddleConfig {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub color: (f32, f32, f32, f32),
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            width: 3.0,
            height: 12.0,
            speed: 1.2,
            color: (1.0, 0.0, 0.0, 1.0),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PongConfig {
    pub arena: ArenaConfig,
    pub ball: BallConfig,
    pub paddle: PaddleConfig,
}

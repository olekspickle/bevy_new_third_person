use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Config {
    pub scale: f32,
    pub geometry: Geometry,
    pub player: PlayerConfig,
}
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Geometry {
    pub main_plane: f32,
    pub quantity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct PlayerConfig {
    pub movement: Movement,
    pub hitbox: Hitbox,
    pub zoom: (f32, f32),
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Hitbox {
    pub radius: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Movement {
    pub actions_in_air: u8,
    pub dash_distance: f32,
    pub speed: f32,
}

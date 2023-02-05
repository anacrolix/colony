use bevy::prelude::*;
use super::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Position {
    pub fn to_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x as f32 * TILE_SIZE,
            self.y as f32 * TILE_SIZE,
            self.z as f32,
        )
    }
    pub fn to_transform_layer(&self, layer: f32) -> Transform {
        Transform::from_xyz(
            self.x as f32 * TILE_SIZE,
            self.y as f32 * TILE_SIZE,
            self.z as f32 + layer,
        )
    }
    pub fn distance(&self, other: &Position) -> i32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt() as i32
    }
}

#[derive(Component, PartialEq, Clone, Debug)]
pub enum TileType {
    Wall, Floor
}

#[derive(Component)]
pub struct Plant {
    pub growth: f32,
    pub plant_type: PlantType,
}

#[derive(Component, PartialEq)]
pub enum PlantType {
    BabyPineTree, PineTree, BabyOakTree, OakTree, BabyCedarTree, CedarTree, BabyBush, Bush, BabyBerryBush, BerryBush
}

#[derive(Component)]
pub struct Renderable {
    //pub glyph: rltk::FontCharType,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<Position>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
pub struct MoveRandom;

#[derive(Component)]
pub struct MonsterGenerator;

#[derive(Component)]
pub struct GeneratedBy {
    pub entity: Entity,
}
#[derive(Component)]
pub struct Targeting {
    pub target: Entity,
}

#[derive(Component)]
pub struct Pathing {
    pub path: Vec<Position>,
    pub destination: Position,
}

#[derive(Component)]
pub struct MoveTowardsTarget;

#[derive(Component)]
pub struct MoveTowardsNearestAttackable;

#[derive(Component)]
pub struct Attackable;

#[derive(Component)]
pub struct Mobile;

#[derive(Component)]
pub struct SizeXYZ {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
impl SizeXYZ {
    pub fn cube(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: x,
        }
    }
    pub fn flat(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: 0.1,
        }
    }
    pub fn flat_2(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: 1.0,
        }
    }
}
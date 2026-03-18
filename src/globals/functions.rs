use crate::globals::SIZE;
use bevy::prelude::*;

pub type GridPos = UVec2;
pub type WorldPos = Vec3;

pub fn world_to_grid(world_pos: &WorldPos) -> GridPos {
    GridPos::new(world_pos.x as u32, world_pos.z as u32)
}

pub fn grid_to_world(grid_pos: &GridPos) -> WorldPos {
    WorldPos::new(grid_pos.x as f32, 0., grid_pos.y as f32)
}

pub fn index_to_grid<T: Into<u32>>(index: T) -> GridPos {
    let s = SIZE as u32;
    let i = index.into();
    let x = i % s;
    let y = i / s;

    GridPos::new(x, y)
}

pub fn worldpos_to_transform(pos: WorldPos) -> Transform {
    Transform::from_xyz(pos.x, pos.y, pos.z)
}

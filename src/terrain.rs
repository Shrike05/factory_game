pub const SIZE: usize = 4;
pub const SIZE_SQUARED: usize = SIZE * SIZE;

mod plugin;
mod systems;
mod types;

pub use self::plugin::TerrainPlugin;
pub use self::types::*;

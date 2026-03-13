use std::fmt::write;

use crate::{
    factory::FactoryType,
    terrain::{SIZE, SIZE_SQUARED},
};
use bevy::{log::tracing_subscriber::fmt, prelude::*};
use bitmaps::Bitmap;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HoveredTile {
    pub pos: IVec2,
    pub hovering: bool,
}

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuildMessage {
    pub pos: IVec2,
}

#[derive(Resource, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct BuildabilityMap {
    pub map: Bitmap<SIZE_SQUARED>,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildSelection {
    Factory(FactoryType),
    Road,
    None,
}

impl BuildMessage {
    pub fn new(pos: IVec2) -> BuildMessage {
        BuildMessage { pos }
    }

    pub fn get_pos(&self) -> &IVec2 {
        &self.pos
    }
}

impl BuildabilityMap {
    fn pos_to_index(&self, x: i32, y: i32) -> usize {
        y as usize * SIZE + x as usize
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        if x >= SIZE as i32 || x < 0 || y >= SIZE as i32 || y < 0 {
            return false;
        }
        self.map.get(self.pos_to_index(x, y))
    }

    pub fn overlaps(&self, tiles: &Vec<IVec2>) -> bool {
        tiles.iter().any(|tile| self.get(tile.x, tile.y))
    }

    pub fn set_real(&mut self, pos: IVec2, val: bool) -> Result<()> {
        let index = self.pos_to_index(pos.x, pos.y);

        if index >= SIZE_SQUARED {
            error!("The position is outside the bounds of the bitmap")
        }
        self.map.set(index, val);

        Ok(())
    }
}

impl std::fmt::Display for BuildabilityMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = "".to_string();
        for i in 0..SIZE_SQUARED {
            let a = self.map.get(i);

            res += if i % SIZE == 0 { "\n" } else { "" };
            res += if a { "X" } else { "#" };
        }
        write!(f, "{}", res)
    }
}

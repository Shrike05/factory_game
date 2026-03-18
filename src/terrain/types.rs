use crate::factory::FactoryType;
use crate::globals::*;
use bevy::prelude::*;
use bitmaps::Bitmap;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HoveredTile {
    pub pos: GridPos,
    pub hovering: bool,
}

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuildMessage {
    pub pos: GridPos,
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
    pub fn new(pos: GridPos) -> BuildMessage {
        BuildMessage { pos }
    }

    pub fn get_pos(&self) -> &GridPos {
        &self.pos
    }
}

impl BuildabilityMap {
    fn pos_to_index(&self, x: u32, y: u32) -> usize {
        y as usize * SIZE + x as usize
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        if x >= SIZE as u32 || y >= SIZE as u32 {
            return false;
        }
        self.map.get(self.pos_to_index(x, y))
    }

    pub fn overlaps(&self, tiles: &[GridPos]) -> bool {
        tiles.iter().any(|tile| self.get(tile.x, tile.y))
    }

    pub fn set_real(&mut self, pos: GridPos, val: bool) -> Result<()> {
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

use crate::LoadState;
use crate::globals::GridPos;
use crate::terrain::systems::*;
use crate::terrain::types::*;
use bevy::prelude::*;
use bitmaps::Bitmap;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildabilityMap { map: Bitmap::new() });
        app.insert_resource(HoveredTile {
            pos: GridPos::new(0, 0),
            hovering: false,
        });
        app.add_systems(OnEnter(LoadState::Ready), spawn_terrain);
        app.add_message::<BuildMessage>();
        app.add_systems(Update, build_event.run_if(in_state(LoadState::Ready)));
    }
}

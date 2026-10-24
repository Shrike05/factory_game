use crate::globals::world_to_grid;
use crate::preview::types::*;
use crate::road::{Road, RoadConstructor};
use crate::states::BuildSelection;
use crate::terrain::{BuildabilityMap, HoveredTile};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct RoadPreviewParams<'w, 's> {
    pub query: Query<'w, 's, (Entity, &'static mut Transform, &'static PreviewRoad)>,
    pub build_map: Res<'w, BuildabilityMap>,
    pub prev_mat: Res<'w, PreviewAssets>,
    pub build_select: Res<'w, State<BuildSelection>>,
    pub road_constructor: Res<'w, RoadConstructor>,
    pub hovered_tile: Res<'w, HoveredTile>,
}

pub fn preview_road(mut commands: Commands, params: RoadPreviewParams) {
    for (entity, _, _) in params.query {
        commands.entity(entity).despawn();
    }

    let mut rc = params.road_constructor.clone();
    rc.set_end_coordinate(params.hovered_tile.pos);
    let road = Road::new(rc.get_list()).unwrap();
    let path_vec = road
        .build_road_scenes(&params.build_map, Some(Color::srgb(1., 1., 1.)))
        .unwrap_or(vec![]);
    let mut path = path_vec.into_iter();

    for (pos, scene) in path {
        commands.spawn_scene(bsn! {
            Transform::from_xyz(pos.x as f32, 0., pos.y as f32)
            Pickable::IGNORE
            PreviewRoad
            @scene
        });
    }
}

pub fn stop_build_road(
    mut road_constructor: ResMut<RoadConstructor>,
    pre_query: Query<Entity, With<PreviewRoad>>,
    mut commands: Commands,
) {
    for entity in pre_query.iter() {
        commands.entity(entity).despawn();
    }

    *road_constructor = RoadConstructor::empty();
}

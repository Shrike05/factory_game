use crate::factory::*;
use crate::globals::*;
use crate::road::{BuildRoadMessage, Road};
use crate::states::BuildSelection;
use crate::terrain::{BuildabilityMap, HoveredTile};
use bevy::prelude::*;
use std::fs;
use std::path::Path;

pub fn production_cycle(mut production: ResMut<Production>) {
    production.set_inventory_item(Item::Rock, 10.);
    production.production_cycle();
}

pub fn remove_factory(
    mut commands: Commands,
    query: Query<Entity, With<Factory>>,
    mut prod: ResMut<Production>,
    mut event: MessageReader<RemoveFactoryMessage>,
    road_queries: Query<(Entity, &Road)>,
) {
    for ev in event.read() {
        let mut en = commands.entity(ev.entity);
        en.despawn();
        prod.remove_factory(ev.entity);
        for (road_entity, road) in road_queries.iter() {
            if road.start_entity == ev.entity || road.end_entity == ev.entity {
                commands.entity(road_entity).despawn();
            }
        }
    }
}

pub fn spawn_factories(
    mut commands: Commands,
    mut production: ResMut<Production>,
    mut msg: MessageReader<NewFactoryEvent>,
) {
    for message in msg.read() {
        let mut entity = commands.spawn_scene(create_factory(message.pos, message.factory_type));

        let entity_id = entity.id();

        match message.factory_type {
            FactoryType::Sink => {
                entity.insert(Sink);
                production.add_sink(entity_id);
            }
            FactoryType::Source => {
                entity.insert(Source);
                production.add_source(entity_id);
            }
            _ => {
                production.add_factory(entity_id, Factory::new(message.pos, message.factory_type));
            }
        };

        entity.observe(
            move |event: On<PointerClick>,
                  query: Query<&Transform>,
                  mut msg: MessageWriter<BuildRoadMessage>,
                  build_selection: Res<State<BuildSelection>>,
                  mut remove_writer: MessageWriter<RemoveFactoryMessage>| {
                on_factory_click(event, query, msg, build_selection, entity_id, remove_writer)
            },
        );
    }
}

fn on_factory_click(
    event: On<PointerClick>,
    query: Query<&Transform>,
    mut msg: MessageWriter<BuildRoadMessage>,
    build_selection: Res<State<BuildSelection>>,
    entity_id: Entity,
    mut remove_writer: MessageWriter<RemoveFactoryMessage>,
) {
    if let Ok(transform) = query.get(event.event_target()) {
        match **build_selection {
            BuildSelection::Road => {
                msg.write(BuildRoadMessage::End(
                    world_to_grid(&transform.translation),
                    entity_id,
                ));
            }
            BuildSelection::Factory(_) | BuildSelection::None => {
                msg.write(BuildRoadMessage::Start(
                    world_to_grid(&transform.translation),
                    entity_id,
                ));
            }
            BuildSelection::Removing => {
                remove_writer.write(RemoveFactoryMessage { entity: entity_id });
            }
        }
    }
}

pub fn create_factory(pos: GridPos, fac_type: FactoryType) -> impl Scene {
    bsn! {
        Factory::new(pos, fac_type)
        Mesh3d(asset_value(Cuboid::new(0.8, 1., 0.8)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 1., 1.)))
        Transform::from_xyz(pos.x as f32, 0., pos.y as f32)

        on(move |event: On<PointerOver>, mut query: Query<&Transform>, mut hovered_tile: ResMut<HoveredTile>| {
            if let Ok(transform) = query.get_mut(event.event_target()) {
                hovered_tile.pos = world_to_grid(&transform.translation);
                hovered_tile.hovering = true;
            }
        })
    }
}

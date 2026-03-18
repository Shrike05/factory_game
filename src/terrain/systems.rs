use crate::factory::{FactoryType, NewFactoryEvent, get_factory_attributes};
use crate::globals::*;
use crate::road::*;
use crate::states::BuildSelection;
use crate::terrain::types::BuildMessage;
use crate::terrain::*;
use bevy::prelude::*;

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::new(Vec3::new(0., 1., 0.), Vec2::new(0.5, 0.5)));
    let material = materials.add(Color::srgb(0.25, 0.25, 0.25));
    let hover_material = materials.add(Color::srgb(0.1, 0.1, 0.1));

    for i in 0..(SIZE * SIZE) {
        let pos = grid_to_world(&index_to_grid(i as u32));

        commands
            .spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                worldpos_to_transform(pos),
            ))
            .observe(update_tile::<Pointer<Over>>(hover_material.clone(), true))
            .observe(update_tile::<Pointer<Out>>(material.clone(), false))
            .observe(tile_clicked::<Pointer<Click>>());
    }
}

pub fn update_tile<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
    hovering: bool,
) -> impl Fn(On<E>, Query<(&mut MeshMaterial3d<StandardMaterial>, &Transform)>, ResMut<HoveredTile>)
{
    move |event, mut query, mut hovered_tile| {
        if let Ok((mut material, transform)) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
            hovered_tile.pos = world_to_grid(&transform.translation);
            hovered_tile.hovering = hovering;
        }
    }
}

pub fn tile_clicked<E: EntityEvent>()
-> impl Fn(On<E>, Query<&Transform>, MessageWriter<BuildMessage>) {
    move |event, mut query, mut msg| {
        if let Ok(transform) = query.get_mut(event.event_target()) {
            msg.write(BuildMessage::new(world_to_grid(&transform.translation)));
        }
    }
}

pub fn build_event(
    mut ev: MessageReader<BuildMessage>,
    mut fac_writer: MessageWriter<NewFactoryEvent>,
    mut road_writer: MessageWriter<BuildRoadMessage>,
    build_map: Res<BuildabilityMap>,
    mut road_constructor: ResMut<RoadConstructor>,
    build_selection: Res<State<BuildSelection>>,
) {
    for build_ev in ev.read() {
        let tiles: Vec<GridPos> = get_factory_attributes(&FactoryType::Empty)
            .shape
            .iter()
            .map(|x| x + build_ev.get_pos())
            .collect();

        match **build_selection {
            BuildSelection::Factory(fac_type) => {
                if !build_map.overlaps(&tiles) {
                    fac_writer.write(NewFactoryEvent::new(*build_ev.get_pos(), fac_type));
                }
            }
            BuildSelection::Road => {
                if road_constructor.get_start().is_some() {
                    road_writer.write(BuildRoadMessage::End(*build_ev.get_pos()));
                    road_constructor.set_end(*build_ev.get_pos());
                } else {
                    road_writer.write(BuildRoadMessage::Start(*build_ev.get_pos()));
                    road_constructor.set_start(*build_ev.get_pos());
                }
            }
            _ => (),
        }
    }
}

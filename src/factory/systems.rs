use crate::factory::{types::Factory, *};
use crate::states::*;
use bevy::prelude::*;
use bevy_terrain::*;

pub fn create_factory_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(FactoryAssets {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::srgb(1., 1., 1.)),
    });
}

pub fn spawn_factories(
    mut commands: Commands,
    fac_assets: Res<FactoryAssets>,
    mut msg: MessageReader<NewFactoryEvent>,
    mut build_map: ResMut<BuildabilityMap>,
    fac_map: Res<FactoryMap>,
) {
    for message in msg.read() {
        let factory = Factory::new(message.pos, message.factory_type);

        commands.spawn((
            factory,
            Mesh3d(fac_assets.mesh.clone()),
            MeshMaterial3d(fac_assets.material.clone()),
            Transform::from_xyz(factory.origin.x as f32, 0., factory.origin.y as f32),
        ));

        let shape: Box<[GridPos]> = fac_map.shapes[&factory.factory_type].clone();
        for offset in shape {
            build_map
                .set_real(factory.origin + offset, true)
                .expect("Couldn't set factory to the build_map");
        }
    }
}

pub fn build_factory_event(
    mut ev: MessageReader<TileClickedMessage>,
    mut fac_writer: MessageWriter<NewFactoryEvent>,
    fac_map: Res<FactoryMap>,
    build_map: Res<BuildabilityMap>,
    build_selection: Res<State<BuildSelection>>,
) {
    for build_ev in ev.read() {
        let tiles: Vec<GridPos> = fac_map.shapes[&FactoryType::Empty]
            .iter()
            .map(|x| x + build_ev.get_pos())
            .collect();

        if !build_map.overlaps(&tiles)
            && let BuildSelection::Factory(fac_type) = **build_selection
        {
            fac_writer.write(NewFactoryEvent::new(*build_ev.get_pos(), fac_type));
        }
    }
}

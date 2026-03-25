use crate::factory::{FactoryMap, FactoryType, NewFactoryEvent};
use crate::road::*;
use crate::states::BuildSelection;
use bevy::prelude::*;
use bevy_terrain::*;

pub fn build_event(
    mut ev: MessageReader<TileClickedMessage>,
    mut fac_writer: MessageWriter<NewFactoryEvent>,
    mut road_writer: MessageWriter<BuildRoadMessage>,
    fac_map: Res<FactoryMap>,
    build_map: Res<BuildabilityMap>,
    mut road_constructor: ResMut<RoadConstructor>,
    build_selection: Res<State<BuildSelection>>,
) {
    for build_ev in ev.read() {
        let tiles: Vec<GridPos> = fac_map.shapes[&FactoryType::Empty]
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

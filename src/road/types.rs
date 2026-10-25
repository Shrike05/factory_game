use crate::globals::*;
use bevy::prelude::*;

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildRoadMessage {
    Start(GridPos, Entity),
    WayPoint(GridPos),
    End(GridPos, Entity),
}

//This Struct
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RemoveRoadMessage {
    pub entity: Entity,
}

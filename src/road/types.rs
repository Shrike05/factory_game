use crate::globals::*;
use crate::terrain::BuildabilityMap;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Resource, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoadConstructor {
    start: Option<GridPos>,
    waypoints: Vec<GridPos>,
    end: Option<GridPos>,
    start_entity: Option<Entity>,
    end_entity: Option<Entity>,
}

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildRoadMessage {
    Start(GridPos, Entity),
    WayPoint(GridPos),
    End(GridPos, Entity),
}

#[allow(unused)]
impl RoadConstructor {
    pub fn new(start: GridPos, waypoints: Vec<GridPos>, end: GridPos) -> Self {
        RoadConstructor {
            start: Some(start),
            waypoints,
            end: Some(end),
            start_entity: None,
            end_entity: None,
        }
    }

    pub fn empty() -> Self {
        RoadConstructor {
            start: None,
            waypoints: vec![],
            end: None,
            start_entity: None,
            end_entity: None,
        }
    }

    pub fn start_new(start: GridPos) -> Self {
        RoadConstructor {
            start: Some(start),
            waypoints: vec![],
            end: None,
            start_entity: None,
            end_entity: None,
        }
    }

    pub fn add_waypoints(&mut self, points: &[GridPos]) {
        points.iter().for_each(|point| self.waypoints.push(*point));
    }

    pub fn get_start(&self) -> (&Option<GridPos>, &Option<Entity>) {
        (&self.start, &self.start_entity)
    }
    pub fn get_end(&self) -> (&Option<GridPos>, &Option<Entity>) {
        (&self.end, &self.end_entity)
    }
    pub fn get_waypoints(&self) -> &Vec<GridPos> {
        &self.waypoints
    }
    pub fn set_start(&mut self, start: GridPos, entity: Entity) {
        self.start = Some(start);
        self.start_entity = Some(entity)
    }
    pub fn set_end(&mut self, end: GridPos, entity: Entity) {
        self.end = Some(end);
        self.end_entity = Some(entity)
    }

    pub fn set_end_coordinate(&mut self, end: GridPos) {
        self.end = Some(end);
    }

    pub fn finished(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    pub fn get_list(&self) -> Vec<GridPos> {
        let mut result = self.waypoints.clone();

        if let Some(end) = self.end {
            result.push(end);
        }

        if let Some(start) = self.start {
            result.insert(0, start);
        }

        result
    }
}

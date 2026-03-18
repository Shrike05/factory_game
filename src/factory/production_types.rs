use crate::factory::types::*;
use bevy::prelude::*;

type Inventory<'w, 's> = Query<'w, 's, ItemComponent>;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemComponent {
    item_type: ItemType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    Empty,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Source {
    item_type: ItemType,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sink;

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct Productions {
    productions: Vec<Production>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Production;

impl Production {
    fn validate<'w, 's>(factories_query: &Query<'w, 's, &Factory>, factory_id: FactoryId) {
        let factory = factories_query
            .get(*factory_id.get())
            .expect("FactoryId doesn't have corresponding Factory");
        //Base case

        //Recurse
    }
}

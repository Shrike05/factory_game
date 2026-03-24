use crate::factory::*;
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
    fn validate<'w, 's>(factories_query: Query<'w, 's, &Factory>, factory_id: &FactoryId) -> bool {
        let factory = factories_query
            .get(*factory_id.get())
            .expect("FactoryId doesn't have corresponding Factory");

        //Base case
        let valid = match factory.get_name().as_string().as_str() {
            "Sink" => factory.outbound.is_empty() && !factory.inbound.is_empty(),
            "Source" => factory.inbound.is_empty() && !factory.outbound.is_empty(),
            _ => !factory.inbound.is_empty() && !factory.outbound.is_empty(),
        };

        //Recurse
        let inbound_valid = valid
            && factory
                .inbound
                .iter()
                .all(|x| Production::validate(factories_query, x));

        let outbound_valid = valid
            && factory
                .outbound
                .iter()
                .all(|x| Production::validate(factories_query, x));

        valid && inbound_valid && outbound_valid
    }
}

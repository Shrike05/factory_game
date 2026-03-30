use crate::factory::{defs::FactoryDef, types::*};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_defs_loader::LoadedDefs;
use bevy_terrain::GridPos;

#[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
pub struct FactoryShapes {
    shape: HashMap<FactoryName, Box<[GridPos]>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
pub struct FactoryMeshes {
    mesh: HashMap<FactoryName, Handle<Mesh>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
pub struct FactoryMaterials {
    material: HashMap<FactoryName, Handle<StandardMaterial>>,
}

impl FactoryShapes {
    pub fn get_grid_tiles(&self, pos: &GridPos, factory_name: &FactoryName) -> Vec<GridPos> {
        let shape = self.shape.get(factory_name).expect("Factory Doesn't Exist");
        shape.iter().map(|x| *x + *pos).collect()
    }
}

pub trait FactoryAttribute {
    type Value;

    fn get(&self, key: &FactoryName) -> &Self::Value;
    fn set(&mut self, key: FactoryName, value: Self::Value);
    fn get_map(&self) -> &HashMap<FactoryName, Self::Value>;
    fn set_map(&mut self, value: HashMap<FactoryName, Self::Value>);
}

macro_rules! impl_factory_resource {
    ($struct_name:ident, $field_name:ident, $value_type:ty) => {
        impl FactoryAttribute for $struct_name {
            type Value = $value_type;

            fn get(&self, key: &FactoryName) -> &Self::Value {
                self.$field_name
                    .get(key)
                    .expect("Factory does not have a Shape")
            }

            fn set(&mut self, key: FactoryName, value: Self::Value) {
                self.$field_name.insert(key, value);
            }

            fn get_map(&self) -> &HashMap<FactoryName, Self::Value> {
                &self.$field_name
            }
            fn set_map(&mut self, value: HashMap<FactoryName, Self::Value>) {
                self.$field_name = value;
            }
        }
    };
}

impl_factory_resource!(FactoryShapes, shape, Box<[GridPos]>);
impl_factory_resource!(FactoryMeshes, mesh, Handle<Mesh>);
impl_factory_resource!(FactoryMaterials, material, Handle<StandardMaterial>);

pub fn init_attributes(mut shapes: ResMut<FactoryShapes>, defs: Res<LoadedDefs<FactoryDef>>) {
    for def in &defs.0 {
        let name = FactoryName::from_string(&def.name);
        shapes.set(name, def.shape.clone().into_boxed_slice());
    }
}

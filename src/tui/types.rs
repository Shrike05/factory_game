use bevy::prelude::*;

#[derive(Resource, Clone, Default, Debug, PartialEq, Eq)]
pub struct TUIInput {
    input: String,
}

impl TUIInput {
    pub fn add<T: Into<String>>(&mut self, addition: T) {
        self.input += &addition.into();
    }

    pub fn get(&self) -> &str {
        &self.input
    }
}

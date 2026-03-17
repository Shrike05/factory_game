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

    pub fn pop_command(&mut self) {
        self.input = "".to_string();
    }
}

#[derive(Message, Clone, Debug, Default, PartialEq, Eq)]
pub struct TUICommand {
    args: Vec<String>,
}

impl TUICommand {
    pub fn new(args: Vec<String>) -> Self {
        TUICommand { args }
    }
}

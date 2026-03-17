use bevy::prelude::*;

const HISTORY: usize = 20;

#[derive(Resource, Clone, Default, Debug, PartialEq, Eq)]
pub struct TUIInput {
    old_commands: Vec<String>,
    input: String,
}

impl TUIInput {
    pub fn add<T: Into<String>>(&mut self, addition: T) {
        self.input += &addition.into();
    }

    pub fn get(&self) -> &str {
        &self.input
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.old_commands
    }

    pub fn pop_command(&mut self) {
        self.old_commands.insert(0, self.input.clone());
        while self.old_commands.len() > HISTORY {
            self.old_commands.pop();
        }
        self.input = "".to_string();
    }

    pub fn remove_char(&mut self) {
        self.input.pop();
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

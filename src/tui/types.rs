use bevy::prelude::*;
use std::str::FromStr;
use tui_logger::TuiWidgetState;

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
        self.old_commands.push(self.input.clone());
        while self.old_commands.len() > HISTORY {
            self.old_commands.remove(0);
        }
        self.input = "".to_string();
    }

    pub fn remove_char(&mut self) {
        self.input.pop();
    }

    pub fn reset(&mut self) {
        self.old_commands = vec![];
        self.input = "".to_string();
    }
}

#[derive(Message, Clone, Debug, Default, PartialEq, Eq)]
pub struct TUICommand {
    input: String,
    args: Vec<String>,
}

impl TUICommand {
    pub fn new<T: Into<String>>(into_input: T) -> Self {
        let input = into_input.into();
        let args = input.split_whitespace().map(|x| x.to_string()).collect();
        TUICommand { input, args }
    }

    pub fn parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.input.parse::<T>()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClearCommand;

impl FromStr for ClearCommand {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.to_lowercase().trim().eq("clear") {
            Ok(ClearCommand)
        } else {
            Err(format!("Couldn't parse ClearCommand, input: {}", s))
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrintEntities;

impl FromStr for PrintEntities {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.to_lowercase().trim().eq("entities") {
            Ok(PrintEntities)
        } else {
            Err(format!("Couldn't parse PrintEntities, input: {}", s))
        }
    }
}

#[derive(Resource)]
pub struct LogState {
    state: TuiWidgetState,
}

impl Default for LogState {
    fn default() -> Self {
        Self {
            // This initializes the state with default scroll/filter settings
            state: TuiWidgetState::new(),
        }
    }
}

impl LogState {
    pub fn get(&self) -> &TuiWidgetState {
        &self.state
    }

    pub fn get_mut(&mut self) -> &mut TuiWidgetState {
        &mut self.state
    }

    pub fn set(&mut self, x: TuiWidgetState) {
        self.state = x;
    }
}

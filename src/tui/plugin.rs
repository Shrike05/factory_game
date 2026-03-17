use crate::tui::systems::*;
use crate::tui::types::*;
use bevy::prelude::*;
use bevy_ratatui::RatatuiPlugins;

pub struct TUIPlugin;

impl Plugin for TUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RatatuiPlugins::default());
        app.add_message::<TUICommand>();
        app.insert_resource(TUIInput::default());
        app.add_systems(Update, (draw_system, input_system));
        app.add_systems(Update, (clear_command, print_entities_command));
    }
}

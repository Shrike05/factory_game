use crate::factory::FactoryType;
use crate::ui::{
    hotbar::{tool_bar, update_button_flashes, update_button_hover_styles},
    root::root_ui,
};
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::states::BuildSelection;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, root_ui.spawn());
        app.add_systems(Update, (update_button_hover_styles, update_button_flashes));
    }
}

use bevy::prelude::*;

use crate::ui::{
    menu::DetailRoot,
    types::{PanelState, SlidePanel},
};

#[derive(States, Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub enum UiState {
    #[default]
    GameView,
    MenuView,
}

pub fn enter_menu_state(
    mut slide_panels: Query<&mut SlidePanel>,
    mut detail_root: Query<(&mut Visibility, &mut Pickable), With<DetailRoot>>,
) {
    for mut panel in slide_panels {
        panel.state = PanelState::Closed;
    }

    for (mut vis, mut pick) in detail_root.iter_mut() {
        *vis = Visibility::Visible;
        *pick = Pickable::default();
    }
}

pub fn exit_menu_state(mut detail_root: Query<(&mut Visibility, &mut Pickable), With<DetailRoot>>) {
    for (mut vis, mut pick) in detail_root.iter_mut() {
        *vis = Visibility::Hidden;
        *pick = Pickable::IGNORE;
    }
}

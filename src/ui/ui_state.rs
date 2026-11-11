use bevy::prelude::*;

use crate::ui::{
    menu::DetailRoot,
    types::{PanelState, SlidePanel},
};

#[derive(States, Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub enum UiState {
    #[default]
    GameView,
    MenuView(MenuState),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MenuState {
    Settings,
    Inventory,
    SkillTree,
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InMenuView;

impl ComputedStates for InMenuView {
    type SourceStates = UiState;

    fn compute(source: UiState) -> Option<Self> {
        match source {
            UiState::MenuView(_) => Some(InMenuView), // Matches any inner variant
            _ => None,
        }
    }
}

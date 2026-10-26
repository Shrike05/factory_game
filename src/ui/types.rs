use bevy::prelude::*;

/// Component to track smooth hover interpolation
#[derive(Component, Clone)]
pub struct SmoothButtonAnimation {
    pub current_scale: f32,
    pub target_scale: f32,
    pub current_y: f32,
    pub target_y: f32,
}

impl Default for SmoothButtonAnimation {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            target_scale: 1.0,
            current_y: 0.0,
            target_y: 0.0,
        }
    }
}

#[derive(Component)]
pub struct ButtonFlashTimer(pub Timer);

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PanelState {
    #[default]
    Open,
    Closed,
}

#[derive(Component, Clone, Default)]
pub struct SlidePanel {
    pub name: String,
    pub state: PanelState,
    pub width: f32,
    pub transition_speed: f32,
    pub to_right: bool,
}

impl SlidePanel {
    pub fn new(name: String, width: f32, to_right: bool) -> Self {
        SlidePanel {
            name,
            to_right,
            width,
            ..default()
        }
    }
}

// Marker component for the arrow text so we can toggle it easily
#[derive(Component, Clone, Default)]
pub struct PopoutButton(pub String);

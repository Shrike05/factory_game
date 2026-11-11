use std::path::Path;

use crate::factory::FactoryType;
use bevy::prelude::*;

pub const HEADING1_FONT_SIZE: f32 = 25.;
pub const HEADING2_FONT_SIZE: f32 = 20.;
pub const HEADING3_FONT_SIZE: f32 = 18.;
pub const SMALL_FONT_SIZE: f32 = 10.;

pub const PRIMARY_BASE: Color = Color::BLACK;
pub const SECONDARY_BASE: Color = Color::srgb(0.039, 0.039, 0.043);

pub const BORDER_COLOR: Color = Color::srgb(0.440, 0.440, 0.494);
pub const SECONDARY_BORDER_COLOR: Color = Color::srgb(0.133, 0.133, 0.157);
pub const HIGHLIGHT_BORDER_COLOR: Color = Color::srgb(0.922, 0.922, 0.961);

pub const PRIMARY_ACCENT: Color = Color::srgb(0.820, 0.820, 0.839);
pub const SECONDARY_ACCENT: Color = Color::srgb(0.557, 0.557, 0.576);

pub const HIGHLIGHT: Color = Color::WHITE;

pub const FONT_COLOR: Color = Color::srgb(0.886, 0.886, 0.906);
pub const SECONDARY_FONT_COLOR: Color = Color::srgb(0.608, 0.608, 0.631);

pub const FONT_PATH: &str = "fonts/Share_Tech_Mono/ShareTechMono-Regular.ttf";

/// Component attached to static grid palette items
/// Holds the factory type this item provides when dragged.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PaletteItem(pub FactoryType);

#[derive(Component, Clone, Default)]
pub struct DragGhost;

/// Component to track smooth hover interpolation
#[derive(Component, Clone)]
pub struct SmoothButtonAnimation {
    pub current_scale: f32,
    pub target_scale: f32,
    pub current_y: f32,
    pub target_y: f32,
    pub animate_up: bool,
}

impl Default for SmoothButtonAnimation {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            target_scale: 1.0,
            current_y: 0.0,
            target_y: 0.0,
            animate_up: true,
        }
    }
}

impl SmoothButtonAnimation {
    pub fn animate_down() -> Self {
        Self {
            current_scale: 1.0,
            target_scale: 1.0,
            current_y: 0.0,
            target_y: 0.0,
            animate_up: false,
        }
    }
}

#[derive(Component)]
pub struct ButtonFlashTimer(pub Timer);

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PanelState {
    Open,
    #[default]
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
            state: PanelState::default(),
            transition_speed: 10.,
        }
    }
}

// Marker component for the arrow text so we can toggle it easily
#[derive(Component, Clone, Default)]
pub struct PopoutButton(pub String);

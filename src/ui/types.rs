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

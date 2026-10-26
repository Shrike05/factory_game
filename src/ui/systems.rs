use crate::ui::types::*;
use bevy::{picking::hover::Hovered, prelude::*};

/// System that smoothly interpolates button scale and translation towards targets each frame
pub fn update_button_hover_styles(
    time: Res<Time>,
    mut query: Query<(&Hovered, &mut UiTransform, &mut SmoothButtonAnimation)>,
) {
    let dt = time.delta_secs();
    // Adjust speed coefficient (e.g., 15.0) to make the animation faster or slower
    let speed = 15.0;

    for (hovered, mut ui_transform, mut anim) in &mut query {
        // Update targets based on hover state
        if hovered.get() {
            anim.target_scale = 1.1;
            anim.target_y = -6.0; // Hover up distance
        } else {
            anim.target_scale = 1.0;
            anim.target_y = 0.0;
        }

        // Smoothly interpolate (lerp) towards target values
        anim.current_scale += (anim.target_scale - anim.current_scale) * (dt * speed).min(1.0);
        anim.current_y += (anim.target_y - anim.current_y) * (dt * speed).min(1.0);

        // Apply to the UI transform
        ui_transform.scale = Vec2::splat(anim.current_scale);
        ui_transform.translation = Val2::px(0.0, anim.current_y);
    }
}

pub fn update_button_flashes(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackgroundColor, &mut ButtonFlashTimer)>,
) {
    let dt = time.delta_secs();
    for (entity, mut bg, mut flash_timer) in &mut query {
        flash_timer.0.tick(std::time::Duration::from_secs_f32(dt));

        if flash_timer.0.is_finished() {
            // Revert back to the normal button color
            *bg = BackgroundColor(Color::srgb(0.25, 0.25, 0.3));
            // Remove the timer component so this system stops tracking it until the next click
            commands.entity(entity).remove::<ButtonFlashTimer>();
        }
    }
}

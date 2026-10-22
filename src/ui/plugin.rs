use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tool_bar.spawn());
        app.add_systems(Update, (update_button_hover_styles, update_button_flashes));
    }
}

/// Spawns a toolbar covering the lower part of the screen with interactive child buttons.
pub fn tool_bar() -> impl Scene {
    bsn! {
        // Toolbar container positioned at the bottom of the screen
        Node {
            position_type: PositionType::Absolute,
            bottom: px(20.),
            left: percent(25.),
            width: percent(50.),
            height: px(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: px(15.),
            padding: UiRect::all(px(10.)),
            border_radius: BorderRadius::all(px(12.)),
        }
        BackgroundColor(Color::srgba(0.12, 0.12, 0.15, 0.85))
        Children [
            @toolbar_button("Tool 1") --
            @toolbar_button("Tool 2") --
            @toolbar_button("Tool 3")
        ]
    }
}

/// Reusable button scene definition with hover and click behaviors
fn toolbar_button(label: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            width: px(110.),
            height: px(50.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(8.)),
        }
        BackgroundColor(Color::srgb(0.25, 0.25, 0.3))
        UiTransform {
            scale: Vec2::ONE,
            translation: Val2::px(0.0, 0.0),
        }
        Hovered::default()
        SmoothButtonAnimation::default()

        // Handle click event via bevy_ui_widgets::Button's Activate event
        on(|activate: On<Activate>, mut commands: Commands, mut query: Query<&mut BackgroundColor>| {
            if let Ok(mut bg) = query.get_mut(activate.entity) {
                *bg = BackgroundColor(Color::srgb(1.0, 0.6, 0.2)); // Flash color

                // Insert a 150ms timer to revert the color automatically
                commands.entity(activate.entity).insert(
                    ButtonFlashTimer(Timer::from_seconds(0.15, TimerMode::Once))
                );
            }
        })

        Children [
            Text(label)
            TextColor(Color::WHITE)
        ]
    }
}

/// Component to track smooth hover interpolation
#[derive(Component, Clone)]
pub struct SmoothButtonAnimation {
    current_scale: f32,
    target_scale: f32,
    current_y: f32,
    target_y: f32,
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

/// System that smoothly interpolates button scale and translation towards targets each frame
fn update_button_hover_styles(
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

/// Component added temporarily during a click flash
#[derive(Component)]
pub struct ButtonFlashTimer(pub Timer);

/// System that reverts the button color back to normal when the flash timer expires
fn update_button_flashes(
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

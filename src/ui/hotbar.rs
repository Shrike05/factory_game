use bevy::{
    log::tracing_subscriber::fmt::format,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::factory::FactoryType;
use crate::states::BuildSelection;

pub fn tool_bar() -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            row_gap: px(8.0),
            column_gap: px(8.0),
            padding: UiRect {
                left: px(10.0),
                right: px(10.0),
                top: px(10.0),
                bottom: px(10.0),
            },
            border: UiRect {
                left: px(2.0),
                right: px(2.0),
                top: px(2.0),
                bottom: px(2.0),
            },
        }
        BorderColor::all(Color::BLACK)
        BackgroundColor(Color::BLACK)
        Children[
            {(0..10).map(|x|{
                bsn! {
                 @toolbar_button(x)
                }
            }).collect::<Vec<_>>()}
        ]
    }
}

fn toolbar_button(n: usize) -> impl Scene {
    bsn! {
        Node {
            width: px(80.0),
            height: px(80.0),
            display: Display::Flex,
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.)),
        }
        SmoothButtonAnimation::default()
        Hovered::default()
        UiTransform{
            scale: Vec2::ONE,
            translation: Val2::px(0., 0.)
        }
        BorderColor::all(Color::srgb(0.239, 0.322, 0.471))
        Children [
            Node {
                position_type: PositionType::Absolute,
                left: px(6.0),
                top: px(4.0),
            }
            TextColor(Color::srgb(0.392, 0.455, 0.545))
            TextFont { font_size: FontSize::Px(10.) }
            Children [
                Text::new(format!("{n}"))
            ]
            --
            Node {
                width: percent(100.0),
                height: percent(100.0),
                display: Display::Flex,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            Children [
                Node::default()
                TextFont { font_size: FontSize::Px(22.) }
                Children [
                    Text::new(format!("F{n}"))
                ]
            ]
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

/// Component added temporarily during a click flash
#[derive(Component)]
pub struct ButtonFlashTimer(pub Timer);

/// System that reverts the button color back to normal when the flash timer expires
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

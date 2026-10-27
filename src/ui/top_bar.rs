use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use crate::ui::types::SmoothButtonAnimation;
use crate::ui::ui_state::UiState;

pub fn top_bar() -> impl Scene {
    bsn! {
        // Unrepresentable CSS: 'transform:' (2D matrix transforms require a separate Transform component)
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(10.0),
            height: px(60.),
            padding: UiRect {
                left: px(12.0),
                right: px(12.0),
                top: px(5.0),
                bottom: px(5.0),
            },
            border: UiRect {
                left: px(2.0),
                right: px(2.0),
                top: px(2.0),
                bottom: px(2.0),
            },
        }
        BackgroundColor(Color::BLACK)
        BorderColor::all(Color::srgb(0.165, 0.231, 0.361))
        ZIndex(5)
        Children[
            @top_button("Inventory".into())
            --
            @top_button("Skill Tree".into())
            --
            @top_button("Settings".into())
            --
        ]
    }
}

fn top_button(name: String) -> impl Scene {
    bsn! {
        Button
        // Unrepresentable CSS: 'cursor:' (Cursor styling is managed at the Window level or via custom interaction plugins)
        Node {
            padding: UiRect {
                left: px(16.0),
                right: px(16.0),
                top: px(8.0),
                bottom: px(8.0),
            },
            border: UiRect {
                left: px(1.0),
                right: px(1.0),
                top: px(1.0),
                bottom: px(1.0),
            },
        }
        BackgroundColor(Color::BLACK)
        Hovered::default()
        SmoothButtonAnimation::animate_down()
        UiTransform::default()
        BorderColor::all(Color::srgb(0.200, 0.255, 0.333))
        TextColor(Color::srgb(0.886, 0.910, 0.941))
        TextFont { font_size: FontSize::Px(12.0)}
        on(|event: On<Activate>, state: Res<State<UiState>>, mut next_state: ResMut<NextState<UiState>>|{
            match **state {
                UiState::GameView => next_state.set(UiState::MenuView),
                UiState::MenuView => next_state.set(UiState::GameView),
            }
        })
        Children [
            Text::new(name)
        ]
    }
}

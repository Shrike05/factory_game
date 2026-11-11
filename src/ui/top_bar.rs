use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;
use bevy::ui_widgets::{Activate, Button};

use crate::ui::types::*;
use crate::ui::ui_state::{MenuState, UiState};

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
                left: px(1.0),
                right: px(1.0),
                top: px(0.0),
                bottom: px(1.0),
            },
            border_radius: BorderRadius::bottom(CornerRadius::all(px(10.)))
        }
        BackgroundColor(Color::BLACK)
        BorderColor::all(BORDER_COLOR)
        ZIndex(5)
        Children[
            @top_button("Inventory".into(), MenuState::Inventory)
            --
            @top_button("Skill Tree".into(), MenuState::SkillTree)
            --
            @top_button("Settings".into(), MenuState::Settings)
        ]
    }
}

fn top_button(name: String, menu_state: MenuState) -> impl Scene {
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
        BorderColor::all(SECONDARY_BORDER_COLOR)
                on(move |event: On<Activate>, state: Res<State<UiState>>, mut next_state: ResMut<NextState<UiState>>|{
            match **state {
                UiState::GameView => next_state.set(UiState::MenuView(menu_state)),
                UiState::MenuView(new_state) => {
                    if new_state == menu_state{
                        next_state.set(UiState::GameView)
                    }else{
                        next_state.set(UiState::MenuView(menu_state))
                    }
                }
            }
        })
        Children [
            TextColor(FONT_COLOR)
            TextFont {
                font: FontSourceTemplate::Handle(FONT_PATH),
                font_size: FontSize::Px(HEADING3_FONT_SIZE)
            }
            Text::new(name)
        ]
    }
}

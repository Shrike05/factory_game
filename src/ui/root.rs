use bevy::prelude::*;

use crate::ui::hotbar::tool_bar;

pub fn root_ui() -> impl Scene {
    bsn! {
        // Unrepresentable CSS: 'user-select:' (User text selection cannot be configured via Bevy Node)
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            padding: UiRect {
                left: px(0.0),
                right: px(0.0),
                top: px(0.0),
                bottom: px(0.0),
            },
            margin: UiRect {
                left: px(0.0),
                right: px(0.0),
                top: px(0.0),
                bottom: px(0.0),
            },
            overflow: Overflow::clip(),
        }
        TextColor(Color::srgb(0.902, 0.929, 0.953))
        Children[
            Node{
                width: percent(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
            }
            Children[@tool_bar()]

        ]
    }
}

use bevy::prelude::*;

use crate::ui::game::*;
use crate::ui::{menu::detail_root_ui, top_bar::top_bar};

pub fn root_ui() -> impl Scene {
    bsn! {
        Pickable::IGNORE
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
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
            }
            Pickable::IGNORE
            Children[@top_bar()]
            --
            @info_panel()
            --
            @grid()
            --
            Node{
                width: percent(100.),
                height: percent(100.)
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
            }
            Pickable::IGNORE
            Children[@tool_bar()]
            --
            @detail_root_ui()
        ]
    }
}

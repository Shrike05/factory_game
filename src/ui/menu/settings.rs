use crate::ui::menu::types::*;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

/// Single key-value row for an individual setting.
pub fn setting_row(entry: SettingEntry) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect {
                left: px(0.0),
                right: px(0.0),
                top: px(4.0),
                bottom: px(4.0),
            },
            border: UiRect {
                left: px(0.0),
                right: px(0.0),
                top: px(0.0),
                bottom: px(1.0),
            },
        }
        BorderColor::all(Color::srgb(0.067, 0.067, 0.067))
        Children [
            Node::default()
            TextColor(Color::srgb(0.533, 0.533, 0.533))
            TextFont { font_size: FontSize::Px(11.0) }
            Children [
                Text::new(entry.label.clone())
            ]
            --
            Node::default()
            TextColor(Color::WHITE)
            TextFont { font_size: FontSize::Px(11.0) }
            Children [
                Text::new(entry.value.clone())
            ]
        ]
    }
}

/// Reusable grouped panel for a settings category (e.g. Video, Audio).
pub fn settings_group(group: SettingsGroupData) -> impl Scene {
    bsn! {
        Node {
            width: percent(48.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10.0),
            padding: UiRect::all(px(16.0)),
            border: UiRect::all(px(1.0)),
        }
        BackgroundColor(Color::srgb(0.02, 0.02, 0.02))
        BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
        Children [
            Node {
                border: UiRect {
                    left: px(0.0),
                    right: px(0.0),
                    top: px(0.0),
                    bottom: px(1.0),
                },
                padding: UiRect {
                    left: px(0.0),
                    right: px(0.0),
                    top: px(0.0),
                    bottom: px(6.0),
                },
            }
            BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
            TextColor(Color::WHITE)
            TextFont { font_size: FontSize::Px(12.0) }
            Children [
                Text::new(group.title.clone())
            ]
            --
            {
                group.entries.into_iter().map(|entry|{
                    bsn!{
                        @setting_row(entry)
                    }
                }).collect::<Vec<_>>()
            }
        ]
    }
}

/// Main Settings Tab UI iterating through settings_group.
pub fn settings_ui(data: SettingsUiData) -> impl Scene {
    bsn! {
        Node {
            width: percent(100.0),
            height: percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
        }
        Children [
            // Banner
            Node {
                width: percent(100.0),
                height: px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: px(12.0),
                    right: px(12.0),
                    top: px(0.0),
                    bottom: px(0.0),
                },
            }
            BackgroundColor(Color::srgb(0.039, 0.039, 0.039))
            Children [
                Node::default()
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(12.0) }
                Children [
                    Text::new("SYSTEM CONFIGURATION")
                ]
            ]
            --
            // Settings Cards Grid
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                flex_grow: 1.0,
                row_gap: px(16.0),
                column_gap: px(16.0),
                padding: UiRect::all(px(20.0)),
            }
            Children [
                {
                    data.groups.into_iter().map(|group|{
                        bsn!{
                            @settings_group(group)
                        }
                    }).collect::<Vec<_>>()
                }
            ]
            --
            // Apply Button Footer
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                row_gap: px(12.0),
                column_gap: px(12.0),
                padding: UiRect {
                    left: px(20.0),
                    right: px(20.0),
                    top: px(16.0),
                    bottom: px(16.0),
                },
            }
            BackgroundColor(Color::srgb(0.031, 0.031, 0.031))
            Children [
                Button
                Node {
                    padding: UiRect {
                        left: px(20.0),
                        right: px(20.0),
                        top: px(10.0),
                        bottom: px(10.0),
                    },
                    border: UiRect::all(px(1.0)),
                }
                BackgroundColor(Color::srgb(0.067, 0.067, 0.067))
                BorderColor::all(Color::srgb(0.267, 0.267, 0.267))
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(11.0) }
                Children [
                    Text::new("APPLY CONFIGURATION")
                ]
            ]
        ]
    }
}

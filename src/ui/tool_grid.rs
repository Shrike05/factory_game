use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::ui::types::SmoothButtonAnimation;

pub fn grid() -> impl Scene {
    bsn! {
        // Unrepresentable CSS: 'transform:' (2D matrix transforms require a separate Transform component)
        // Unrepresentable CSS: 'transition:' (CSS transitions require Bevy tweening systems)
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            right: px(0.0),
            top: percent(50.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
        }
        ZIndex(10)
        Children [
            Button
            Node {
                width: px(24.0),
                height: px(50.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect {
                    left: px(2.0),
                    right: px(2.0),
                    top: px(2.0),
                    bottom: px(2.0),
                },
            }
            BorderColor::all(Color::srgb(0.165, 0.231, 0.361))
            TextColor(Color::srgb(0.220, 0.741, 0.973))
            TextFont { font_size: FontSize::Px(11.) }
            Children [
                Text::new(">")
            ]
            --
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: px(16.0),
                    right: px(16.0),
                    top: px(16.0),
                    bottom: px(16.0),
                },
                border: UiRect {
                    left: px(2.0),
                    right: px(2.0),
                    top: px(2.0),
                    bottom: px(2.0),
                },
            }
            BorderColor::all(Color::srgb(0.165, 0.231, 0.361))
            BackgroundColor(Color::BLACK)
            Children [
                //Text Storage Chest
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect {
                        left: px(0.0),
                        right: px(0.0),
                        top: px(0.0),
                        bottom: px(12.0),
                    },
                }
                TextColor(Color::srgb(0.220, 0.741, 0.973))
                TextFont { font_size: FontSize::Px(14.) }
                Children [
                    Text::new("Factories")
                ]
                -- //Actual Grid
                Node {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::flex(4, 1.0)],
                    grid_template_rows: vec![RepeatedGridTrack::flex(4, 1.0)],
                    row_gap: px(8.0),
                    column_gap: px(8.0),
                }
                Children[
                    {(0..16).map(|x|{
                        bsn! {
                            @grid_item(x)
                        }
                    }).collect::<Vec<_>>()}
                ]
            ]
        ]
    }
}

fn grid_item(n: usize) -> impl Scene {
    bsn! {
        // Unrepresentable CSS: 'box-shadow:' (Bevy 2D UI lacks built-in drop shadows; requires custom mesh or shader)
        Node {
            width: px(100.0),
            height: px(100.0),
            display: Display::Flex,
            //position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect {
                left: px(1.0),
                right: px(1.0),
                top: px(1.0),
                bottom: px(1.0),
            },
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
                width: percent(100.0),
                height: percent(100.0),
                display: Display::Flex,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            Children [
                // Unrepresentable CSS: 'filter:' (CSS blur/filters are not supported on Bevy UI nodes)
                Node::default()
                TextFont { font_size: FontSize::Px(22.0)}
                Children [
                    Text::new(format!("F{n}"))
                ]
            ]
        ]
    }
}

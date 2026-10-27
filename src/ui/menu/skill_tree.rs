use crate::ui::menu::types::*;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

pub fn skill_detail_view(detail: SkillDetailData) -> impl Scene {
    let detail_color = if detail.is_max_level {
        Color::srgb(0.1, 0.1, 0.1)
    } else {
        Color::WHITE
    };
    let detail_text_color = if detail.is_max_level {
        Color::srgb(0.5, 0.5, 0.5)
    } else {
        Color::BLACK
    };
    bsn! {
        Node {
            width: percent(30.0),
            height: percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(16.0)),
            border: UiRect {
                left: px(0.0),
                right: px(2.0),
                top: px(0.0),
                bottom: px(0.0),
            },
        }
        BackgroundColor(Color::srgb(0.02, 0.02, 0.02))
        BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
        Children [
            // Header
            Node {
                width: percent(100.0),
                height: px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
            }
            BackgroundColor(Color::srgb(0.039, 0.039, 0.039))
            Children [
                Node::default()
                Children [
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(12.0) }
                    Text::new("SKILL SPECIFICATION")
                ]
                --
                Node::default()
                Children [
                TextColor(Color::srgb(0.4, 0.4, 0.4))
                TextFont { font_size: FontSize::Px(10.0) }
                    Text::new(detail.category_tier.clone())
                ]
            ]
            --
            // Thumbnail
            Node {
                width: percent(100.0),
                height: px(140.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: px(0.0),
                    right: px(0.0),
                    top: px(12.0),
                    bottom: px(16.0),
                },
                border: UiRect::all(px(1.0)),
            }
            BackgroundColor(Color::BLACK)
            BorderColor::all(Color::srgb(0.2, 0.2, 0.2))
            Children [
                Node::default()
                Children [
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(32.0) }
                    Text::new("[ + ]")
                ]
                --
                Node::default()
                Children [
                TextColor(Color::srgb(0.4, 0.4, 0.4))
                TextFont { font_size: FontSize::Px(10.0) }
                    Text::new(detail.figure_caption.clone())
                ]
            ]
            --
            // Descriptions & Tactical Modifiers
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
            }
            Children [
                Node::default()
                Children [
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(16.0) }
                    Text::new(detail.title.clone())
                ]
                --
                Node::default()
                Children [
                TextColor(Color::srgb(0.533, 0.533, 0.533))
                TextFont { font_size: FontSize::Px(10.0) }
                    Text::new(detail.level_status.clone())
                ]
                --
                Node::default()
                Children [
                TextColor(Color::srgb(0.8, 0.8, 0.8))
                TextFont { font_size: FontSize::Px(11.0) }
                    Text::new(detail.description.clone())
                ]
                --
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(4.0),
                    padding: UiRect::all(px(10.0)),
                    margin: UiRect {
                        left: px(0.0),
                        right: px(0.0),
                        top: px(8.0),
                        bottom: px(8.0),
                    },
                    border: UiRect::all(px(1.0)),
                }
                BackgroundColor(Color::srgb(0.039, 0.039, 0.039))
                BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
                Children [
                    {detail.stats.into_iter().map(|stat|{
                            bsn!{
                                Node::default()
                                Children [
                                    TextColor(Color::WHITE)
                                    TextFont { font_size: FontSize::Px(11.0) }
                                    Text::new(format!("+ {}", stat))
                                ]
                            }
                        }).collect::<Vec<_>>()
                    }
                ]
            ]
            --
            // Action Button
            Button
            Node {
                width: percent(100.0),
                height: px(40.0),
                border: UiRect::all(px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center
            }
            BackgroundColor(detail_color)
            BorderColor::all(Color::WHITE)
            Children [
                Node::default()
                Children[
                    TextColor(detail_text_color)
                    TextFont { font_size: FontSize::Px(12.0) }
                    Text::new(if detail.is_max_level {
                        "MAX LEVEL REACHED".to_string()
                    } else {
                        format!("UPGRADE SKILL [COST: {} SP]", detail.cost_points)
                    })
                ]
            ]
        ]
    }
}

/// Single reusable skill node widget.
pub fn skill_node_view(node: SkillNodeData) -> impl Scene {
    let border_color = if node.is_selected {
        Color::WHITE
    } else if node.is_unlocked {
        Color::srgb(0.6, 0.6, 0.6)
    } else {
        Color::srgb(0.2, 0.2, 0.2)
    };

    let bg_color = if node.is_selected {
        Color::srgb(0.133, 0.133, 0.133)
    } else if node.is_unlocked {
        Color::srgb(0.05, 0.05, 0.05)
    } else {
        Color::srgb(0.015, 0.015, 0.015)
    };

    bsn! {
        Node {
            width: px(130.0),
            height: px(70.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(6.0)),
            border: UiRect::all(if node.is_selected { px(2.0) } else { px(1.0) }),
        }
        BackgroundColor(bg_color)
        BorderColor::all(border_color)
        Children [
            Node::default()
            Children [
            TextColor(Color::srgb(0.4, 0.4, 0.4))
            TextFont { font_size: FontSize::Px(9.0) }
                Text::new(node.code.clone())
            ]
            --
            Node::default()
            Children [

            TextColor(Color::WHITE)
            TextFont { font_size: FontSize::Px(11.0) }
                Text::new(node.name.clone())
            ]
            --
            Node::default()
            Children [
            TextColor(Color::srgb(0.667, 0.667, 0.667))
            TextFont { font_size: FontSize::Px(9.0) }
                Text::new(node.status_text.clone())
            ]
        ]
    }
}

/// The 60-70% Node Canvas section rendering all node items.
pub fn skill_tree_canvas(nodes: &[SkillNodeData]) -> impl Scene {
    bsn! {
        Node {
            width: percent(70.0),
            height: percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
        }
        BackgroundColor(Color::BLACK)
        Children [
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
                Children [
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(12.0) }
                    Text::new("NEURAL SPECIALIZATION MATRIX")
                ]
            ]
            --
            Node {
                height: percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceAround,
                row_gap: px(16.0),
                column_gap: px(16.0),
                padding: UiRect::all(px(24.0)),
                overflow: Overflow::clip(),
            }
            Children [
                {nodes.into_iter().map(|node| bsn!{@skill_node_view(node.clone())}).collect::<Vec<_>>()}
            ]
        ]
    }
}

pub fn skill_tree_ui(data: SkillTreeUiData) -> impl Scene {
    bsn! {
        Node {
            width: percent(100.0),
            height: percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
        }
        Children [
            // 30% Detail View
            @skill_detail_view(data.selected_detail)
            --
            // 60-70% Node Canvas
            @skill_tree_canvas(data.nodes.as_slice())
        ]
    }
}

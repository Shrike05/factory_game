use crate::ui::menu::types::*;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

pub fn inventory_ui(data: InventoryUiData) -> impl Scene {
    bsn! {
        Node {
            width: percent(100.0),
            height: percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
        }
        Children [
            // 80% Grid Section
            Node {
                width: percent(80.0),
                height: percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                border: UiRect {
                    left: px(0.0),
                    right: px(2.0),
                    top: px(0.0),
                    bottom: px(0.0),
                },
            }
            BackgroundColor(Color::srgb(0.008, 0.008, 0.008))
            BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
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
                        Text::new("Inventory")
                    ]
                ]
                --
                // Iterating through each graph card
                Node {
                    height: percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: px(12.0),
                    column_gap: px(12.0),
                    padding: UiRect::all(px(12.0)),
                    overflow: Overflow::clip(),
                }
                Children [
                    {
                        data.items_telemetry.into_iter().map(|item|{
                            bsn!{
                                @item_graph_card(item)
                            }
                        }).collect::<Vec<_>>()
                    }
                ]
            ]
            --
            // 20% Manifest Sidebar
            @items_list_ui(
                data.items_manifest
            )
        ]
    }
}

/// Single manifest row entry.
pub fn item_manifest_row(entry: ItemManifestEntry) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(10.0)),
            border: UiRect {
                left: px(0.0),
                right: px(0.0),
                top: px(0.0),
                bottom: px(1.0),
            },
        }
        BackgroundColor(Color::srgb(0.031, 0.031, 0.031))
        BorderColor::all(Color::srgb(0.102, 0.102, 0.102))
        Children [
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
            }
            Children [
                Node::default()
                Children [
                    TextColor(Color::WHITE)
                    TextFont { font_size: FontSize::Px(11.0) }
                    Text::new(entry.name.clone())
                ]
                --
                Node::default()
                Children [
                    TextColor(Color::srgb(0.333, 0.333, 0.333))
                    TextFont { font_size: FontSize::Px(9.0) }
                    Text::new(entry.category)
                ]
            ]
            --
            Node::default()
            Children [
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(11.0) }
                Text::new(entry.exact_qty.clone())
            ]
        ]
    }
}

/// The 20% Manifest sidebar, iterating through item_manifest_row.
pub fn items_list_ui(entries: Vec<ItemManifestEntry>) -> impl Scene {
    bsn! {
        Node {
            width: percent(20.0),
            height: percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
        }
        BackgroundColor(Color::srgb(0.02, 0.02, 0.02))
        Children [
            // Title Header
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
                    Text::new("ITEMS")
                ]
            ]
            --
            // Scrollable List
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                overflow: Overflow::clip(),
            }
            Children [
                {
                    entries.into_iter().map(|item|{
                        bsn!{
                            @item_manifest_row(item.clone())
                        }
                    }).collect::<Vec<_>>()
                }
            ]
        ]
    }
}

/// Reusable individual graph card for item growth telemetry.
pub fn item_graph_card(item: ItemTelemetry) -> impl Scene {
    bsn! {
        Node {
            width: percent(48.),
            height: percent(20.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(10.0)),
            margin: UiRect::all(px(0.0)),
            border: UiRect::all(px(1.0)),
        }
        BackgroundColor(Color::srgb(0.031, 0.031, 0.031))
        BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
        Children [
            // Card Header
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: px(0.0),
                    right: px(0.0),
                    top: px(0.0),
                    bottom: px(8.0),
                },
            }
            Children [
                Node::default()
                Children [
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(12.0) }
                    Text::new(item.title.clone())
                ]
                --
                Node {
                    padding: UiRect {
                        left: px(6.0),
                        right: px(6.0),
                        top: px(2.0),
                        bottom: px(2.0),
                    },
                    border: UiRect::all(px(1.0)),
                }
                BackgroundColor(Color::srgb(0.102, 0.102, 0.102))
                BorderColor::all(Color::srgb(0.267, 0.267, 0.267))
                Children [
                TextColor(Color::WHITE)
                TextFont { font_size: FontSize::Px(10.0) }
                    Text::new(item.growth_rate.clone())
                ]
            ]
            --
            // Plot Canvas Area
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                padding: UiRect::all(px(6.0)),
                border: UiRect::all(px(1.0)),
            }
            BackgroundColor(Color::BLACK)
            BorderColor::all(Color::srgb(0.102, 0.102, 0.102))
            Children [
                // Y-Axis Ticks
                Node {
                    width: px(40.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                }
                Children [
                TextColor(Color::srgb(0.333, 0.333, 0.333))
                TextFont { font_size: FontSize::Px(9.0) }
                    Text::new(item.max_yield.clone())
                    --
                TextColor(Color::srgb(0.333, 0.333, 0.333))
                TextFont { font_size: FontSize::Px(9.0) }
                    Text::new(item.min_yield.clone())
                ]
                --
                // Graph Line
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_grow: 1.0,
                    padding: UiRect::all(px(4.0)),
                    border: UiRect::all(px(1.0)),
                }
                BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
                Children [
                    Node {
                        width: percent(100.0),
                        height: px(1.0),
                    }
                    BackgroundColor(Color::srgb(0.067, 0.067, 0.067))
                    --
                    Node {
                        width: percent(100.0),
                        height: px(2.0),
                    }
                    BackgroundColor(Color::WHITE)
                ]
            ]
            --
            // Card Footer
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: px(0.0),
                    right: px(0.0),
                    top: px(6.0),
                    bottom: px(0.0),
                },
            }
            Children [
                Node::default()
                Children [
                    TextColor(Color::srgb(0.4, 0.4, 0.4))
                    TextFont { font_size: FontSize::Px(10.0) }
                    Text::new("CURRENT:")
                ]
                --
                Node::default()
                Children [
                    TextColor(Color::WHITE)
                    TextFont { font_size: FontSize::Px(10.0) }
                    Text::new(item.current_yield.clone())
                ]
                --
                Node::default()
                Children [
                    TextColor(Color::srgb(0.533, 0.533, 0.533))
                    TextFont { font_size: FontSize::Px(10.0) }
                    Text::new(item.rate.clone())
                ]
            ]
        ]
    }
}

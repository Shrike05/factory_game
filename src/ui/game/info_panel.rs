use bevy::text::FontSourceTemplate;
use bevy::ui_widgets::Button;
use bevy::{prelude::*, ui_widgets::Activate};
use strum::IntoEnumIterator;

use crate::factory::{Item, Production};
use crate::ui::types::*;

pub fn info_panel() -> impl Scene {
    bsn! {
        // Unrepresentable CSS: 'transition:' (CSS transitions require Bevy tweening systems)
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            left: px(0.0),
            top: px(20.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::FlexStart,
        }
        UiTransform::default()
        SlidePanel::new("info".into(), 90., false)
        ZIndex(10)
        Children [
            // Unrepresentable CSS: 'box-shadow:' (Bevy 2D UI lacks built-in drop shadows; requires custom mesh or shader)
            // Unrepresentable CSS: 'backdrop-filter:' (Backdrop filters are not supported in Bevy UI)
            Node {
                min_width: px(200.0),
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
            BorderColor::all(BORDER_COLOR)
            BackgroundColor(Color::BLACK)
            Children [
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect {
                        left: px(0.0),
                        right: px(0.0),
                        top: px(0.0),
                        bottom: px(12.0),
                    },
                    border: UiRect::bottom(px(1.))
                }
                BorderColor::all(SECONDARY_BORDER_COLOR)
                Children [
                    TextColor(FONT_COLOR)
                    TextFont {
                        font: FontSourceTemplate::Handle(FONT_PATH),
                        font_size: FontSize::Px(HEADING2_FONT_SIZE)
                    }
                    Text::new("Inventory Status")
                ]
                --
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
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
                }
                Children[
                {
                    Item::iter().map(|item|{
                        info_item(item, 0)
                    }).collect::<Vec<_>>()
                }
                ]
            ]
            --
            Button
            // Unrepresentable CSS: 'cursor:' (Cursor styling is managed at the Window level or via custom interaction plugins)
            // Unrepresentable CSS: 'box-shadow:' (Bevy 2D UI lacks built-in drop shadows; requires custom mesh or shader)
            Node {
                width: px(24.0),
                height: px(50.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::right(CornerRadius::circular(Val::Px(19.))),
                border: UiRect{
                    top: px(1.),
                    left: px(0.),
                    right: px(1.),
                    bottom: px(1.),
                }
            }
            BorderColor::all(BORDER_COLOR)
            BackgroundColor(PRIMARY_BASE)
            PopoutButton("info")
            on(|event: On<Activate>, mut slide_panels: Query<&mut SlidePanel>, popouts: Query<&PopoutButton>|{
                if let Ok(popout) = popouts.get(event.entity) {
                    for mut slide_panel in slide_panels.iter_mut(){
                        if slide_panel.name == popout.0{
                            slide_panel.state = match slide_panel.state{
                                PanelState::Open => PanelState::Closed,
                                PanelState::Closed => PanelState::Open
                            }
                        }
                    }
                }
            })
            Children [
                Text::new("<")
            ]
        ]
    }
}

fn info_item(item: Item, amount: u64) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
        }
        Children [
            Node::default()
            TextColor(FONT_COLOR)
            Children [
                TextColor(FONT_COLOR)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(HEADING3_FONT_SIZE)
                }
                Text::new(format!("{item:?}:"))
            ]
            --
            Node::default()
            TextColor(FONT_COLOR)
            Children [
                InfoItem(item)
                TextColor(FONT_COLOR)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(HEADING3_FONT_SIZE)
                }
                Text::new(format!("{amount}"))
            ]
        ]
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct InfoItem(Item);

pub fn update_info_items(mut info_items: Query<(&mut Text, &InfoItem)>, prod: Res<Production>) {
    for (mut text, item) in info_items.iter_mut() {
        if let Some(amount) = prod.inventory.get(&item.0) {
            text.0 = amount.to_string();
        }
    }
}

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};

use crate::factory::FactoryType;
use crate::ui::types::{PaletteItem, PanelState, PopoutButton, SlidePanel, SmoothButtonAnimation};

pub fn grid() -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            right: px(0.0),
            top: percent(50.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
        }
        UiTransform::default()
        SlidePanel::new("grid".into(), 95., true)
        ZIndex(10)
        Children [
            Button
            Node {
                width: px(24.0),
                height: px(80.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::left(CornerRadius::circular(Val::Px(19.)))
            }
            BackgroundColor(Color::BLACK)
            PopoutButton("grid")
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
                // Text Storage Chest
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
                -- // Actual Grid (4x4 static palette)
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

/// Map grid index to a FactoryType variant
fn factory_for_index(n: usize) -> FactoryType {
    match n {
        0 => FactoryType::Empty,
        1 => FactoryType::Sink,
        _ => FactoryType::Source, // Adjust to match your FactoryType enum variants
    }
}

fn grid_item(n: usize) -> impl Scene {
    let factory = factory_for_index(n);

    bsn! {
        Button
        Node {
            width: px(100.0),
            height: px(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
        }
        // PaletteItem marks this item as a static source for drag-and-drop
        PaletteItem(factory)
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
            Pickable::IGNORE
            Children [
                Node::default()
                Pickable::IGNORE
                TextFont { font_size: FontSize::Px(22.0) }
                Children [
                    Pickable::IGNORE
                    Text::new(format!("{factory:?}"))
                ]
            ]
        ]
    }
}

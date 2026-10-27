use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::factory::FactoryType;
use crate::states::BuildSelection;
use crate::ui::types::{PaletteItem, SmoothButtonAnimation};

pub fn tool_bar() -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            row_gap: px(8.0),
            column_gap: px(8.0),
            padding: UiRect {
                left: px(10.0),
                right: px(10.0),
                top: px(10.0),
                bottom: px(10.0),
            },
            border: UiRect {
                left: px(2.0),
                right: px(2.0),
                top: px(2.0),
                bottom: px(2.0),
            },
        }
        BorderColor::all(Color::BLACK)
        BackgroundColor(Color::BLACK)
        Children[
            {(0..10).map(|x|{
                bsn! {
                    @toolbar_item(x)
                }
            }).collect::<Vec<_>>()}
            --
            Node{
                flex_direction: FlexDirection::Column,
                height: percent(100.)
            }
            Children[
                Button
                SmoothButtonAnimation::default()
                Hovered::default()
                UiTransform::default()
                Node{
                    width: px(30.0),
                    height: px(30.0),
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                }
                BackgroundColor(Color::srgb(1., 0., 0.))
                TextFont { font_size: FontSize::Px(15.) }
                on(|event: On<PointerClick>, mut next_state: ResMut<NextState<BuildSelection>>|{
                    next_state.set(BuildSelection::Removing);
                })
                Children[Text::new("T")]
            ]
        ]
    }
}

pub fn toolbar_item(n: usize) -> impl Scene {
    bsn! {
        Node {
            width: px(80.0),
            height: px(80.0),
            display: Display::Flex,
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.)),
        }
        SmoothButtonAnimation::default()
        Hovered::default()
        UiTransform{
            scale: Vec2::ONE,
            translation: Val2::px(0., 0.)
        }
        BorderColor::all(Color::srgb(0.239, 0.322, 0.471))
        // Accept drops from palette items:
        on(|event: On<PointerDragDrop>,
            palette_query: Query<&PaletteItem>,
            mut toolbar_query: Query<&mut ToolBarItem>,
            children_query: Query<&Children>| {
            // Check if the dragged entity has a PaletteItem
            if let Ok(palette) = palette_query.get(event.dropped) && let Ok(children) = children_query.get(event.entity){
                for child in children.iter() {
                    if let Ok(mut item) = toolbar_query.get_mut(child) {
                        item.0 = Some(palette.0);
                    }
                }
            }
        })
        Children [
            Node {
                position_type: PositionType::Absolute,
                left: px(6.0),
                top: px(4.0),
            }
            TextColor(Color::srgb(0.392, 0.455, 0.545))
            TextFont { font_size: FontSize::Px(10.) }
            Text::new(format!("{n}"))
            --
            Button
            Node {
                width: percent(100.0),
                height: percent(100.0),
                display: Display::Flex,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            // Read active ToolBarItem dynamically on click
            on(|event: On<Activate>,
                items: Query<&ToolBarItem>,
                mut next_state: ResMut<NextState<BuildSelection>>| {
                if let Ok(item) = items.get(event.entity) && let Some(factory) = item.0 {
                    next_state.set(BuildSelection::Factory(factory));
                }
            })
            ToolBarItem(None)
            Visibility::Hidden
            Children [
                TextFont { font_size: FontSize::Px(22.) }
                Text::new("")
            ]
        ]
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct ToolBarItem(pub Option<FactoryType>);

/// Updates visibility and text when ToolBarItem changes
pub fn tool_bar_item_update(
    mut tool_bar_item_query: Query<
        (&mut Visibility, &ToolBarItem, &Children),
        Changed<ToolBarItem>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (mut visibility, item, children) in tool_bar_item_query.iter_mut() {
        match item.0 {
            Some(factory) => {
                *visibility = Visibility::Visible;
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        *text = Text::new(format!("{factory:?}"));
                    }
                }
            }
            None => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

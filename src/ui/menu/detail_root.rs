use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::ui::menu::inventory_detail::inventory_ui;
use crate::ui::menu::settings::settings_ui;
use crate::ui::menu::skill_tree::{skill_detail_view, skill_tree_ui};
use crate::ui::menu::types::*;
use crate::ui::ui_state::{MenuState, UiState};

pub fn detail_root_ui() -> impl Scene {
    let skill_tree_data = SkillTreeUiData {
        selected_detail: SkillDetailData {
            category_tier: "0".into(),
            figure_caption: "Smelter Upgrade".into(),
            title: "Smelter".into(),
            level_status: "1".into(),
            description: "Smelt ore into ingots".into(),
            stats: vec!["0".to_string()],
            prerequisite: "".into(),
            cost_points: 100,
            is_max_level: false,
        },
        nodes: vec![
            SkillNodeData {
                code: "Sm".into(),
                name: "Smelter".into(),
                tier: 0,
                status_text: "Locked".into(),
                is_unlocked: false,
                is_selected: true,
            },
            SkillNodeData {
                code: "Cr".into(),
                name: "Crusher".into(),
                tier: 0,
                status_text: "Locked".into(),
                is_unlocked: false,
                is_selected: true,
            },
            SkillNodeData {
                code: "Fr".into(),
                name: "Former".into(),
                tier: 1,
                status_text: "Locked".into(),
                is_unlocked: false,
                is_selected: true,
            },
        ],
    };
    let inventory_data = InventoryUiData {
        items_telemetry: vec![
            ItemTelemetry {
                title: "Rock".into(),
                growth_rate: "+10".into(),
                current_yield: "0".into(),
                rate: "0".into(),
                min_yield: "0".into(),
                max_yield: "10".into(),
                plot_values: (0..50).into_iter().map(|x| (x * x) as f32).collect(),
            },
            ItemTelemetry {
                title: "Iron".into(),
                growth_rate: "+10".into(),
                current_yield: "0".into(),
                rate: "0".into(),
                min_yield: "0".into(),
                max_yield: "10".into(),
                plot_values: (1..50).into_iter().map(|x| (x as f32).ln()).collect(),
            },
        ],
        items_manifest: vec![ItemManifestEntry {
            name: "Rock".into(),
            category: "Mineral".into(),
            exact_qty: "10".into(),
        }],
    };

    let settings_data = SettingsUiData {
        groups: vec![SettingsGroupData {
            title: "Sound Settings".into(),
            entries: vec![
                SettingEntry {
                    label: "Master".into(),
                    value: "70".into(),
                },
                SettingEntry {
                    label: "BGM".into(),
                    value: "70".into(),
                },
                SettingEntry {
                    label: "SE".into(),
                    value: "70".into(),
                },
            ],
        }],
    };
    bsn! {
        Node{
            position_type: PositionType::Absolute,
            width: percent(100.),
            height: percent(100.),
        }
        BackgroundColor(Color::Srgba(Srgba { red: 0., green: 0., blue: 0., alpha: 0.6 }))
        Pickable::IGNORE
        Visibility::Hidden
        DetailRoot
        Children[
            Node{
                position_type: PositionType::Absolute,
                width: percent(96.),
                height: percent(93.),
                left: percent(2.),
                top: px(60.),
            }
            Pickable::IGNORE
            BackgroundColor(Color::BLACK)
            Children[
                @inventory_ui(inventory_data)
                InventoryTab
                --
                @skill_tree_ui(skill_tree_data)
                SkillTreeTab
                --
                @settings_ui(settings_data)
                SettingsTab
            ]
        ]
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DetailRoot;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct InventoryTab;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SkillTreeTab;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SettingsTab;

pub fn update_menu_visibility(
    state: Res<State<UiState>>,
    mut menus: ParamSet<(
        Query<&mut Node, With<InventoryTab>>,
        Query<&mut Node, With<SkillTreeTab>>,
        Query<&mut Node, With<SettingsTab>>,
    )>,
) {
    if !state.is_changed() {
        return;
    }

    let (inv_display, skill_display, settings_display) = match state.get() {
        UiState::MenuView(MenuState::Inventory) => (Display::Flex, Display::None, Display::None),
        UiState::MenuView(MenuState::SkillTree) => (Display::None, Display::Flex, Display::None),
        UiState::MenuView(MenuState::Settings) => (Display::None, Display::None, Display::Flex),
        _ => (Display::None, Display::None, Display::None),
    };

    for mut node in &mut menus.p0() {
        node.display = inv_display;
    }
    for mut node in &mut menus.p1() {
        node.display = skill_display;
    }
    for mut node in &mut menus.p2() {
        node.display = settings_display;
    }
}

use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::ui::menu::inventory_detail::inventory_ui;
use crate::ui::menu::skill_tree::{skill_detail_view, skill_tree_ui};
use crate::ui::menu::types::*;

pub fn detail_root_ui() -> impl Scene {
    bsn! {
            Node{
                position_type: PositionType::Absolute,
                width: percent(96.),
                height: percent(96.),
                left: percent(2.),
                top: percent(3.),
            }
            BackgroundColor(Color::BLACK)
            Children[
    //        @inventory_ui(InventoryUiData{
    //            items_telemetry: vec![
    //                ItemTelemetry{
    //                    title: "Rock".into(),
    //                    growth_rate: "+10".into(),
    //                    current_yield: "0".into(),
    //                    rate: "0".into(),
    //                    min_yield: "0".into(),
    //                    max_yield: "10".into(),
    //                    plot_values: (0..50).into_iter().map(|x| (x*x) as f32).collect()
    //                },
    //                ItemTelemetry{
    //                    title: "Iron".into(),
    //                    growth_rate: "+10".into(),
    //                    current_yield: "0".into(),
    //                    rate: "0".into(),
    //                    min_yield: "0".into(),
    //                    max_yield: "10".into(),
    //                    plot_values: (1..50).into_iter().map(|x| (x as f32).ln()).collect()
    //                }
    //            ],
    //            items_manifest: vec![
    //                ItemManifestEntry{
    //                    name: "Rock".into(),
    //                    category: "Mineral".into(),
    //                    exact_qty: "10".into(),
    //                }
    //            ]
    //        })
                @skill_tree_ui(SkillTreeUiData { selected_detail: SkillDetailData {
                    category_tier: "0".into(),
                    figure_caption: "Smelter Upgrade".into(),
                    title: "Smelter".into(),
                    level_status: "1".into(),
                    description: "Smelt ore into ingots".into(),
                    stats: vec!["0".to_string()],
                    prerequisite: "".into(),
                    cost_points: 100,
                    is_max_level: false
                }, nodes: vec![SkillNodeData{
                    code: "Sm".into(),
                    name: "Smelter".into(),
                    tier: 0,
                    status_text: "Locked".into(),
                    is_unlocked: false,
                    is_selected: true,
                }]
                })
            ]
        }
}

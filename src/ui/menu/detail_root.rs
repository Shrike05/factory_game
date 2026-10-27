use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::ui::menu::inventory_detail::inventory_ui;
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
        @inventory_ui(InventoryUiData{
            items_telemetry: vec![
                ItemTelemetry{
                    title: "Rock".into(),
                    growth_rate: "+10".into(),
                    current_yield: "0".into(),
                    rate: "0".into(),
                    min_yield: "0".into(),
                    max_yield: "10".into()
                },
                ItemTelemetry{
                    title: "Iron".into(),
                    growth_rate: "+10".into(),
                    current_yield: "0".into(),
                    rate: "0".into(),
                    min_yield: "0".into(),
                    max_yield: "10".into()
                }
            ],
            items_manifest: vec![
                ItemManifestEntry{
                    name: "Rock".into(),
                    category: "Mineral".into(),
                    exact_qty: "10".into(),
                }
            ]
        })
        ]
    }
}

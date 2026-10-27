use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct ItemTelemetry {
    pub title: String,
    pub growth_rate: String,
    pub current_yield: String,
    pub rate: String,
    pub min_yield: String,
    pub max_yield: String,
}

#[derive(Clone, Debug)]
pub struct ItemManifestEntry {
    pub name: String,
    pub category: String,
    pub exact_qty: String,
}

#[derive(Clone, Debug)]
pub struct InventoryUiData {
    pub items_telemetry: Vec<ItemTelemetry>,
    pub items_manifest: Vec<ItemManifestEntry>,
}

#[derive(Clone, Debug)]
pub struct SkillNodeData {
    pub code: String,
    pub name: String,
    pub tier: u8,
    pub status_text: String,
    pub is_unlocked: bool,
    pub is_selected: bool,
}

#[derive(Clone, Debug)]
pub struct SkillDetailData {
    pub category_tier: String,
    pub figure_caption: String,
    pub title: String,
    pub level_status: String,
    pub description: String,
    pub stats: Vec<String>,
    pub prerequisite: String,
    pub cost_points: u32,
    pub is_max_level: bool,
}

#[derive(Clone, Debug)]
pub struct SkillTreeUiData {
    pub selected_detail: SkillDetailData,
    pub nodes: Vec<SkillNodeData>,
}

#[derive(Clone, Debug)]
pub struct SettingEntry {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct SettingsGroupData {
    pub title: String,
    pub entries: Vec<SettingEntry>,
}

#[derive(Clone, Debug)]
pub struct SettingsUiData {
    pub groups: Vec<SettingsGroupData>,
}

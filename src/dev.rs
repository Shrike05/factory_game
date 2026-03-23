use crate::{factory::FactoryName, states::BuildSelection};
use bevy::prelude::*;
use bevy_devtools::TUICommand;
use clap::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_selection_command);
    }
}

#[derive(Parser, Debug)]
#[command(name = "game")]
struct BuildSelectionParser {
    #[command(subcommand)]
    command: BuildSelectionVariants,
}

#[derive(Subcommand, Debug)]
enum BuildSelectionVariants {
    /// Build a road (no extra arguments)
    Road,
    /// Build a factory (requires a type)
    Factory {
        /// The type of factory to build
        factory_type: String,
    },
    None,
}

pub fn build_selection_command(
    mut command_reader: MessageReader<TUICommand>,
    mut build_selection_state: ResMut<NextState<BuildSelection>>,
) {
    for command in command_reader.read() {
        if let Ok(build_selection_parser) = command.parse_clap::<BuildSelectionParser>() {
            let selection = match build_selection_parser.command {
                BuildSelectionVariants::Road => BuildSelection::Road,
                BuildSelectionVariants::Factory { factory_type } => {
                    BuildSelection::Factory(FactoryName::from_string(factory_type))
                }
                BuildSelectionVariants::None => BuildSelection::None,
            };

            build_selection_state.set(selection);
        }
    }
}

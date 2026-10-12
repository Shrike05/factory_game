use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tool_bar.spawn());
    }
}

fn tool_bar() -> impl Scene {
    bsn! {
        Node {
            width: percent(100.),
            height: Val::Px(100.),
            justify_content: JustifyContent::Center,
            justify_self: JustifySelf::End,
            align_self: AlignSelf::End,
        }
        BackgroundColor(Color::Srgba(Srgba::rgb(0., 0., 0.)))

        Children [
            Node {
                width: Val::Px(70.),
                height: Val::Px(120.),
            }
            BackgroundColor(Color::Srgba(Srgba::rgb(0., 0.4, 0.)))
        ]
    }
}

use crate::tui::types::*;
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use bevy_ratatui::event::KeyMessage;

pub fn draw_system(tui_input: Res<TUIInput>, mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = ratatui::text::Text::raw(format!("> {}", tui_input.get()));
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}

pub fn input_system(
    mut tui_input: ResMut<TUIInput>,
    mut messages: MessageReader<KeyMessage>,
    mut tui_command_writer: MessageWriter<TUICommand>,
) {
    for message in messages.read() {
        if let crossterm::event::KeyCode::Enter = message.code {
            tui_command_writer.write(TUICommand::new(
                tui_input
                    .get()
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect(),
            ));

            tui_input.pop_command();
            continue;
        }
        if let Some(m) = message.0.code.as_char() {
            tui_input.add(m);
        }
    }
}

use std::ops::Add;

use crate::tui::types::*;
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use bevy_ratatui::event::KeyMessage;
use crossterm::event::KeyEventKind;

pub fn draw_system(tui_input: Res<TUIInput>, mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let history = tui_input
            .get_history()
            .iter()
            .fold("".to_string(), |acc, x| acc + "\n" + x);
        let text = history + &format!("\n> {}", tui_input.get());
        let final_text = ratatui::text::Text::raw(text);
        frame.render_widget(final_text, frame.area());
    })?;

    Ok(())
}

pub fn input_system(
    mut tui_input: ResMut<TUIInput>,
    mut messages: MessageReader<KeyMessage>,
    mut tui_command_writer: MessageWriter<TUICommand>,
) {
    for message in messages.read() {
        if message.kind != KeyEventKind::Press {
            continue;
        }

        if !tui_input.get().is_empty()
            && let crossterm::event::KeyCode::Enter = message.code
        {
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

        if let crossterm::event::KeyCode::Backspace = message.code {
            tui_input.remove_char();
        }

        if let Some(m) = message.0.code.as_char() {
            tui_input.add(m);
        }
    }
}

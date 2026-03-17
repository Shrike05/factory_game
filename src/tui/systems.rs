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

pub fn input_system(mut tui_input: ResMut<TUIInput>, mut messages: MessageReader<KeyMessage>) {
    for message in messages.read() {
        if let Some(m) = message.0.code.as_char() {
            tui_input.add(m);
        }
    }
}

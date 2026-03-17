use crate::tui::types::*;
use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use bevy_ratatui::event::KeyMessage;
use crossterm::event::KeyEventKind;
use log::info;
use ratatui::layout::*;
use ratatui::widgets::*;

pub fn draw_system(tui_input: Res<TUIInput>, mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(frame.area());

        let log_widget = tui_logger::TuiLoggerWidget::default()
            .block(Block::default().borders(Borders::ALL))
            .output_file(false)
            .output_target(false)
            .output_line(false)
            .output_timestamp(None)
            .output_level(None);

        let history = tui_input
            .get_history()
            .iter()
            .fold("".to_string(), |acc, x| acc + x + "\n");

        let text = history + &format!("> {}", tui_input.get());
        let final_text =
            Paragraph::new(text).block(Block::default().title("Shell").borders(Borders::ALL));

        frame.render_widget(final_text, chunks[0]);
        frame.render_widget(log_widget, chunks[1]);
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
            tui_command_writer.write(TUICommand::new(tui_input.get()));

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

pub fn clear_command(
    mut command_reader: MessageReader<TUICommand>,
    mut tui_input: ResMut<TUIInput>,
) {
    for command in command_reader.read() {
        if command.parse::<ClearCommand>().is_ok() {
            tui_input.reset();
        }
    }
}

pub fn print_entities_command(
    mut command_reader: MessageReader<TUICommand>,
    entities: Query<Entity>,
) {
    for command in command_reader.read() {
        if command.parse::<PrintEntities>().is_err() {
            continue;
        }

        for entity in entities {
            info!("{:?}", entity);
        }
    }
}

mod input;
mod model;
mod msg;
mod tui;
mod update;
mod view;
mod words;

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use input::handle_input;
use model::{AppState, Model};
use msg::{AppMsg, Message};
use ratatui::{prelude::*, widgets::Paragraph};
use update::update;
use view::view;

use crate::words::get_words;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();
    let mut current_msg = Some(Message::AppMessage(AppMsg::InitMsg));

    while model.app_state != AppState::Quiting {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
        // Handle events and map to a Message
        current_msg = handle_input(&mut model)?;

        let mut log = OpenOptions::new()
            .write(true)
            .create(true)
            .open("log/log.txt")?;
        log.write(format!("{}\n{}", model.current_words, model.current_typed).as_bytes())?;
    }

    tui::restore_terminal()?;
    Ok(())
}

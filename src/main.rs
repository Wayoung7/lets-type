mod input;
mod model;
mod msg;
mod tui;
mod update;
mod view;

use std::{error::Error, fs::OpenOptions, io::Write, time::SystemTime};

use input::handle_input;
use model::{AppState, Model};
use msg::{AppMsg, Message};
use update::update;
use view::view;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();
    let mut current_msg = Some(Message::AppMessage(AppMsg::InitMsg));
    let now = SystemTime::now();

    while model.app_state != AppState::Quiting {
        model.time_elapsed = now.elapsed().unwrap();
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
        // Handle events and map to a Message
        current_msg = handle_input(&mut model)?;
    }

    tui::restore_terminal()?;
    Ok(())
}

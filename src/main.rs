mod input;
mod model;
mod msg;
mod tui;
mod update;
mod view;

use std::{
    error::Error,
    time::{Duration, SystemTime},
};

use input::handle_input;
use model::{AppState, Model};
use msg::{AppMsg, Message};
use update::update;
use view::view;

fn main() -> Result<(), Box<dyn Error>> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = Model::default();
    let mut current_msgs = vec![Message::AppMessage(AppMsg::InitMsg)];
    let mut loop_timer = SystemTime::now();

    while model.app_state != AppState::Quiting {
        if model.app_state.is_typing()
            && SystemTime::now()
                .duration_since(model.timer)
                .unwrap_or(Duration::ZERO)
                > Duration::from_secs(1)
        {
            current_msgs.push(Message::WaitMsg);
        }

        // Stop timer while not typing
        if model.app_state.is_typing() {
            model.time_elapsed += loop_timer.elapsed().unwrap_or(Duration::ZERO);
        }
        loop_timer = SystemTime::now();
        terminal.draw(|f| view(&mut model, f))?;

        // Process updates as long as they return a non-None message
        while !current_msgs.is_empty() {
            if let Some(current_msg) = current_msgs.pop() {
                let mut new_msgs = update(&mut model, current_msg);
                current_msgs.append(&mut new_msgs);
            }
        }
        // Handle events and map to a Message
        current_msgs = handle_input(&mut model)?;
    }

    tui::restore_terminal()?;
    Ok(())
}

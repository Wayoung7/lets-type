use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    model::{AppState, Model},
    msg::{AppMsg, Message, TypingMsg},
};

pub fn handle_input(model: &mut Model) -> Result<Vec<Message>, Box<dyn Error>> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(model, key));
            }
        }
    }
    Ok(Vec::new())
}

fn handle_key(model: &mut Model, key: KeyEvent) -> Vec<Message> {
    match key.code {
        KeyCode::Esc => {
            if model.app_state == AppState::Info {
                vec![Message::AppMessage(AppMsg::RunMsg)]
            } else if model.app_state.is_running() {
                vec![Message::AppMessage(AppMsg::QuitMsg)]
            } else {
                Vec::new()
            }
        }
        KeyCode::Tab => {
            if model.app_state != AppState::Info && model.app_state != AppState::Loading {
                vec![Message::AppMessage(AppMsg::InfoMsg)]
            } else {
                vec![Message::AppMessage(AppMsg::RunMsg)]
            }
        }
        KeyCode::Backspace => {
            if model.app_state.is_running() && model.enable_backspace {
                vec![Message::TypingMessage(TypingMsg::BackSpaceMsg)]
            } else {
                Vec::new()
            }
        }
        KeyCode::Char(c) => {
            if c.is_alphabetic() || c == ' ' {
                vec![Message::TypingMessage(TypingMsg::InputCharMsg(c))]
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    model::{AppState, Model, TypingState},
    msg::{AppMsg, Message, TypingMsg},
};

pub fn handle_input(model: &mut Model) -> Result<Option<Message>, Box<dyn Error>> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(model, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(model: &mut Model, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => {
            if model.app_state == AppState::Setting {
                Some(Message::AppMessage(AppMsg::RunMsg))
            } else if model.app_state.is_running() {
                Some(Message::AppMessage(AppMsg::QuitMsg))
            } else {
                None
            }
        }
        KeyCode::Tab => {
            if model.app_state != AppState::Setting && model.app_state != AppState::Loading {
                Some(Message::AppMessage(AppMsg::SetMsg))
            } else {
                None
            }
        }
        KeyCode::Backspace => {
            if model.app_state.is_running() && model.enable_backspace {
                Some(Message::TypingMessage(TypingMsg::BackSpaceMsg))
            } else {
                None
            }
        }
        KeyCode::Char(c) => {
            if c.is_alphabetic() || c == ' ' {
                Some(Message::TypingMessage(TypingMsg::InputCharMsg(c)))
            } else {
                None
            }
        }
        _ => None,
    }
}

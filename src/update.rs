use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Seek},
    time::SystemTime,
};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use crate::{
    model::{AppState, Model, TypingState},
    msg::{AppMsg, Message, TypingMsg},
};

pub fn update(model: &mut Model, msg: Message) -> Vec<Message> {
    match msg {
        Message::TypingMessage(type_msg) => {
            if model.app_state.is_running() {
                handle_type_msg(model, type_msg)
            } else {
                Vec::new()
            }
        }
        Message::AppMessage(app_msg) => handle_app_msg(model, app_msg),
        Message::ReloadWordsMsg => reload_words(model),
        Message::CalAccuracyMsg => cal_accuracy(model),
        Message::CalWPMMsg => cal_wpm(model),
        Message::RestartTimerMsg => restart_timer(model),
        Message::WaitMsg => {
            model.app_state = AppState::Running(TypingState::Waiting);
            Vec::new()
        }
        Message::TypeMsg => {
            model.app_state = AppState::Running(TypingState::Typing);
            Vec::new()
        }
        Message::EmptyMsg => Vec::new(),
    }
}

fn handle_type_msg(model: &mut Model, type_msg: TypingMsg) -> Vec<Message> {
    match type_msg {
        TypingMsg::InputCharMsg(c) => {
            if c as u8 == model.current_words.as_bytes()[model.current_typed_len()] {
                vec![
                    Message::TypingMessage(TypingMsg::InputCorrectCharMsg(c)),
                    Message::RestartTimerMsg,
                    if model.app_state.is_waiting() {
                        Message::TypeMsg
                    } else {
                        Message::EmptyMsg
                    },
                ]
            } else {
                vec![
                    Message::TypingMessage(TypingMsg::InputWrongCharMsg(c)),
                    Message::RestartTimerMsg,
                    Message::RestartTimerMsg,
                    if model.app_state.is_waiting() {
                        Message::TypeMsg
                    } else {
                        Message::EmptyMsg
                    },
                ]
            }
        }
        TypingMsg::InputCorrectCharMsg(c) => {
            model.current_typed.push(c);
            model.num_correct += 1;
            if model.current_typed_len() == model.current_words_len() {
                // Finish a line
                vec![Message::ReloadWordsMsg, Message::CalAccuracyMsg]
            } else {
                vec![Message::CalAccuracyMsg]
            }
        }
        TypingMsg::InputWrongCharMsg(c) => {
            model.num_mistake += 1;
            if model.allow_typing_after_mistake {
                model.current_typed.push(c);
            }
            if model.current_typed_len() == model.current_words_len() {
                // Finish a line
                vec![Message::ReloadWordsMsg, Message::CalAccuracyMsg]
            } else {
                vec![Message::CalAccuracyMsg]
            }
        }
        TypingMsg::BackSpaceMsg => {
            if model.current_typed_len() != 0 {
                if model.current_typed.chars().last().unwrap() as u8
                    == model.current_words.as_bytes()[model.current_typed_len() - 1]
                {
                    model.num_correct -= 1;
                }
                model.current_typed.pop().unwrap();
                vec![Message::CalAccuracyMsg]
            } else {
                Vec::new()
            }
        }
    }
}

fn handle_app_msg(model: &mut Model, app_msg: AppMsg) -> Vec<Message> {
    match app_msg {
        AppMsg::QuitMsg => {
            model.app_state = AppState::Quiting;
            Vec::new()
        }
        AppMsg::RunMsg => {
            model.app_state = AppState::Running(TypingState::Waiting);
            Vec::new()
        }
        AppMsg::InfoMsg => {
            model.app_state = AppState::Info;
            Vec::new()
        }
        AppMsg::InitMsg => {
            // Load file buffers
            model.buf_readers = load_buff();

            // Load random words
            model.current_words =
                get_words(model, model.num_words_each_line).expect("Fail to get words");
            vec![Message::AppMessage(AppMsg::RunMsg)]
        }
    }
}

fn load_buff() -> Vec<BufReader<File>> {
    let mut res: Vec<BufReader<File>> = Vec::new();
    for entry in fs::read_dir("./words/").expect("Read directory error") {
        let path = entry.expect("Read file path error").path();
        let file = File::open(path).expect("Fail to open file");
        let reader = BufReader::new(file);
        res.push(reader);
    }
    res
}

fn cal_accuracy(model: &mut Model) -> Vec<Message> {
    model.accuracy = model.num_correct as f32 / (model.num_correct + model.num_mistake) as f32;
    Vec::new()
}

fn cal_wpm(model: &mut Model) -> Vec<Message> {
    model.wpm = model.num_words_finished as f32 / (model.time_elapsed.as_secs_f32() / 60.);
    Vec::new()
}

fn reload_words(model: &mut Model) -> Vec<Message> {
    model.current_words = get_words(model, model.num_words_each_line).expect("Fail to get words");
    model.current_typed = String::new();
    model.num_words_finished += model.num_words_each_line;
    vec![Message::CalWPMMsg]
}

pub fn get_words(model: &mut Model, amount: usize) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();
    for _ in 0..amount {
        let reader = model
            .buf_readers
            .choose_mut(&mut thread_rng())
            .expect("No file found in the buffer. You should put txt files which contain words inside \"words\" directory");
        reader.seek(std::io::SeekFrom::Start(0))?;
        res.push_str(&format!(
            "{} ",
            reader
                .lines()
                .map(|l| l.expect("Couldn't read line"))
                .choose(&mut thread_rng())
                .expect("No lines in file")
        ));
    }
    Ok(res)
}

fn restart_timer(model: &mut Model) -> Vec<Message> {
    model.timer = SystemTime::now();
    Vec::new()
}

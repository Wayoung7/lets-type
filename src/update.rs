use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{
    model::{AppState, Model, TypingState},
    msg::{AppMsg, Message, TypingMsg},
};

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::TypingMessage(type_msg) => {
            if model.app_state.is_running() {
                handle_type_msg(model, type_msg)
            } else {
                None
            }
        }
        Message::AppMessage(app_msg) => handle_app_msg(model, app_msg),
        Message::ReloadWordsMsg => reload_words(model),
        Message::CalAccuracyMsg => cal_accuracy(model),
        _ => None,
    }
}

fn handle_type_msg(model: &mut Model, type_msg: TypingMsg) -> Option<Message> {
    match type_msg {
        TypingMsg::InputCharMsg(c) => {
            if c as u8 == model.current_words.as_bytes()[model.current_typed_len()] {
                Some(Message::TypingMessage(TypingMsg::InputCorrectCharMsg(c)))
            } else {
                Some(Message::TypingMessage(TypingMsg::InputWrongCharMsg(c)))
            }
            // model.current_typed.push(c);
            // if c as u8 == model.current_words.as_bytes()[model.current_typed_len()] {
            //     model.num_correct += 1;
            // } else {
            //     model.num_mistake += 1;
            // }
            // if model.current_typed_len() == model.current_words_len() {
            //     // Finish a line
            //     Some(Message::ReloadWordsMsg)
            // } else {
            //     None
            // }

            // if c as u8 == model.current_words.as_bytes()[model.current_typed_len()] {
            //     // Typed correct word
            //     model.current_typed.push(c);
            //     model.num_correct += 1;
            //     if model.current_typed_len() == model.current_words_len() {
            //         // Finish a line
            //         Some(Message::ReloadWordsMsg)
            //     } else {
            //         None
            //     }
            // } else {
            //     // Typed wrong word
            //     if model.allow_typing_after_mistake {
            //         model.num_mistake += 1;
            //     } else {
            //         None
            //     }
            // }
        }
        TypingMsg::InputCorrectCharMsg(c) => {
            model.current_typed.push(c);
            model.num_correct += 1;
            if model.current_typed_len() == model.current_words_len() {
                // Finish a line
                Some(Message::ReloadWordsMsg)
            } else {
                Some(Message::CalAccuracyMsg)
            }
        }
        TypingMsg::InputWrongCharMsg(c) => {
            model.num_mistake += 1;
            if model.allow_typing_after_mistake {
                model.current_typed.push(c);
            }
            if model.current_typed_len() == model.current_words_len() {
                // Finish a line
                Some(Message::ReloadWordsMsg)
            } else {
                Some(Message::CalAccuracyMsg)
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
                None
            } else {
                None
            }
        }
    }
}

fn handle_app_msg(model: &mut Model, app_msg: AppMsg) -> Option<Message> {
    match app_msg {
        AppMsg::QuitMsg => {
            model.app_state = AppState::Quiting;
            None
        }
        AppMsg::RunMsg => {
            model.app_state = AppState::Running(TypingState::Waiting);
            None
        }
        AppMsg::SetMsg => {
            model.app_state = AppState::Setting;
            None
        }
        AppMsg::InitMsg => {
            // Load file buffers
            model.buf_readers = load_buff();

            // Load random words
            model.current_words = get_words(model, model.num_words_each_line).unwrap();
            Some(Message::AppMessage(AppMsg::RunMsg))
        }
    }
}

fn load_buff() -> Vec<BufReader<File>> {
    let mut res: Vec<BufReader<File>> = Vec::new();
    for ch in 'a'..'z' {
        let file = File::open(format!("words/{}.txt", ch)).unwrap();
        let reader = BufReader::new(file);
        res.push(reader);
    }
    res
}

fn cal_accuracy(model: &mut Model) -> Option<Message> {
    model.accuracy = model.num_correct as f32 / (model.num_correct + model.num_mistake) as f32;
    None
}

fn reload_words(model: &mut Model) -> Option<Message> {
    model.current_words = get_words(model, model.num_words_each_line).unwrap();
    model.current_typed = String::new();
    // Some(Message::AppMessage(AppMsg::RunMsg))
    None
}

pub fn get_words(model: &mut Model, amount: usize) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();
    if model.word_start_with != ' ' {
        let reader = &mut model.buf_readers[model.word_start_with as usize - 'a' as usize];
        reader.seek(std::io::SeekFrom::Start(0))?;
        // let mut index: Vec<i64> = (0..amount)
        //     .map(|_| thread_rng().gen_range(0..*num_lines as i64))
        //     .collect();
        reader
            .lines()
            .map(|l| l.expect("Couldn't read line"))
            .choose_multiple(&mut thread_rng(), amount)
            .iter()
            .for_each(|s| {
                res.push_str(&format!("{} ", s));
            });
        // let words = lines.choose_multiple(&mut thread_rng(), amount);
        // for word in words.iter() {
        //     res.push_str(&format!("{} ", word));
        // }
        // lines
        //     ;
        // println!("{m:?}");
    }
    Ok(res)
}

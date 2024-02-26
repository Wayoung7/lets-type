use std::{
    fs::File,
    io::BufReader,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
pub struct Model {
    pub app_state: AppState,
    pub buf_readers: Vec<BufReader<File>>,
    pub current_words: String,
    pub current_typed: String,
    pub allow_typing_after_mistake: bool,
    pub num_correct: i32,
    pub num_mistake: i32,
    pub enable_backspace: bool,
    pub num_words_each_line: usize,
    pub accuracy: f32,
    pub time_elapsed: Duration,
    pub num_words_finished: usize,
    pub wpm: f32,
    pub timer: SystemTime,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            app_state: AppState::Loading,
            buf_readers: Vec::new(),
            current_words: String::new(),
            current_typed: String::new(),
            allow_typing_after_mistake: true,
            num_correct: 0,
            num_mistake: 0,
            enable_backspace: true,
            num_words_each_line: 30,
            accuracy: 1.,
            time_elapsed: Duration::ZERO,
            num_words_finished: 0,
            wpm: 0.,
            timer: SystemTime::now(),
        }
    }
}

impl Model {
    pub fn current_words_len(&self) -> usize {
        self.current_words.chars().count()
    }

    pub fn current_typed_len(&self) -> usize {
        self.current_typed.chars().count()
    }
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    Running(TypingState),
    Quiting,
    Info,
    Loading,
}

impl AppState {
    pub fn is_running(&self) -> bool {
        *self == AppState::Running(TypingState::Typing)
            || *self == AppState::Running(TypingState::Waiting)
    }

    pub fn is_typing(&self) -> bool {
        *self == AppState::Running(TypingState::Typing)
    }

    pub fn is_waiting(&self) -> bool {
        *self == AppState::Running(TypingState::Waiting)
    }
}

#[derive(Debug, PartialEq)]
pub enum TypingState {
    Typing,
    Waiting,
}

#[derive(Debug)]
pub enum Message {
    AppMessage(AppMsg),
    TypingMessage(TypingMsg),
    ReloadWordsMsg,
    CalAccuracyMsg,
}

#[derive(Debug)]
pub enum AppMsg {
    QuitMsg,
    SetMsg,
    RunMsg,
    InitMsg,
}

#[derive(Debug)]
pub enum TypingMsg {
    InputCharMsg(char),
    InputCorrectCharMsg(char),
    InputWrongCharMsg(char),
    BackSpaceMsg,
}

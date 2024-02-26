#[derive(Debug)]
pub enum Message {
    AppMessage(AppMsg),
    TypingMessage(TypingMsg),
    ReloadWordsMsg,
    CalAccuracyMsg,
    CalWPMMsg,
    RestartTimerMsg,
    WaitMsg,
    TypeMsg,
    EmptyMsg,
}

#[derive(Debug)]
pub enum AppMsg {
    QuitMsg,
    InfoMsg,
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

use std::process::Command;

pub struct Terminal {
    pub logs: Vec<Log>,
    pub prompt: Prompt,
    pub state: TerminalState,
}

pub struct Log {
    pub command: Command,
    pub answer: CommandAnswer,
}

pub struct CommandAnswer {}

pub struct Prompt {}

pub enum TerminalState {}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Move {
        direction: Direction,
        distance: Distance,
    },
    FollowMe,
    Place {},
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Distance {
    Pixel(f32),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TerminalLogs(Vec<Command>);

impl From<Vec<Command>> for TerminalLogs {
    fn from(from: Vec<Command>) -> Self {
        Self(from)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalPrompt {
    Default,
}

impl Default for TerminalPrompt {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Terminal {
    pub logs: TerminalLogs,
    pub prompt: TerminalPrompt,
    pub command_input: String,
}

use yew::{html, Html};

const SUCCESS: &str = "#4dff4d";
const WARNING: &str = "#ffe400";
const FAILURE: &str = "#ff5050";

enum Status {
    Success,
    Warning,
    Failure,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Status::Success => SUCCESS,
            Status::Warning => WARNING,
            Status::Failure => FAILURE,
        })
    }
}

#[derive(Default, PartialEq)]
pub enum StatusKind {
    #[default]
    Initial,
    Connection,
    ServerDown,
    PlayerNotFound,
    Whitelisted,
    Success,
    InvalidInput,
    Unexpected,
}

impl std::fmt::Display for StatusKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Connection => "Failed to connect to server",
            Self::ServerDown => "Minecraft server is down",
            Self::PlayerNotFound => "Player doesn't exist",
            Self::Whitelisted => "Already whitelisted",
            Self::Success => "Success",
            Self::InvalidInput => "Invalid input",
            Self::Unexpected => "Unexpected server response",
            _ => unreachable!(),
        })
    }
}

impl From<&StatusKind> for Status {
    fn from(value: &StatusKind) -> Self {
        match value {
            StatusKind::Success | StatusKind::Whitelisted => Self::Success,
            StatusKind::PlayerNotFound | StatusKind::InvalidInput => Self::Warning,
            StatusKind::Connection | StatusKind::ServerDown | StatusKind::Unexpected => {
                Self::Failure
            }
            _ => unreachable!(),
        }
    }
}

impl StatusKind {
    pub const fn from_u8(byte: &u8) -> Self {
        match byte {
            0 => Self::ServerDown,
            1 => Self::PlayerNotFound,
            2 => Self::Whitelisted,
            3 => Self::Success,
            _ => Self::Unexpected,
        }
    }

    pub const fn is_new(&self) -> bool {
        !matches!(self, Self::Initial)
    }

    pub fn as_html(&self) -> Html {
        html! { <p color={ Status::from(self).to_string() }> { self.to_string() } </p> }
    }
}

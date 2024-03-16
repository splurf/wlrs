use dotenv::{dotenv, var};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread::spawn;
use tungstenite::handshake::HandshakeRole;
use tungstenite::{accept, Message};

type Result<T, E = Error> = std::result::Result<T, E>;

struct Error(String);

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self(value.to_string())
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Self(value.to_string())
    }
}

impl<T: HandshakeRole> From<tungstenite::HandshakeError<T>> for Error {
    fn from(value: tungstenite::HandshakeError<T>) -> Self {
        Self(value.to_string())
    }
}

impl From<dotenv::Error> for Error {
    fn from(value: dotenv::Error) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

fn handle_stream(stream: Result<TcpStream, std::io::Error>, pass: &str) -> Result<()> {
    let mut ws = accept(stream?)?;

    let msg = ws.read()?;
    let user = String::from_utf8(msg.into_data())?;

    let stdout = Command::new("mcrcon")
        .args(["-p", pass, format!("whitelist add {}", user).as_str()])
        .output()?
        .stdout;

    let status = if stdout.is_empty() {
        // "Minecraft server is down"
        0
    } else {
        let res = String::from_utf8_lossy(stdout.as_slice());

        if res.starts_with("That player does not exist") {
            1 // "Player doesn't exist"
        } else if res.starts_with("Player is already whitelisted") {
            2 // "Already whitelisted"
        } else if res.starts_with("Added") {
            3 // "Success"
        } else {
            4 // "Unexpected server response"
        }
    };
    ws.send(Message::Binary(vec![status])).map_err(Into::into)
}

fn main() -> Result<()> {
    dotenv()?;
    let addr = var("WLRS_SERVER_ADDR")?;
    let pass = var("RCON_PASS")?;

    let server = TcpListener::bind(addr)?;

    for stream in server.incoming() {
        let pass = pass.clone();

        spawn(move || {
            if let Err(e) = handle_stream(stream, &pass) {
                eprintln!("{}", e)
            }
        });
    }
    unreachable!()
}

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

fn handle_stream(stream: Result<TcpStream, std::io::Error>) -> Result<String> {
    let mut ws = accept(stream?)?;

    let msg = ws.read()?;
    let user = String::from_utf8(msg.into_data())?;

    let status = Command::new("mcrcon")
        .args([
            "-p",
            "8q94Jeeplamp1",
            format!("whitelist add {}", user).as_str(),
        ])
        .status()?;

    ws.send(Message::binary([status.success() as u8]))?;
    Ok(user)
}

fn main() -> Result<()> {
    let server = TcpListener::bind("localhost:8080")?;

    for stream in server.incoming() {
        println!("{:?}", stream);

        spawn(move || {
            if let Err(e) = handle_stream(stream) {
                eprintln!("{}", e)
            }
        });
    }
    unreachable!()
}

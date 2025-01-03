use super::{
    cmd::{cmd_echo, cmd_get, cmd_set},
    db::MemoryDatabase, respv2::{Parser, RESPv2Type, Serialize},
};

use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};
use tokio::sync::Mutex;

pub struct Redis;

type PeekableBoxes<'a> = std::iter::Peekable<std::slice::Iter<'a, Box<RESPv2Type>>>;

impl Redis {
    pub async fn handle(
        buffer: String,
        db: Arc<Mutex<impl MemoryDatabase>>,
    ) -> Result<String, Error> {
        if buffer.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Empty buffer received."));
        }

        let parse_result = buffer.try_parse_to_respv2();

        if parse_result.is_err() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                parse_result.unwrap_err().to_string(),
            ));
        }

        if let RESPv2Type::Array(vec) = parse_result.unwrap() {
            let mut itr = vec.iter().peekable();

            while let Some(type_box) = itr.next() {
                if let RESPv2Type::String(data) = type_box.as_ref() {
                    return Self::command_handler(data, &mut itr, db).await;
                }
            }
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid RESPv2 data received.",
        ))
    }

    async fn command_handler(
        data: &String,
        itr: &mut PeekableBoxes<'_>,
        db: Arc<Mutex<impl MemoryDatabase>>,
    ) -> Result<String, Error> {
        match data.to_lowercase().as_str() {
            "ping" => {
                return Ok("PONG".serialize_to_respv2());
            }
            "echo" => cmd_echo(itr.next()),
            "set" => cmd_set(itr.next(), itr.next(), &db).await,
            "get" => cmd_get(itr.next(), &db).await,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid command.")),
        }
    }
}

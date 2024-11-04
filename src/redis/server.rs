use tokio::sync::Mutex;

use crate::respv2::{Parser, RESPv2Types, Serialize};
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use super::db::MemoryDatabase;

pub struct Redis;

type PeekableBoxes<'a> = std::iter::Peekable<std::slice::Iter<'a, Box<RESPv2Types>>>;

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

        if let RESPv2Types::Array(vec) = parse_result.unwrap() {
            let mut itr = vec.iter().peekable();

            while let Some(type_box) = itr.next() {
                if let RESPv2Types::String(data) = type_box.as_ref() {
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
            "get" => cmt_get(itr.next(), &db).await,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid command.")),
        }
    }
}

async fn cmd_set(
    key: Option<&Box<RESPv2Types>>,
    value: Option<&Box<RESPv2Types>>,
    db: &Arc<Mutex<impl MemoryDatabase>>,
) -> Result<String, Error> {
    if let (Some(key), Some(value)) = (key, value) {
        if let (RESPv2Types::String(key), RESPv2Types::String(value)) =
            (key.as_ref(), value.as_ref())
        {
            let mut db = db.lock().await;
            let db = db.set(key, value);

            if db.is_err() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    db.unwrap_err().to_string(),
                ));
            }

            return Ok("OK".serialize_to_respv2());
        }
    }

    return Err(Error::new(
        ErrorKind::InvalidData,
        "SET command needs two arguments: SET [key] [value]",
    ));
}

async fn cmt_get(
    key: Option<&Box<RESPv2Types>>,
    db: &Arc<Mutex<impl MemoryDatabase>>,
) -> Result<String, Error> {
    if key.is_none() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "GET command needs another argument: GET [key]",
        ));
    }

    if let RESPv2Types::String(key) = key.unwrap().as_ref() {
        let db = db.lock().await;
        let db = db.get(key);

        if db.is_none() {
            return Ok("$-1\r\n".to_string());
        }

        return Ok(db.unwrap().serialize_to_respv2());
    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Wrong use of GET command.",
        ));
    }
}

fn cmd_echo(value: Option<&Box<RESPv2Types>>) -> Result<String, Error> {
    if value.is_none() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "ECHO command needs another argument: ECHO [message]",
        ));
    }

    if let RESPv2Types::String(echo) = value.unwrap().as_ref() {
        return Ok(echo.serialize_to_respv2());
    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Wrong use of ECHO command.",
        ));
    }
}

use crate::{
    redis::db::MemoryDatabase,
    respv2::{RESPv2Types, Serialize},
};
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};
use tokio::sync::Mutex;

pub async fn cmd_get(
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

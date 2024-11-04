use crate::{
    redis::db::MemoryDatabase,
    respv2::{RESPv2Types, Serialize},
};
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};
use tokio::sync::Mutex;

pub async fn cmd_set(
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
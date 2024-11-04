use crate::redis::respv2::{RESPv2Type, Serialize};
use std::io::{Error, ErrorKind};

pub fn cmd_echo(value: Option<&Box<RESPv2Type>>) -> Result<String, Error> {
    if value.is_none() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "ECHO command needs another argument: ECHO [message]",
        ));
    }

    if let RESPv2Type::String(echo) = value.unwrap().as_ref() {
        return Ok(echo.serialize_to_respv2());
    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Wrong use of ECHO command.",
        ));
    }
}

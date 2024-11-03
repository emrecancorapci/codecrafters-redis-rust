use crate::respv2::{Parser, RESPv2Types, Serialize};
use std::io::{Error, ErrorKind};

pub struct Redis;

impl Redis {
    pub async fn handle(buffer: String) -> Result<String, Error> {
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

        match parse_result.unwrap() {
            RESPv2Types::Array(vec) => {
                let mut itr = vec.iter().peekable();

                while let Some(data) = itr.next() {
                    match data.as_ref() {
                        RESPv2Types::Array(_) => todo!(),
                        RESPv2Types::Number(_) => todo!(),
                        RESPv2Types::String(str) => match str.to_lowercase().as_str() {
                            "ping" => {
                                return Ok("PONG".serialize_to_respv2());
                            }
                            "echo" => match itr.peek() {
                                Some(echo) => match echo.as_ref() {
                                    RESPv2Types::String(echo) => {
                                        return Ok(echo.serialize_to_respv2());
                                    }
                                    _ => {
                                        return Err(Error::new(
                                            ErrorKind::InvalidData,
                                            "Wrong use of ECHO command.",
                                        ))
                                    }
                                },
                                None => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidData,
                                        "ECHO command needs another argument: ECHO [message]",
                                    ))
                                }
                            },
                            _ => {
                                return Err(Error::new(ErrorKind::InvalidData, "Invalid command."))
                            }
                        },
                        RESPv2Types::Bulk(_) => todo!(),
                        RESPv2Types::Error(_) => todo!(),
                        RESPv2Types::Null => todo!(),
                    }
                }
            }
            RESPv2Types::Number(_) => todo!(),
            RESPv2Types::String(_) => todo!(),
            RESPv2Types::Bulk(_) => todo!(),
            RESPv2Types::Error(_) => todo!(),
            RESPv2Types::Null => todo!(),
        }

        Ok("".to_string())
    }
}

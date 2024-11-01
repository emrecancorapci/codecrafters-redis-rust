use std::fmt::Debug;

#[derive(PartialEq, Eq, Debug)]
pub enum RESPv2Types {
    Array(Vec<Box<RESPv2Types>>),
    Number(u64),
    String(String),
    Null,
}

pub enum RESPv2Errors {
    InvalidCommand,
    InvalidData,
    InvalidLength,
    InvalidType,
}

impl Debug for RESPv2Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for RESPv2Errors {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidCommand => String::from("InvalidCommand"),
            Self::InvalidData => String::from("InvalidData"),
            Self::InvalidLength => String::from("InvalidLength"),
            Self::InvalidType => String::from("InvalidType"),
        }
    }
}

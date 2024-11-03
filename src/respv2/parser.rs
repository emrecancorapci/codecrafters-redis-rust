use super::{RESPv2Errors, RESPv2Types};

pub trait Parser {
    fn try_parse_to_respv2(&self) -> Result<RESPv2Types, RESPv2Errors>;
}

impl Parser for &str {
    fn try_parse_to_respv2(&self) -> Result<RESPv2Types, RESPv2Errors> {
        RESPv2Parser::parse(self.to_string())
    }
}

impl Parser for String {
    fn try_parse_to_respv2(&self) -> Result<RESPv2Types, RESPv2Errors> {
        RESPv2Parser::parse(self.to_string())
    }
}

impl Parser for &String {
    fn try_parse_to_respv2(&self) -> Result<RESPv2Types, RESPv2Errors> {
        RESPv2Parser::parse(self.to_string())
    }
}

pub struct RESPv2Parser;

impl RESPv2Parser {
    pub fn parse(buffer: String) -> Result<RESPv2Types, RESPv2Errors> {
        let data = buffer.lines().collect::<Vec<&str>>();
        let mut iterator = data.iter();

        if data.len() == 0 || data[0].is_empty() {
            return Err(RESPv2Errors::InvalidLength);
        }

        let mut first_line = iterator.next().unwrap().chars();

        let operation = first_line.next().unwrap();
        let first_line = first_line.collect::<String>();

        match operation {
            '+' | '-' => Self::parse_string(first_line.as_str()),
            ':' => Self::parse_integer(first_line.as_str()),
            '$' => {
                if first_line.is_empty() {
                    Err(RESPv2Errors::InvalidData)
                } else if first_line == "0" {
                    Ok(RESPv2Types::String(String::from("")))
                } else if first_line == "-1" {
                    Ok(RESPv2Types::Null)
                } else {
                    Self::parse_string(iterator.next().unwrap())
                }
            }
            '*' => {
                if first_line.is_empty() || data.len() < 1 {
                    Err(RESPv2Errors::InvalidData)
                } else if first_line == "0" {
                    Ok(RESPv2Types::Array(vec![]))
                } else if first_line == "-1" {
                    Ok(RESPv2Types::Null)
                } else {
                    Self::parse_array(&mut iterator)
                }
            }
            _ => Err(RESPv2Errors::InvalidCommand),
        }
    }

    fn parse_string(data: &str) -> Result<RESPv2Types, RESPv2Errors> {
        Ok(RESPv2Types::String(data.to_string()))
    }

    fn parse_integer(data: &str) -> Result<RESPv2Types, RESPv2Errors> {
        match data.parse::<u64>() {
            Ok(num) => Ok(RESPv2Types::Number(num)),
            Err(_) => Err(RESPv2Errors::InvalidType),
        }
    }

    fn parse_array(iterator: &mut std::slice::Iter<'_, &str>) -> Result<RESPv2Types, RESPv2Errors> {
        let mut error = false;
        let mut error_type = RESPv2Errors::InvalidType;

        let mut array: Vec<Box<RESPv2Types>> = vec![];

        while let Some(line) = iterator.next() {
            if error {
                break;
            }

            let mut chars = line.chars();

            let operation = chars.next().unwrap();
            let num = chars.collect::<String>();

            if num.is_empty() {
                error = true;
                error_type = RESPv2Errors::InvalidData;
            }

            match operation {
                '+' | '-' => match Self::parse_string(num.as_str()) {
                    Ok(data) => array.push(Box::new(data)),
                    Err(e) => {
                        error = true;
                        error_type = e;
                    }
                },
                ':' => match Self::parse_integer(num.as_str()) {
                    Ok(data) => array.push(Box::new(data)),
                    Err(e) => {
                        error = true;
                        error_type = e;
                    }
                },
                '$' => {
                    if num == "0" {
                        array.push(Box::new(RESPv2Types::String(String::from(""))));
                    } else if num == "-1" {
                        array.push(Box::new(RESPv2Types::Null));
                    } else {
                        match Self::parse_string(iterator.next().unwrap()) {
                            Ok(data) => array.push(Box::new(data)),
                            Err(e) => {
                                error = true;
                                error_type = e;
                            }
                        }
                    }
                }
                '*' => {
                    if num == "0" {
                        array.push(Box::new(RESPv2Types::Array(vec![])));
                    } else if num == "-1" {
                        array.push(Box::new(RESPv2Types::Null));
                    } else {
                        match Self::parse_array(iterator) {
                            Ok(data) => array.push(Box::new(data)),
                            Err(e) => {
                                error = true;
                                error_type = e;
                            }
                        }
                    }
                }
                _ => {
                    error = true;
                    error_type = RESPv2Errors::InvalidCommand;
                }
            }
        }

        if error {
            Err(error_type)
        } else {
            Ok(RESPv2Types::Array(array))
        }
    }
}

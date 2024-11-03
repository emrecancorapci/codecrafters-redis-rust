use super::RESPv2Types;

pub trait Serialize {
    fn serialize_to_respv2(&self) -> String;
}

impl Serialize for u64 {
    fn serialize_to_respv2(&self) -> String {
        format!(":{}\r\n", self)
    }
}

impl Serialize for &u64 {
    fn serialize_to_respv2(&self) -> String {
        format!(":{}\r\n", self)
    }
}

impl Serialize for &str {
    fn serialize_to_respv2(&self) -> String {
        format!("+{}\r\n", self)
    }
}

impl Serialize for String {
    fn serialize_to_respv2(&self) -> String {
        format!("+{}\r\n", self)
    }
}

impl Serialize for &String {
    fn serialize_to_respv2(&self) -> String {
        format!("+{}\r\n", self)
    }
}

pub trait SerializeError {
    fn serialize_error_to_respv2(&self) -> String;
}

impl SerializeError for &str {
    fn serialize_error_to_respv2(&self) -> String {
        format!("-{}\r\n", self)
    }
}

impl SerializeError for String {
    fn serialize_error_to_respv2(&self) -> String {
        format!("-{}\r\n", self)
    }
}

impl SerializeError for &String {
    fn serialize_error_to_respv2(&self) -> String {
        format!("-{}\r\n", self)
    }
}

pub trait SerializeBulk {
    fn serialize_bulk_to_respv2(&self) -> String;
}

impl SerializeBulk for &str {
    fn serialize_bulk_to_respv2(&self) -> String {
        format!("${}\r\n{}\r\n", self.len(), self)
    }
}

impl SerializeBulk for String {
    fn serialize_bulk_to_respv2(&self) -> String {
        format!("${}\r\n{}\r\n", self.len(), self)
    }
}

impl SerializeBulk for &String {
    fn serialize_bulk_to_respv2(&self) -> String {
        format!("${}\r\n{}\r\n", self.len(), self)
    }
}

impl SerializeBulk for u64 {
    fn serialize_bulk_to_respv2(&self) -> String {
        format!("${}\r\n{}\r\n", self.to_string().len(), self)
    }
}

impl Serialize for Vec<Box<RESPv2Types>> {
    fn serialize_to_respv2(&self) -> String {
        if self.len() == 0 {
            return String::from("*0\r\n");
        } else if self.len() == 1 {
            return format!("*1\r\n{}", self[0].serialize_to_respv2());
        } else {
            return format!(
                "*{}\r\n{}",
                self.len(),
                self.iter()
                    .map(|x| x.serialize_to_respv2())
                    .collect::<String>()
            );
        }
    }
}

impl Serialize for RESPv2Types {
    fn serialize_to_respv2(&self) -> String {
        match self {
            RESPv2Types::Number(num) => num.serialize_to_respv2(),
            RESPv2Types::String(string) => string.serialize_to_respv2(),
            RESPv2Types::Error(error) => error.serialize_error_to_respv2(),
            RESPv2Types::Null => String::from("$-1\r\n"),
            RESPv2Types::Bulk(bulk) => bulk.serialize_bulk_to_respv2(),
            RESPv2Types::Array(array) => array.serialize_to_respv2(),
        }
    }
}

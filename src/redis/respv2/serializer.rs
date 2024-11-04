use super::RESPv2Type;

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

impl Serialize for Vec<Box<RESPv2Type>> {
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

impl Serialize for RESPv2Type {
    fn serialize_to_respv2(&self) -> String {
        match self {
            RESPv2Type::Number(num) => num.serialize_to_respv2(),
            RESPv2Type::String(string) => string.serialize_to_respv2(),
            RESPv2Type::Error(error) => error.serialize_error_to_respv2(),
            RESPv2Type::Null => String::from("$-1\r\n"),
            RESPv2Type::Bulk(bulk) => bulk.serialize_bulk_to_respv2(),
            RESPv2Type::Array(array) => array.serialize_to_respv2(),
        }
    }
}

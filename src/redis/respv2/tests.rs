use crate::redis::respv2::{Parser, RESPv2Parser, RESPv2Type};

#[test]
fn respv2_parser_string() {
    let data = String::from("+OK\r\n");
    let result = data.try_parse_to_respv2().unwrap();

    assert_eq!(result, RESPv2Type::String(String::from("OK")));
}

#[test]
fn respv2_parser_integer() {
    let data = String::from(":1000\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Type::Number(1000));
}

#[test]
fn respv2_parser_error() {
    let data = String::from("-ERR unknown command\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(
        result,
        RESPv2Type::String(String::from("ERR unknown command"))
    );
}

#[test]
fn respv2_parser_bulk_string() {
    let data = String::from("$6\r\nfoobar\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Type::String(String::from("foobar")));
}

#[test]
fn respv2_parser_null_bulk_string() {
    let data = String::from("$-1\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Type::Null);
}

#[test]
fn respv2_parser_empty_bulk_string() {
    let data = String::from("$0\r\n\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Type::String(String::from("")));
}

#[test]
fn respv2_parser_array() {
    let data = String::from("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(
        result,
        RESPv2Type::Array(vec![
            Box::new(RESPv2Type::String(String::from("foo"))),
            Box::new(RESPv2Type::String(String::from("bar"))),
        ])
    );
}

#[test]
fn respv2_parser_null_array() {
    let data = String::from("*-1\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Type::Null);
}

#[test]
fn respv2_parser_empty_array() {
    let data = String::from("*0\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Type::Array(vec![]));
}

#[test]
fn respv2_parser_invalid_data() {
    let data = String::from("$\r\n");
    let result = RESPv2Parser::parse(data);

    assert!(result.is_err() && result.unwrap_err().to_string() == "InvalidData");
}

#[test]
fn respv2_parser_invalid_command() {
    let data = String::from("!\r\n");
    let result = RESPv2Parser::parse(data);

    assert!(result.is_err() && result.unwrap_err().to_string() == "InvalidCommand");
}

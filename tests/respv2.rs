use redis_starter_rust::respv2::{RESPv2Parser, RESPv2Types};

#[test]
fn test_respv2_parser_string() {
    let data = String::from("+OK\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Types::String(String::from("OK")));
}

#[test]
fn test_respv2_parser_integer() {
    let data = String::from(":1000\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Types::Number(1000));
}

#[test]
fn test_respv2_parser_error() {
    let data = String::from("-ERR unknown command\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(
        result,
        RESPv2Types::String(String::from("ERR unknown command"))
    );
}

#[test]
fn test_respv2_parser_bulk_string() {
    let data = String::from("$6\r\nfoobar\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Types::String(String::from("foobar")));
}

#[test]
fn test_respv2_parser_null_bulk_string() {
    let data = String::from("$-1\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Types::Null);
}

#[test]
fn test_respv2_parser_empty_bulk_string() {
    let data = String::from("$0\r\n\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Types::String(String::from("")));
}

#[test]
fn test_respv2_parser_array() {
    let data = String::from("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(
        result,
        RESPv2Types::Array(vec![
            Box::new(RESPv2Types::String(String::from("foo"))),
            Box::new(RESPv2Types::String(String::from("bar"))),
        ])
    );
}

#[test]
fn test_respv2_parser_null_array() {
    let data = String::from("*-1\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Types::Null);
}

#[test]
fn test_respv2_parser_empty_array() {
    let data = String::from("*0\r\n");
    let result = RESPv2Parser::parse(data).unwrap();

    assert_eq!(result, RESPv2Types::Array(vec![]));
}

#[test]
fn test_respv2_parser_invalid_data() {
    let data = String::from("$\r\n");
    let result = RESPv2Parser::parse(data);

    assert!(result.is_err() && result.unwrap_err().to_string() == "InvalidData");
}

#[test]
fn test_respv2_parser_invalid_command() {
    let data = String::from("!\r\n");
    let result = RESPv2Parser::parse(data);

    assert!(result.is_err() && result.unwrap_err().to_string() == "InvalidCommand");
}

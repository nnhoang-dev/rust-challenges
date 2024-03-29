use std::{ collections::HashMap, iter::Peekable, str::Chars };

#[derive(PartialEq, Debug)]
pub enum Value {
    Number(i64),
    True,
    False,
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

// match character or not
pub fn match_char(src: &mut Peekable<Chars>, expected: char) -> Result<(), &'static str> {
    if src.next_if_eq(&expected).is_none() { Err("Do not match") } else { Ok(()) }
}

pub fn parse(input: &str) -> Result<Value, &'static str> {
    let remove_whitespace: String = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let mut src = remove_whitespace.chars().peekable();
    match src.peek().unwrap() {
        '[' => parse_array(&mut src),
        '{' => parse_object(&mut src),
        _ => Err("Can not parse"),
    }
}

pub fn parse_value(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    match src.peek() {
        Some('{') => parse_object(src),
        Some('[') => parse_array(src),
        Some('"') => parse_string(src),
        Some(_c) if _c.is_numeric() => parse_number(src),
        _ => parse_bool(src),
    }
}

pub fn parse_string(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    match_char(src, '"')?;
    let mut res = String::new();

    while let Some(_c) = src.next_if(|c| *c != '"') {
        res.push(_c);
    }
    match_char(src, '"')?;

    Ok(Value::String(res))
}

pub fn parse_number(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    let mut res: String = String::new();

    while let Some(_c) = src.next_if(|c| c.is_numeric()) {
        res.push(_c);
    }

    Ok(Value::Number(res.parse().expect("Can not parse")))
}

pub fn parse_bool(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    let mut res: String = String::new();

    while let Some(_c) = src.next_if(|c| *c != ',') {
        res.push(_c);
    }

    match res.as_str() {
        "true" => Ok(Value::True),
        "false" => Ok(Value::False),
        _ => Err("Can not parse"),
    }
}

pub fn parse_array(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    match_char(src, '[')?;

    if src.next_if_eq(&']').is_some() {
        return Ok(Value::Array(Vec::new()));
    }

    let mut array = Vec::new();

    loop {
        let value = parse_value(src)?;
        array.push(value);

        if let Some(_c) = src.next_if(|c| *c == ',') {
            continue;
        } else {
            break;
        }
    }
    match_char(src, ']')?;

    Ok(Value::Array(array))
}

pub fn parse_object(src: &mut Peekable<Chars>) -> Result<Value, &'static str> {
    match_char(src, '{')?;

    if src.next_if_eq(&'}').is_some() {
        return Ok(Value::Object(HashMap::new()));
    }

    let mut object = HashMap::new();

    loop {
        let key = parse_string(src)?;
        match_char(src, ':')?;
        let value = parse_value(src)?;
        if let Value::String(k) = key {
            object.insert(k, value);
        }

        if let Some(_c) = src.next_if(|c| *c == ',') {
            continue;
        } else {
            break;
        }
    }

    Ok(Value::Object(object))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let src = std::fs::read_to_string("tests/1.json").expect("Can not found test file");
        let parsed = parse(&src);
        let expectation = Value::Object(HashMap::new());
        assert_eq!(parsed, Ok(expectation));
    }

    #[test]
    fn test_2() {
        let src = std::fs::read_to_string("tests/2.json").expect("Can not found test file");
        let parsed = parse(&src);
        let mut body = HashMap::new();
        body.insert("key".to_string(), Value::String("value".to_string()));

        let expectation = Value::Object(body);
        assert_eq!(parsed, Ok(expectation));
    }

    #[test]
    fn test_3() {
        let src = std::fs::read_to_string("tests/3.json").expect("Can not found test file");
        let parsed = parse(&src);
        let mut body = HashMap::new();
        body.insert("key1".to_string(), Value::True);
        body.insert("key2".to_string(), Value::False);
        body.insert("key3".to_string(), Value::String("value".to_string()));
        body.insert("key4".to_string(), Value::Number(101));

        let expectation = Value::Object(body);
        assert_eq!(parsed, Ok(expectation));
    }

    #[test]
    fn test_4() {
        let src = std::fs::read_to_string("tests/4.json").expect("Can not found test file");
        let parsed = parse(&src);
        let mut body = HashMap::new();
        body.insert("key".to_string(), Value::String("value".to_string()));
        body.insert("key1".to_string(), Value::Number(101));
        body.insert("key2".to_string(), Value::Object(HashMap::new()));
        body.insert("key3".to_string(), Value::Array(vec![]));

        let expectation = Value::Object(body);
        assert_eq!(parsed, Ok(expectation));
    }

    #[test]
    fn test_5() {
        let src = std::fs::read_to_string("tests/5.json").expect("Can not found test file");
        let parsed = parse(&src);
        let mut body = HashMap::new();
        let payload_org = vec![
            Value::String("vbi".to_string()),
            Value::String("techfest".to_string())
        ];
        let mut payload_presenter = HashMap::new();
        payload_presenter.insert("name".to_string(), Value::String("Dung".to_string()));
        payload_presenter.insert("age".to_string(), Value::Number(27));
        payload_presenter.insert("occupation".to_string(), Value::String("Engineer".to_string()));

        body.insert("title".to_string(), Value::String("Rust".to_string()));
        body.insert("year".to_string(), Value::Number(2023));
        body.insert("live".to_string(), Value::True);
        body.insert("organizers".to_string(), Value::Array(payload_org));
        body.insert("presenter".to_string(), Value::Object(payload_presenter));

        let expectation = Value::Object(body);
        assert_eq!(parsed, Ok(expectation));
    }
}

use crate::parsec::*;
use std::{collections::HashMap, error::Error, fs, rc::Rc, str::Chars};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    pub(crate) name: String,
    pub(crate) attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Label {
    pub(crate) label: String,
    pub(crate) heading: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Label(Label),
    Tag(Tag),
    Text(String),
}

fn lift_tag(v: Vec<String>) -> Token {
    let mut attributes = HashMap::new();
    let name = v[0].clone();
    for attr in v[1..].iter() {
        let attr: Vec<&str> = attr.split('=').collect();
        attributes.insert(attr[0].to_string(), attr[1].to_string());
    }
    Token::Tag(Tag { name, attributes })
}

#[test]
fn test_parse_label() {
    let mut input = "*page12|".chars();
    let p = parse_label();
    assert_eq!(
        p(&mut input).unwrap(),
        Token::Label(Label {
            label: "page12".to_string(),
            heading: "page12".to_string(),
        })
    );
}

#[test]
fn test_parse_label_heading() {
    let mut input = "*page12|wakeup".chars();
    let p = parse_label();
    assert_eq!(
        p(&mut input).unwrap(),
        Token::Label(Label {
            label: "page12".to_string(),
            heading: "wakeup".to_string(),
        })
    );
}

fn parse_label() -> Parsec<Token> {
    Rc::new(|input: &mut Chars| {
        let label = between(parse_char('*'), string_none_of("|"), parse_char('|'))(input)?;
        let heading = if let Ok(Some(heading)) = optional(nonspace())(input) {
            heading
        } else {
            label.clone()
        };
        Ok(Token::Label(Label { label, heading }))
    })
}

#[test]
fn test_parse_line_tag() {
    let mut input = "@a2aT a=2".chars();
    let p = parse_line_tag();
    assert_eq!(
        p(&mut input).unwrap(),
        Token::Tag(Tag {
            name: "a2aT".to_string(),
            attributes: {
                let mut map = HashMap::new();
                map.insert("a".to_string(), "2".to_string());
                map
            }
        })
    );
}

#[test]
fn test_parse_quoted_tag() {
    let mut input = "@eval exp=\"sf.scriptresname = '桜ルート十二日目'\"".chars();
    let p = parse_line_tag();
    assert_eq!(
        p(&mut input).unwrap(),
        Token::Tag(Tag {
            name: "eval".to_string(),
            attributes: {
                let mut map = HashMap::new();
                map.insert(
                    "exp".to_string(),
                    "sf.scriptresname = '桜ルート十二日目'".to_string(),
                );
                map
            }
        })
    );
}

fn parse_line_tag() -> Parsec<Token> {
    Rc::new(|input: &mut Chars| {
        parse_char('@')(input)?;
        let name = word()(input)?;
        spaces()(input)?;
        let attributes = sep_by(parse_key_value(), spaces())(input)?
            .into_iter()
            .collect::<HashMap<String, String>>();
        Ok(Token::Tag(Tag { name, attributes }))
    })
}

#[test]
fn test_parse_key_value() {
    let mut input = "a=2".chars();
    let p = parse_key_value();
    assert_eq!(p(&mut input).unwrap(), ("a".to_string(), "2".to_string()));
}

#[test]
fn test_parse_quoted_key_value() {
    let mut input = "exp=\"sf.scriptresname = '桜ルート十二日目'\"".chars();
    let p = parse_key_value();
    assert_eq!(
        p(&mut input).unwrap(),
        (
            "exp".to_string(),
            "sf.scriptresname = '桜ルート十二日目'".to_string()
        )
    );
}

fn parse_key_value() -> Parsec<(String, String)> {
    Rc::new(|input: &mut Chars| {
        let key = string_none_of("= \r\t\n")(input)?;
        parse_char('=')(input)?;
        let value = quoted_string()(input)?;
        Ok((key, value))
    })
}

#[test]
fn test_quoted_string() {
    let mut input = "\"sf.scriptresname = '桜ルート十二日目'\"".chars();
    let p = quoted_string();
    assert_eq!(
        p(&mut input).unwrap(),
        "sf.scriptresname = '桜ルート十二日目'".to_string()
    );
}

#[test]
fn test_quoted_string_but_unquoted() {
    let mut input = "2".chars();
    let p = quoted_string();
    assert_eq!(p(&mut input).unwrap(), "2".to_string());
}

fn quoted_string() -> Parsec<String> {
    choice(vec![
        between(parse_char('"'), string_none_of("\""), parse_char('"')),
        string_none_of(" \r\t\n"),
    ])
}

#[test]
fn test_parse_inlined_tag() {
    let input = "[lr]";
    let p = parse_inlined_tag();
    assert_eq!(
        p(&mut input.chars()).unwrap(),
        Token::Tag(Tag {
            name: "lr".to_string(),
            attributes: HashMap::new(),
        })
    );
}

fn parse_inlined_tag() -> Parsec<Token> {
    between(
        parse_char('['),
        fmap(many1(try_parse(none_of("]"))), |v| {
            lift_tag(
                v.iter()
                    .collect::<String>()
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        }),
        lookahead(parse_char(']')),
    )
}

#[test]
fn test_parse_text_rune() {
    let mut input = "[[".chars();
    let p = parse_text_rune();
    assert_eq!(p(&mut input).unwrap(), '[');

    let mut input = "[lf]".chars();
    let p = parse_text_rune();
    assert_eq!(
        p(&mut input),
        Err(ParsecError {
            msg: ParsecErrorKind::UnexpectedChar('['),
        })
    );
}

fn parse_text_rune() -> Parsec<char> {
    choice(vec![
        fmap(string("[["), |_: String| '['),
        none_of("@[]"),
        fmap(string("]]"), |_| ']'),
        fmap(string("@@"), |_| '@'),
    ])
}

#[test]
fn test_parse_text() {
    let mut input = "hello world.[lr]".chars();
    let p = parse_text();
    assert_eq!(
        p(&mut input).unwrap(),
        Token::Text("hello world.".to_string())
    );
}

#[test]
fn test_parse_text_easy() {
    let mut input = "hello world.".chars();
    let p = parse_text();
    assert_eq!(
        p(&mut input).unwrap(),
        Token::Text("hello world.".to_string())
    );
}

fn parse_text() -> Parsec<Token> {
    Rc::new(|input: &mut Chars| {
        let result = many1(parse_text_rune())(input)
            .map(|v| v.into_iter().collect::<String>())
            .map(Token::Text);
        result
    })
}

#[test]
fn test_parse_token() {
    let page = "*page47|";
    let line_tag = "@a2aT file=o小さな公園-(曇)";
    let inlined_tag = "[lr]";
    let text = "Illya and I are alone in the small park a little way from the shopping district.";

    let p = parse_token();
    assert_eq!(
        p(&mut page.chars()).unwrap(),
        Token::Label(Label {
            label: "page47".to_string(),
            heading: "page47".to_string()
        })
    );
    assert_eq!(
        p(&mut line_tag.chars()).unwrap(),
        Token::Tag(Tag {
            name: "a2aT".to_string(),
            attributes: {
                let mut map = HashMap::new();
                map.insert("file".to_string(), "o小さな公園-(曇)".to_string());
                map
            }
        })
    );

    assert_eq!(
        p(&mut inlined_tag.chars()).unwrap(),
        Token::Tag(Tag {
            name: "lr".to_string(),
            attributes: HashMap::new(),
        })
    );

    assert_eq!(
        p(&mut text.chars()).unwrap(),
        Token::Text(
            "Illya and I are alone in the small park a little way from the shopping district."
                .to_string()
        )
    );
}

fn parse_token() -> Parsec<Token> {
    Rc::new(|input: &mut Chars| {
        spaces_and_newlines()(input)?;
        choice(vec![
            parse_label(),
            parse_line_tag(),
            parse_inlined_tag(),
            parse_text(),
        ])(input)
    })
}

#[test]
fn test_parse_tokens() {
    let mut input = "
    *page47|
    @sestop file=se009 time=1500 nowait=true
    Illya and I are alone in the small park a little way from the shopping district.[lr]
    @pg
    "
    .chars();
    let p = parse_tokens();
    assert_eq!(
        p(&mut input).unwrap(),
        vec![
            Token::Label(Label {
                label: "page47".to_string(),
                heading: "page47".to_string()
            }),
            Token::Tag(Tag {
                name: "sestop".to_string(),
                attributes: {
                    let mut map = HashMap::new();
                    map.insert("file".to_string(), "se009".to_string());
                    map.insert("time".to_string(), "1500".to_string());
                    map.insert("nowait".to_string(), "true".to_string());
                    map
                }
            }),
            Token::Text(
                "Illya and I are alone in the small park a little way from the shopping district."
                    .to_string()
            ),
            Token::Tag(Tag {
                name: "lr".to_string(),
                attributes: HashMap::new(),
            }),
            Token::Tag(Tag {
                name: "pg".to_string(),
                attributes: HashMap::new(),
            }),
        ]
    );
}

#[test]
fn test_parse_tokens_hard() {
    let input = "@download id=0000783
*page0|&f.scripttitle
@eval exp=\"sf.scriptresname = '桜ルート十二日目'\"";
    let p = parse_tokens();
    assert_eq!(
        p(&mut input.chars()).unwrap(),
        vec![
            Token::Tag(Tag {
                name: "download".to_string(),
                attributes: {
                    let mut map = HashMap::new();
                    map.insert("id".to_string(), "0000783".to_string());
                    map
                }
            }),
            Token::Label(Label {
                label: "page0".to_string(),
                heading: "&f.scripttitle".to_string()
            }),
            Token::Tag(Tag {
                name: "eval".to_string(),
                attributes: {
                    let mut map = HashMap::new();
                    map.insert(
                        "exp".to_string(),
                        "sf.scriptresname = '桜ルート十二日目'".to_string(),
                    );
                    map
                }
            }),
        ]
    );
}

fn parse_delimiter() -> Parsec<()> {
    choice(vec![
        newline(),
        discard(lookahead(one_of("@["))),
        discard(one_of("]")),
    ])
}

fn parse_tokens() -> Parsec<Vec<Token>> {
    Rc::new(|input: &mut Chars| {
        let result = sep_by(parse_token(), parse_delimiter())(input);
        result
    })
}

#[test]
fn test_parse_ks() {
    const KS: &str = "*page47|";
    fs::write("test.ks", KS).unwrap();
    let mut tokens = parse_ks("test.ks").unwrap();
    fs::remove_file("test.ks").unwrap();
    assert_eq!(
        tokens.next().unwrap(),
        Token::Label(Label {
            label: "page47".to_string(),
            heading: "page47".to_string()
        })
    );
}

pub fn parse_ks(path: &str) -> Result<impl Iterator<Item = Token>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let result = run_parser_str(parse_tokens(), input.as_str())?;
    Ok(result.into_iter())
}

#[test]
fn test_parse_ks_string() {
    let input = "@say storage=sak1209_shi_0010
    “O[line3]Oh yeah. It’s good if it’s decided. Sakura makes white stew, so let’s go look at the chicken meat.”";
    let tokens = parse_ks_string(input).unwrap();
    assert_eq!(
        tokens.collect::<Vec<Token>>(),
        vec![
            Token::Tag(Tag {
                name: "say".to_string(),
                attributes: {
                    let mut map = HashMap::new();
                    map.insert("storage".to_string(), "sak1209_shi_0010".to_string());
                    map
                }
            }),
            Token::Text(
                "“O"
                    .to_string()
            ),
            Token::Tag(Tag {
                name: "line3".to_string(),
                attributes: HashMap::new(),
            }),
            Token::Text(
                "Oh yeah. It’s good if it’s decided. Sakura makes white stew, so let’s go look at the chicken meat.”"
                    .to_string()
            ),
        ]
    );
}

pub fn parse_ks_string(input: &str) -> Result<impl Iterator<Item = Token>, Box<dyn Error>> {
    let result = run_parser_str(parse_tokens(), input)?;
    Ok(result.into_iter())
}

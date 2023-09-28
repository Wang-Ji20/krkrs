use crate::parsec::*;
use std::{collections::HashMap, rc::Rc, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
    name: String,
    attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Page(i64),
    Tag(Tag),
    Text(String),
}

fn lift_tag(v: Vec<String>) -> Tag {
    let mut attributes = HashMap::new();
    let name = v[0].clone();
    for attr in v[1..].iter() {
        let attr: Vec<&str> = attr.split('=').collect();
        attributes.insert(attr[0].to_string(), attr[1].to_string());
    }
    Tag { name, attributes }
}

#[test]
fn test_parse_page() {
    let mut input = "*page12|".chars();
    let p = parse_page();
    assert_eq!(p(&mut input).unwrap(), Token::Page(12));
}

fn parse_page() -> Parsec<Token> {
    Rc::new(|input: &mut Chars| {
        string("*page")(input)?;
        let page_num = fmap(dec_num(), Token::Page)(input);
        parse_char('|')(input)?;
        page_num
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

fn parse_line_tag() -> Parsec<Token> {
    Rc::new(|input: &mut Chars| {
        parse_char('@')(input)?;
        nonspaces()(input).map(lift_tag).map(Token::Tag)
    })
}

#[test]
fn test_parse_inlined_tag() {
    let mut input = "[lr]";
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
    Rc::new(|input: &mut Chars| {
        parse_char('[')(input)?;
        let result = many1(try_parse(none_of("]")))(input)
            .map(|v| {
                v.iter()
                    .collect::<String>()
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .map(lift_tag)
            .map(Token::Tag);
        parse_char(']')(input)?;
        result
    })
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
        fmap(string("[["), |v: String| '['),
        none_of("@[]"),
        fmap(string("]]"), |v| ']'),
        fmap(string("@@"), |v| '@'),
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
    assert_eq!(p(&mut page.chars()).unwrap(), Token::Page(47));
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
            parse_page(),
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
    @textoff
    @sestop file=se009 time=1500 nowait=true
    @a2aT file=o小さな公園-(曇)
    @play file=bgm05 time=0
    @texton
    Illya and I are alone in the small park a little way from the shopping district.[lr]
    Maybe all the children are at school or maybe such a small park isn’t popular anymore.[lr]
    We start to talk in the empty winter park, bathed in a strangely tense atmosphere.
    @pg
    "
    .chars();
    let p = parse_tokens();
    println!("{:?}", p(&mut input).unwrap());
}

fn parse_delimiter() -> Parsec<()> {
    choice(vec![newline(), discard(lookahead(one_of("@[")))])
}

fn parse_tokens() -> Parsec<Vec<Token>> {
    Rc::new(|input: &mut Chars| {
        let result = sep_by(parse_token(), parse_delimiter())(input);
        result
    })
}

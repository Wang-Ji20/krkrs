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
        let v: Vec<&str> = attr.split('=').collect();
        attributes.insert(v[0].to_string(), v[1].to_string());
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

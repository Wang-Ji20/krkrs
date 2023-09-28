use std::rc::Rc;

use super::*;

#[test]
pub(crate) fn test_char() {
    let mut input = "你好世界".chars();
    let p = parse_char('你');
    assert_eq!(p(&mut input).unwrap(), '你');
}

pub fn parse_char(c: char) -> Parsec<char> {
    satisfy(Rc::new(move |x| x == c))
}

#[test]
pub(crate) fn test_one_of() {
    let mut input = "你好世界".chars();
    let p = one_of("你好".to_string());
    assert_eq!(p(&mut input).unwrap(), '你');
    let mut input = "好世界".chars();
    assert_eq!(p(&mut input).unwrap(), '好');
}

pub fn one_of(cs: String) -> Parsec<char> {
    satisfy(Rc::new(move |x| cs.contains(x)))
}

#[test]
pub(crate) fn test_space() {
    let mut input = "  你好世界".chars();
    let p = space();
    assert_eq!(p(&mut input).unwrap(), ' ');
    assert_eq!(p(&mut input).unwrap(), ' ');
}

pub fn space() -> Parsec<char> {
    satisfy(Rc::new(move |x| x.is_whitespace()))
}

#[test]
pub(crate) fn test_spaces() {
    let mut input = "  你好世界".chars();
    let p = spaces();
    assert_eq!(p(&mut input).unwrap(), ());
}

pub fn spaces() -> Parsec<()> {
    skip_many(satisfy(Rc::new(move |x| {
        x.is_whitespace() || x == '\u{3000}'
    })))
}

#[test]
fn test_none_of() {
    let mut input = "j你好".chars();
    let p = none_of("你好".to_string());
    assert_eq!(p(&mut input).unwrap(), 'j');
    assert_eq!(
        p(&mut input),
        Err(ParsecError {
            msg: ParsecErrorKind::UnexpectedChar('你')
        })
    );
}

pub fn none_of(s: String) -> Parsec<char> {
    satisfy(Rc::new(move |c: char| !s.contains(c)))
}

#[test]
fn test_digit() {
    let mut input = "123".chars();
    let p = digit();
    assert_eq!(p(&mut input).unwrap(), '1');
    assert_eq!(p(&mut input).unwrap(), '2');
    assert_eq!(p(&mut input).unwrap(), '3');
}

pub fn digit() -> Parsec<char> {
    satisfy(Rc::new(move |c: char| c.is_digit(10)))
}

#[test]
fn test_dec_num() {
    let mut input = "123".chars();
    let p = dec_num();
    assert_eq!(p(&mut input).unwrap(), 123);
}

pub fn dec_num() -> Parsec<i64> {
    Rc::new(move |input: &mut Chars| {
        let try_digit = try_parse(digit());
        let digits = many1(try_digit)(input)?;
        Ok(digits.iter().collect::<String>().parse::<i64>().unwrap())
    })
}

#[test]
fn test_letter() {
    let mut input = "abc".chars();
    let p = letter();
    assert_eq!(p(&mut input).unwrap(), 'a');
    assert_eq!(p(&mut input).unwrap(), 'b');
    assert_eq!(p(&mut input).unwrap(), 'c');
}

pub fn letter() -> Parsec<char> {
    satisfy(Rc::new(move |c: char| c.is_alphabetic()))
}

#[test]
fn test_alpha_num() {
    let mut input = "abc123".chars();
    let p = alpha_num();
    assert_eq!(p(&mut input).unwrap(), 'a');
    assert_eq!(p(&mut input).unwrap(), 'b');
    assert_eq!(p(&mut input).unwrap(), 'c');
    assert_eq!(p(&mut input).unwrap(), '1');
    assert_eq!(p(&mut input).unwrap(), '2');
    assert_eq!(p(&mut input).unwrap(), '3');
}

pub fn alpha_num() -> Parsec<char> {
    satisfy(Rc::new(move |c: char| c.is_alphanumeric()))
}

#[test]
fn test_lf() {
    let mut input = "\n".chars();
    let p = lf();
    assert_eq!(p(&mut input).unwrap(), '\n');
}

pub fn lf() -> Parsec<char> {
    satisfy(Rc::new(move |c: char| c == '\n'))
}

#[test]
fn test_string() {
    let mut input = "你好世界".chars();
    let p = string("你好");
    assert_eq!(p(&mut input).unwrap(), "你好".to_string());
}

pub fn string(s: &'static str) -> Parsec<String> {
    Rc::new(move |input: &mut Chars| {
        input
            .take(s.len())
            .zip(s.chars())
            .map(|(x, y)| {
                if x == y {
                    Ok(x)
                } else {
                    Err(ParsecError {
                        msg: ParsecErrorKind::UnexpectedChar(x),
                    })
                }
            })
            .collect::<Result<String, ParsecError>>()
    })
}

#[test]
fn test_crlf() {
    let mut input = "\r\na".chars();
    let p = crlf();
    assert_eq!(p(&mut input).unwrap(), "\r\n".to_string());
}

pub fn crlf() -> Parsec<String> {
    string("\r\n")
}

#[test]
fn test_newline() {
    let mut input = "\n".chars();
    let p = newline();
    assert_eq!(p(&mut input).unwrap(), ());

    let mut input = "\r\n".chars();
    assert_eq!(p(&mut input).unwrap(), ());
}

pub fn newline() -> Parsec<()> {
    choice(vec![discard(crlf()), discard(lf())])
}

#[test]
fn test_parse_word() {
    let mut input = "abuhskh hjjh1hh".chars();
    let p = word();
    assert_eq!(p(&mut input).unwrap(), "abuhskh".to_string());
    input.next();
    assert_eq!(p(&mut input).unwrap(), "hjjh1hh".to_string());
}

pub fn word() -> Parsec<String> {
    fmap(many1(alpha_num()), |v| v.into_iter().collect::<String>())
}

#[test]
fn test_parse_nonspace() {
    let mut input = "abuh#@#skh".chars();
    let p = nonspace();
    assert_eq!(p(&mut input).unwrap(), "abuh#@#skh".to_string());
}

pub fn nonspace() -> Parsec<String> {
    fmap(many1(none_of(" \t\r\n".to_string())), |v| {
        v.into_iter().collect::<String>()
    })
}

#[test]
fn test_parse_nonspaces() {
    let mut input = "abuh#@#skh   $%%^&".chars();
    let p = nonspaces();
    assert_eq!(
        p(&mut input).unwrap(),
        vec!["abuh#@#skh".to_string(), "$%%^&".to_string()]
    );
}

pub fn nonspaces() -> Parsec<Vec<String>> {
    sep_by(nonspace(), spaces())
}

#[test]
fn test_words() {
    let mut input = "abuhskh hjjh1hh".chars();
    let p = words();
    assert_eq!(
        p(&mut input).unwrap(),
        vec!["abuhskh".to_string(), "hjjh1hh".to_string()]
    );
}

pub fn words() -> Parsec<Vec<String>> {
    sep_by(word(), space())
}

//! # Combinator
//!
//! Combinator submodule provides functions that combines parsers.
//! for example, we use `between` to construct a parser that parses something between `left` and `right`.

use crate::parsec::*;
use std::{error::Error, fmt::Display, rc::Rc, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub struct ParsecError {
    pub(crate) msg: ParsecErrorKind,
}

impl Error for ParsecError {}

impl Display for ParsecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.msg {
            ParsecErrorKind::UnexpectedEOF => write!(f, "Unexpected EOF"),
            ParsecErrorKind::UnexpectedChar(c) => write!(f, "Unexpected char {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsecErrorKind {
    UnexpectedEOF,
    UnexpectedChar(char),
}

/// A parser is a function that takes a `Chars` iterator and returns a `Result<T, ParsecError>`.
/// the iterator is mutable because the parser may consume the input, but of course it will not
/// modify the input.
pub type Parsec<T> = Rc<dyn Fn(&mut Chars) -> Result<T, ParsecError>>;

#[test]
fn test_satisfy() {
    let mut input = "你好世界".chars();
    let p = satisfy(Rc::new(move |x| x == '你'));
    assert_eq!(p(&mut input).unwrap(), '你');
}

/// the parser goes ahead regradless success or failure.
pub fn satisfy(p: Rc<dyn Fn(char) -> bool>) -> Parsec<char> {
    Rc::new(move |input: &mut Chars| {
        let next = input.next().ok_or(ParsecError {
            msg: ParsecErrorKind::UnexpectedEOF,
        })?;
        if p(next) {
            Ok(next)
        } else {
            Err(ParsecError {
                msg: ParsecErrorKind::UnexpectedChar(next),
            })
        }
    })
}

#[test]
fn test_lookahead() {
    let mut input = "你好世界".chars();
    let p = lookahead(parse_char('你'));
    assert_eq!(p(&mut input).unwrap(), '你');
    assert_eq!(input.next().unwrap(), '你');
}

/// the parser peeks the parse result, but do not consume the input.
/// thus named `lookahead`.
pub fn lookahead<T: 'static>(p: Parsec<T>) -> Parsec<T> {
    Rc::new(move |input: &mut Chars| {
        let mut input_clone = input.clone();
        let result = p(&mut input_clone);
        result
    })
}

#[test]
fn test_try_parse() {
    let mut input = "你好世界".chars();
    let p = try_parse(parse_char('您'));
    assert_eq!(
        p(&mut input).unwrap_err().msg,
        ParsecErrorKind::UnexpectedChar('你')
    );
    assert_eq!(input.next().unwrap(), '你');
}

/// the parser will not consume the input if it fails.
pub fn try_parse<T: 'static>(p: Parsec<T>) -> Parsec<T> {
    Rc::new(move |input: &mut Chars| {
        let mut input_clone = input.clone();
        match p(&mut input_clone) {
            Ok(x) => {
                *input = input_clone;
                Ok(x)
            }
            Err(e) => Err(e),
        }
    })
}

#[test]
fn test_choice() {
    let mut input = "你好世界".chars();
    let p = choice(vec![parse_char('你'), parse_char('您'), parse_char('好')]);
    assert_eq!(p(&mut input).unwrap(), '你');
    assert_eq!(p(&mut input).unwrap(), '好');
    assert_eq!(
        p(&mut input),
        Err(ParsecError {
            msg: ParsecErrorKind::UnexpectedChar('世')
        })
    );
}

/// the parser will try every parser in the vector, and return the first success.
/// upon success, the input will be consumed.
pub fn choice<T: 'static>(ps: Vec<Parsec<T>>) -> Parsec<T> {
    Rc::new(move |input: &mut Chars| {
        ps.clone().into_iter().map(try_parse).fold(
            Err(ParsecError {
                msg: ParsecErrorKind::UnexpectedEOF,
            }),
            |acc, p| match acc {
                Ok(x) => Ok(x),
                Err(_) => p(input),
            },
        )
    })
}

#[test]
fn test_many() {
    let mut input = "你你你你你你".chars();
    let p = many(parse_char('你'));
    assert_eq!(
        p(&mut input).unwrap(),
        vec!['你', '你', '你', '你', '你', '你']
    );
}

/// the parser will try to parse the input as many times as possible.
/// upon failure, the input will not be consumed.
pub fn many<T: 'static>(p: Parsec<T>) -> Parsec<Vec<T>> {
    Rc::new(move |input: &mut Chars| {
        let mut result = vec![];
        let many_parser = try_parse(p.clone());
        loop {
            match many_parser(input) {
                Ok(x) => result.push(x),
                Err(_) => break,
            }
        }
        Ok(result)
    })
}

#[test]
fn test_many1() {
    let mut input = "好".chars();
    let p = many1(parse_char('你'));
    assert_eq!(
        p(&mut input),
        Err(ParsecError {
            msg: ParsecErrorKind::UnexpectedChar('好')
        })
    );
}

/// the parser will try to parse the input as many times as possible, but more than once
pub fn many1(p: Parsec<char>) -> Parsec<Vec<char>> {
    Rc::new(move |input: &mut Chars| {
        let mut result = vec![];
        let many1_parser = try_parse(p.clone());
        match many1_parser(input) {
            Ok(x) => result.push(x),
            Err(e) => return Err(e),
        }
        loop {
            match many1_parser(input) {
                Ok(x) => result.push(x),
                Err(_) => break,
            }
        }
        Ok(result)
    })
}

#[test]
fn test_skip_many() {
    let mut input = "你你你你你你好".chars();
    let p = skip_many(parse_char('你'));
    assert_eq!(p(&mut input).unwrap(), ());
    let p2 = parse_char('好');
    assert_eq!(p2(&mut input).unwrap(), '好');
}

/// the parser will skip all seq appearence of pattern p
pub fn skip_many<T: 'static>(p: Parsec<T>) -> Parsec<()> {
    Rc::new(move |input: &mut Chars| {
        let many_parser = try_parse(p.clone());
        loop {
            match many_parser(input) {
                Ok(_) => (),
                Err(_) => break,
            }
        }
        Ok(())
    })
}

#[test]
fn test_skip_many1() {
    let mut input = "你你你你你你好".chars();
    let p = skip_many1(parse_char('你'));
    assert_eq!(p(&mut input).unwrap(), ());
    let p2 = parse_char('好');
    assert_eq!(p2(&mut input).unwrap(), '好');
}

/// the parser will skip all seq appearence of pattern p, but more than once
pub fn skip_many1(p: Parsec<char>) -> Parsec<()> {
    Rc::new(move |input: &mut Chars| {
        let p_parser = try_parse(p.clone());
        match p_parser(input) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        loop {
            match p_parser(input) {
                Ok(_) => (),
                Err(_) => break,
            }
        }
        Ok(())
    })
}

#[test]
fn test_sep_by() {
    let mut input = "你,你,你,你,你,你,你".chars();
    let p = sep_by(parse_char('你'), parse_char(','));
    assert_eq!(
        p(&mut input).unwrap(),
        vec!['你', '你', '你', '你', '你', '你', '你']
    );
}

/// the parser will parse ps separated by seps
pub fn sep_by<T: 'static, U: 'static>(p: Parsec<T>, sep: Parsec<U>) -> Parsec<Vec<T>> {
    Rc::new(move |input: &mut Chars| {
        let t_parser = try_parse(p.clone());
        let sep_parser = try_parse(sep.clone());
        let mut result = vec![];
        match t_parser(input) {
            Ok(x) => result.push(x),
            Err(_) => return Ok(result),
        }
        loop {
            match sep_parser(input) {
                Ok(_) => (),
                Err(_) => break,
            }
            match t_parser(input) {
                Ok(x) => result.push(x),
                Err(_) => break,
            }
        }
        Ok(result)
    })
}

#[test]
fn test_fmap() {
    let mut input = "1".chars();
    let p = fmap(parse_char('1'), |x| x.to_digit(10).unwrap());
    assert_eq!(p(&mut input).unwrap(), 1);
}

/// the function can convert the result of a parser from type T to type U
pub fn fmap<T: 'static, U: 'static>(p: Parsec<T>, f: fn(T) -> U) -> Parsec<U> {
    Rc::new(move |input: &mut Chars| p(input).map(f))
}

#[test]
fn test_bind() {
    let mut input = "12".chars();
    let p = bind(parse_char('1'), Rc::new(|_| parse_char('2')));
    assert_eq!(p(&mut input).unwrap(), '2');
}

/// Monad m => m a -> (a -> m b) -> m b
pub fn bind<T: 'static, U: 'static>(p: Parsec<T>, f: Rc<dyn Fn(T) -> Parsec<U>>) -> Parsec<U> {
    Rc::new(move |input: &mut Chars| {
        let x = p(input)?;
        f(x)(input)
    })
}

#[test]
fn test_pure() {
    let mut input = "".chars();
    let p = pure(1);
    assert_eq!(p(&mut input).unwrap(), 1);
}

/// this function returns a parser that always returns x
pub fn pure<T: 'static + Clone>(x: T) -> Parsec<T> {
    Rc::new(move |_: &mut Chars| Ok(x.clone()))
}

#[test]
fn test_discard() {
    let mut input = "1".chars();
    let p = discard(parse_char('1'));
    assert_eq!(p(&mut input).unwrap(), ());
}

/// this function discards the result of a parser
pub fn discard<T: 'static>(p: Parsec<T>) -> Parsec<()> {
    fmap(p, |_| ())
}

#[test]
fn test_between() {
    let mut input = "*page34|".chars();
    let p = between(string("*page"), dec_num(), parse_char('|'));
    assert_eq!(p(&mut input).unwrap(), 34);
}

/// this function parses something between left and right
pub fn between<T: 'static, U: 'static, V: 'static>(
    open: Parsec<T>,
    p: Parsec<U>,
    close: Parsec<V>,
) -> Parsec<U> {
    Rc::new(move |input: &mut Chars| {
        open(input)?;
        let result = p(input)?;
        close(input)?;
        Ok(result)
    })
}

#[test]
fn test_optional() {
    let mut input = "你好".chars();
    let p = optional(parse_char('你'));
    assert_eq!(p(&mut input).unwrap(), Some('你'));
    assert_eq!(p(&mut input).unwrap(), None);
}

/// does not consume the input if the parser fails
pub fn optional<T: 'static>(p: Parsec<T>) -> Parsec<Option<T>> {
    Rc::new(move |input: &mut Chars| {
        let mut input_clone = input.clone();
        match p(&mut input_clone) {
            Ok(x) => {
                *input = input_clone;
                Ok(Some(x))
            }
            Err(_) => Ok(None),
        }
    })
}

#[macro_export]
macro_rules! mkpc {
    ($binding:ident <- $x:expr; $($y:tt)*) => {
        bind($x, Rc::new(move |$binding| { mkpc!($($y)*) }))
    };

    ($x:expr; $($y:tt)*) => {
        bind($x, Rc::new(move |_| { mkpc!($($y)*) }))
    };

    ($st:stmt; $($y:tt)*) => {{
        $st;
        mkpc!($($y)*)
    }};

    (return $x:expr) => {
        pure($x)
    };

    ($x:expr) => {
        $x
    };

    () => {}
}

#[test]
fn test_pc() {
    let mut input = "12|3".chars();
    let p: Parsec<u8> = mkpc![
        x <- parse_char('1');
        y <- parse_char('2');
        parse_char('|');
        z <- parse_char('3');
        return x.to_digit(10).unwrap() as u8 + y.to_digit(10).unwrap() as u8 + z.to_digit(10).unwrap() as u8
    ];
    assert_eq!(p(&mut input).unwrap(), 6);
}

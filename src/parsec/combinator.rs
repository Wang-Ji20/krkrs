use crate::parsec::*;
use std::{rc::Rc, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub struct ParsecError {
    pub(crate) msg: ParsecErrorKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsecErrorKind {
    UnexpectedEOF,
    UnexpectedChar(char),
}

pub type Parsec<T> = Rc<dyn Fn(&mut Chars) -> Result<T, ParsecError>>;

#[test]
fn test_satisfy() {
    let mut input = "你好世界".chars();
    let p = satisfy(Rc::new(move |x| x == '你'));
    assert_eq!(p(&mut input).unwrap(), '你');
}

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

pub fn many<T: 'static>(p: Parsec<T>) -> Parsec<Vec<T>> {
    Rc::new(move |input: &mut Chars| {
        let mut result = vec![];
        loop {
            match try_parse(p.clone())(input) {
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

pub fn many1(p: Parsec<char>) -> Parsec<Vec<char>> {
    Rc::new(move |input: &mut Chars| {
        let mut result = vec![];
        match try_parse(p.clone())(input) {
            Ok(x) => result.push(x),
            Err(e) => return Err(e),
        }
        loop {
            match try_parse(p.clone())(input) {
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

pub fn fmap<T: 'static, U: 'static>(p: Parsec<T>, f: fn(T) -> U) -> Parsec<U> {
    Rc::new(move |input: &mut Chars| p(input).map(f))
}

#[test]
fn test_discard() {
    let mut input = "1".chars();
    let p = discard(parse_char('1'));
    assert_eq!(p(&mut input).unwrap(), ());
}

pub fn discard<T: 'static>(p: Parsec<T>) -> Parsec<()> {
    fmap(p, |_| ())
}

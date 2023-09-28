//! Parsec is a parsing combinator library for this project.

pub mod char;
pub use char::*;

pub mod combinator;
pub use combinator::*;

use std::str::Chars;

#[test]
fn test_runparser() {
    let mut input = "你".chars();
    let p = parse_char('你');
    assert_eq!(run_parser(p, &mut input).unwrap(), '你');
}

pub fn run_parser<T>(parser: Parsec<T>, input: &mut Chars) -> Result<T, ParsecError> {
    parser(input)
}

#[test]
fn test_runparser_str() {
    let p = parse_char('你');
    assert_eq!(run_parser_str(p, "你").unwrap(), '你');
}

pub fn run_parser_str<T>(parser: Parsec<T>, input: &str) -> Result<T, ParsecError> {
    run_parser(parser, &mut input.chars())
}

#[cfg(test)]
mod tests {
    const TEST_TEXT: &str = "
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
    ";

    use super::*;

    #[test]
    fn test_char() {
        let linebreak_parser = sep_by(
            fmap(many(none_of("\n".to_string())), |v| {
                v.into_iter().collect::<String>()
            }),
            lf(),
        );
        let mut input = TEST_TEXT.chars();
        println!("{:?}", linebreak_parser(&mut input));
    }
}

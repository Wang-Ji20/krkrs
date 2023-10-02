use crate::interpreter::parser::*;
use std::fmt::{self, Debug, Formatter};

pub struct State {
    page: i64,
    music: String,
    scene: Vec<String>,
    text: Vec<String>,
    cur_token: Option<Token>,
    tokens: Box<dyn Iterator<Item = Token>>,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("page", &self.page)
            .field("music", &self.music)
            .field("scene", &self.scene)
            .field("text", &self.text)
            .finish()
    }
}

#[derive(Debug)]
pub enum Command {
    Preceed,
    Choose(i64),
}

impl State {
    pub fn new_from_ks(filename: &str) -> State {
        let mut s = State {
            page: 0,
            music: String::new(),
            scene: Vec::new(),
            text: Vec::new(),
            cur_token: None,
            tokens: Box::new(parse_ks(filename).unwrap()),
        };
        s.eval();
        s
    }

    pub fn eval(&mut self) {
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Page(n) => {
                    self.page = n;
                }
                Token::Tag(tag) => match tag.name.as_str() {
                    "lr" | "pg" => break,
                    _ => continue,
                },
                Token::Text(text) => {
                    self.text.push(text);
                }
            }
        }
    }

    pub fn eval_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Preceed => {
                if let Some(Token::Tag(tag)) = &self.cur_token {
                    if tag.name == "pg" {
                        self.scene.clear();
                        self.text.clear();
                    }
                }
                self.eval();
            }
            Command::Choose(n) => {
                self.page = n;
                self.eval();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}

use crate::interpreter::parser::*;
use std::fmt::{self, Debug, Formatter};

pub struct State {
    label: Label,
    music: String,
    scene: Vec<String>,
    text: Vec<String>,
    cur_token: Option<Token>,
    tokens: Box<dyn Iterator<Item = Token>>,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("label", &self.label)
            .field("music", &self.music)
            .field("scene", &self.scene)
            .field("text", &self.text)
            .finish()
    }
}

#[derive(Debug)]
pub enum Command {
    Preceed,
    Choose(Label),
}

impl State {
    pub fn new_from_ks(filename: &str) -> State {
        let mut s = State {
            label: Label {
                label: String::new(),
                heading: String::new(),
            },
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
            self.cur_token = Some(token.clone());
            match token {
                Token::Label(l) => {
                    self.label = l;
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
            Command::Choose(l) => {
                self.label = l;
                self.eval();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// This test only runs in my local machine.
    #[ignore]
    #[test]
    fn test_state() {
        let mut s = State::new_from_ks("public/lorerei.ks");
        assert_eq!(s.text, vec!["I go outside with Illya."]);
        s.eval_cmd(Command::Preceed);
        assert_eq!(s.text, vec![
            "I go outside with Illya.",
            "We can’t spare the time to go shopping often, so we’ll have to push ourselves and buy about three days’ worth of groceries.\n"
         ]);
        s.eval_cmd(Command::Preceed);
        assert_eq!(
            s.text,
            vec![
               "“Then let’s buy a lot. What do you want, Illya? Well, we have to start with today’s lunch.”\n"
            ]
         )
    }
}

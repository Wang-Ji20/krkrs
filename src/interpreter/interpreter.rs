use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit};

use crate::{interpreter::parser::*, utils};
use std::fmt::{self, Debug, Formatter};

#[wasm_bindgen]
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

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Command {
    pub kind: CommandKind,
    data: Option<String>,
}

#[wasm_bindgen]
impl Command {
    pub fn new_preceed() -> Command {
        Command {
            kind: CommandKind::Preceed,
            data: None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum CommandKind {
    Preceed,
    Choose,
}

#[wasm_bindgen]
impl State {
    pub fn new_from_ks(filename: &str) -> State {
        utils::set_panic_hook();
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

    pub async fn new_from_web(url: &str) -> State {
        utils::set_panic_hook();
        let mut opts = RequestInit::new();
        opts.method("GET");
        let url = format!("/{}", url);
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let resp_val = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();
        let resp: web_sys::Response = resp_val.dyn_into().unwrap();
        let text = JsFuture::from(resp.text().unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap();
        let mut s = State {
            label: Label {
                label: String::new(),
                heading: String::new(),
            },
            music: String::new(),
            scene: Vec::new(),
            text: Vec::new(),
            cur_token: None,
            tokens: Box::new(parse_ks_string(text.as_str()).unwrap()),
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
        match cmd.kind {
            CommandKind::Preceed => {
                if let Some(Token::Tag(tag)) = &self.cur_token {
                    if tag.name == "pg" {
                        self.scene.clear();
                        self.text.clear();
                    }
                }
                self.eval();
            }
            CommandKind::Choose => {}
        }
    }

    pub fn render(&self) -> String {
        let mut s = String::new();
        for line in &self.text {
            s.push_str(line);
            s.push('\n');
        }
        s
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
        s.eval_cmd(Command {
            kind: CommandKind::Preceed,
            data: None,
        });
        assert_eq!(s.text, vec![
            "I go outside with Illya.",
            "We can’t spare the time to go shopping often, so we’ll have to push ourselves and buy about three days’ worth of groceries.\n"
         ]);
        s.eval_cmd(Command {
            kind: CommandKind::Choose,
            data: None,
        });
        assert_eq!(
            s.text,
            vec![
               "“Then let’s buy a lot. What do you want, Illya? Well, we have to start with today’s lunch.”\n"
            ]
         )
    }
}

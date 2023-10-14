//! Command line interface for the application.
pub use crate::{interface::App, interpreter::interpreter::State, presentation::cli::KrkrsCli};

impl App {
    pub fn new_cli_from_ks(filename: &str) -> App {
        let state = State::new_from_ks(filename);
        App {
            ui: Box::new(KrkrsCli {}),
            state,
        }
    }

    pub async fn new_cli_from_url(url: &str) -> App {
        let state = State::new_from_web(url).await;
        App {
            ui: Box::new(KrkrsCli {}),
            state,
        }
    }

    pub fn run(&mut self) {
        Self::greeting();
        loop {
            self.ui.render(&self.state).unwrap();
            if self.handle_input() {
                break;
            }
        }
    }

    fn handle_input(&mut self) -> bool {
        use std::io::{stdin, stdout, Write};

        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        let input = input.trim();
        match input {
            "q" => true,
            _ => {
                self.state.eval_cmd(input);
                false
            }
        }
    }

    fn greeting() {
        println!("Welcome to krkrs!");
    }
}

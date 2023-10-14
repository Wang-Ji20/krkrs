use crate::interpreter::interpreter::State;

pub mod wasm;

pub mod cli;

pub trait UI {
    fn render(&mut self, state: &State) -> Result<(), String>;
}

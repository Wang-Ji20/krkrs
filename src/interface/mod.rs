use wasm_bindgen::prelude::wasm_bindgen;

use crate::{interpreter::interpreter::State, presentation::UI};

pub mod cli;

pub mod wasm;

#[wasm_bindgen]
pub struct App {
    ui: Box<dyn UI>,
    state: State,
}

pub mod utils;

mod interpreter;
mod parsec;

mod vfs;

mod presentation;

pub mod interface;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, krkrs!");
}

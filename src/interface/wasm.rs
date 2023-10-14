use crate::{presentation::wasm::WebUI, utils};
use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

use super::*;

#[wasm_bindgen]
impl App {
    pub async fn new_web_from_url(url: &str, renderer: Function) -> App {
        utils::set_panic_hook();
        let state = State::new_from_web(url).await;
        let mut app = App {
            ui: Box::new(WebUI::new(renderer)),
            state,
        };
        app.app_init();
        app
    }

    pub fn handle_web_input(&mut self, input: &str) {
        self.state.eval_cmd(input);
        self.ui.render(&self.state).unwrap();
    }

    pub(self) fn app_init(&mut self) {
        self.ui.render(&self.state).unwrap();
    }
}

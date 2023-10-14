use crate::interpreter::interpreter::{RenderContext, State};
use crate::presentation::UI;
use js_sys::global;
use wasm_bindgen::JsValue;

//
// data: rust side game state ----- render() ----> ts side presentation (NO STATE!!!)
//

#[derive(Debug)]
pub struct WebUI {
    render_callback: js_sys::Function,
}

impl From<RenderContext> for JsValue {
    fn from(ctx: RenderContext) -> Self {
        let scene = ctx
            .scene
            .into_iter()
            .map(JsValue::from)
            .collect::<js_sys::Array>();
        let text = ctx
            .text
            .into_iter()
            .map(JsValue::from)
            .collect::<js_sys::Array>();
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"scene".into(), &JsValue::from(scene)).unwrap();
        js_sys::Reflect::set(&obj, &"text".into(), &JsValue::from(text)).unwrap();
        obj.into()
    }
}

impl UI for WebUI {
    fn render(&mut self, state: &State) -> Result<(), String> {
        self.render_callback
            .call1(&global(), &JsValue::from(state.get_render_ctx()))
            .map_err(|e| e.as_string().unwrap_or("".to_string()))
            .and_then(|_| Ok(()))
    }
}

impl WebUI {
    pub fn new(render_callback: js_sys::Function) -> WebUI {
        WebUI { render_callback }
    }
}

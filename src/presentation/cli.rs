use crate::interpreter::interpreter::State;

use super::UI;

#[derive(Debug)]
pub struct KrkrsCli {}

impl UI for KrkrsCli {
    fn render(&mut self, state: &State) -> Result<(), String> {
        println!("game state: {:?}", state.get_render_ctx());
        Ok(())
    }
}

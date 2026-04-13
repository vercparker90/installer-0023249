use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wrap_command(cmd: &str) -> String {
    cmd.to_string()
}

mod utils;

use potterscript_parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_potterscript(code: &str) -> String {
    let ast = potterscript_parser::parse_program(code).unwrap();
    serde_json::to_string(&ast).unwrap()
}

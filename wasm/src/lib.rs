use potterscript_parser;
// TODO uncomment everything when potterscript-runtime crate "no-std" version is available
// so it works in WASM
// use potterscript_runtime;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(code: &str) -> String {
    let (_, ast) = potterscript_parser::parse_program(code).unwrap();
    serde_json::to_string(&ast).unwrap()
}

// #[wasm_bindgen]
// pub fn parse_and_run(code: &str) {
//     let (_, program) = potterscript_parser::parse_program(code).unwrap();
//     let mut runtime = potterscript_runtime::Runtime::new();
//     runtime.eval(program);
// }

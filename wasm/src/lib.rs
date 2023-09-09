use potterscript_parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_potterscript(code: &str) -> String {
    let ast = potterscript_parser::parse_program(code).unwrap();
    serde_json::to_string(&ast).unwrap()
}

// #[wasm_bindgen]
// pub fn parse_and_run(code: &str) {
//     let (_, program) = potterscript_parser::parse_program(code).unwrap();
//     let mut runtime = potterscript_runtime::Runtime::new();
//     runtime.eval(program);
// }

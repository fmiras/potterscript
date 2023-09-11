use potterscript_parser;
use potterscript_runtime::{self, RuntimeAdapter};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

#[wasm_bindgen]
pub fn parse(code: &str) -> String {
    let (_, ast) = potterscript_parser::parse_program(code).unwrap();
    serde_json::to_string(&ast).unwrap()
}

struct WasmRuntimeAdapter;

impl RuntimeAdapter for WasmRuntimeAdapter {
    fn create_random_index(&self) -> usize {
        js_sys::Math::floor(js_sys::Math::random() * 4.0) as usize
    }

    fn lumos(&self, string: String) -> String {
        string
    }

    fn log(&self, string: &str) {
        console::log_1(&string.into());
    }
}

#[wasm_bindgen]
pub fn parse_and_run(code: &str) {
    let (_, program) = potterscript_parser::parse_program(code).unwrap();
    let mut runtime = potterscript_runtime::Runtime::new(WasmRuntimeAdapter);
    runtime.eval(program);
}

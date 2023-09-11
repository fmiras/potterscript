//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use potterscript_wasm;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn parse() {
    let code = r#"index = 0

  quidditch {
    snake = ~Serpensortia
    ~WingardiumLeviosa snake
    ~WingardiumLeviosa snake
    snake = snake + " some string"
    ~Revelio snake
    ~Incendio snake
    ~Revelio snake
    ~Engorgio index
  
    if index == 4 {
      snitch # Break loop
    }
  }
  "#;

    let ast = potterscript_wasm::parse(code);

    assert_eq!(
        ast,
        r#"[{"VariableAssignment":["index",{"Atom":{"Integer":0}}]},{"Quidditch":[{"VariableAssignment":["snake",{"SpellCast":["Serpensortia",null]}]},{"ExpressionStatement":{"SpellCast":["WingardiumLeviosa",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["WingardiumLeviosa",{"Atom":{"Variable":"snake"}}]}},{"VariableAssignment":["snake",{"BinaryOperation":["Plus",{"Atom":{"Variable":"snake"}},{"Atom":{"String":" some string"}}]}]},{"ExpressionStatement":{"SpellCast":["Revelio",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["Incendio",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["Revelio",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["Engorgio",{"Atom":{"Variable":"index"}}]}},{"If":[{"BinaryOperation":["Equal",{"Atom":{"Variable":"index"}},{"Atom":{"Integer":4}}]},["Snitch",{"ExpressionStatement":{"Comment":" Break loop"}}],[]]}]}]"#
    );
}

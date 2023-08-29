use potterscript_parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_potterscript(code: &str) -> String {
    let ast = potterscript_parser::parse_program(code).unwrap();
    serde_json::to_string(&ast).unwrap()
}

#[test]
fn test_parse_potterscript() {
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
    let ast = potterscript_parser::parse_program(code).unwrap();
    let json = serde_json::to_string(&ast).unwrap();

    println!("{}", json);

    assert_eq!(
        json,
        r#"["",[{"VariableAssignment":["index",{"Atom":{"Integer":0}}]},{"Quidditch":[{"VariableAssignment":["snake",{"SpellCast":["Serpensortia",null]}]},{"ExpressionStatement":{"SpellCast":["WingardiumLeviosa",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["WingardiumLeviosa",{"Atom":{"Variable":"snake"}}]}},{"VariableAssignment":["snake",{"BinaryOperation":["Plus",{"Atom":{"Variable":"snake"}},{"Atom":{"String":" some string"}}]}]},{"ExpressionStatement":{"SpellCast":["Revelio",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["Incendio",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["Revelio",{"Atom":{"Variable":"snake"}}]}},{"ExpressionStatement":{"SpellCast":["Engorgio",{"Atom":{"Variable":"index"}}]}},{"If":[{"BinaryOperation":["Equal",{"Atom":{"Variable":"index"}},{"Atom":{"Integer":4}}]},["Snitch",{"ExpressionStatement":{"Comment":" Break loop"}}],[]]}]}]]"#
    );
}

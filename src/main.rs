mod interpreter;
mod parser;

fn main() {
    let input = include_str!("../input.potter");
    let spell_cast = "~Avada Kedabra;";
    let (_, expression) = parser::parse_spell_cast(spell_cast).unwrap();
    interpreter::eval(expression);
}

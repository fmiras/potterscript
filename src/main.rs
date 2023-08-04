mod parser;

fn main() {
    let input = include_str!("../input.potter");
    let spell_cast = "~Revelio -> \"Hello, World!\";";
    dbg!(parser::parse_spell_cast(spell_cast));
}

mod interpreter;
mod parser;

fn main() {
    let input = include_str!("../input.potter");
    let (_, program) = parser::parse_program(input).unwrap();
    interpreter::eval(program);
}

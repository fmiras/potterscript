fn main() {
    let input = include_str!("../input.potter");
    let (_, program) = potterscript_parser::parse_program(input).unwrap();

    println!("PotterScript AST: {:#?}", program);

    let mut interpreter = potterscript_interpreter::Interpreter::new();

    println!("\nRunning PotterScript program...");
    interpreter.eval(program);
    println!("PotterScript program finished running.");
}

use std::io::Write;

fn main() {
    let input = include_str!("../input.potter");
    let (_, program) = potterscript_parser::parse_program(input).unwrap();

    println!("PotterScript AST: {:#?}", program);

    let mut interpreter = potterscript_interpreter::Interpreter::new();

    println!("\nRunning PotterScript program...");
    interpreter.eval(program);
    println!("PotterScript program finished running.");

    println!("PotterScript REPL (Cast `~AvadaKedavra` to exit)");

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        // parse input
        let (_, program) = potterscript_parser::parse_program(&input).unwrap();
        interpreter.eval(program);
    }
}

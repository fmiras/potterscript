use std::io::Write;

fn main() {
    let mut interpreter = potterscript_interpreter::Interpreter::new();
    println!("PotterScript REPL (Cast `~AvadaKedavra` to exit)");

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let (_, program) = potterscript_parser::parse_program(&input).unwrap();
        interpreter.eval(program);
    }
}

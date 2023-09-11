use std::io::Write;

use potterscript_runtime::DefaultRuntimeAdapter;

fn main() {
    let mut runtime = potterscript_runtime::Runtime::new(DefaultRuntimeAdapter);
    println!("PotterScript REPL (Cast `~AvadaKedabra` to exit)");

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let (_, program) = potterscript_parser::parse_program(&input).unwrap();
        runtime.eval(program);
    }
}

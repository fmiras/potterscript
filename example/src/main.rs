use potterscript_runtime::DefaultRuntimeAdapter;

fn main() {
    let input = include_str!("../input.potter");
    let (_, program) = potterscript_parser::parse_program(input).unwrap();

    println!("PotterScript AST: {:#?}", program);

    let mut runtime = potterscript_runtime::Runtime::new(DefaultRuntimeAdapter);

    println!("\nRunning PotterScript program...");
    runtime.eval(program);
    println!("PotterScript program finished running.");
}

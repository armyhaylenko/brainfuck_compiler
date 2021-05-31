
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file = args.get(1).expect("File location not specified");
    let code = std::fs::read_to_string(file).expect("Brainfuck source not found");
    println!("{}", brainfuck_compiler::run(code));
    Ok(())
}

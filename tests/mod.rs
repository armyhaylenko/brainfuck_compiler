#[test]
fn test_normal_code() {
    let code = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    assert_eq!(brainfuck_compiler::run(code), "Hello World!\n")
}

#[test]
fn test_dirty_code() {
    let code = String::from("++++++++[asd>++++[>++>+++>+++>+<<<<-dsa]>+>+>->>+[ads<]<-]>>.>---.+++++++..+++.>adasd>.<-.<.+++.--asd----adasd.--------.>>+.>++.");
    assert_eq!(brainfuck_compiler::run(code), "Hello World!\n")
}

#[test]
fn test_empty_code() {
    let code = String::new();
    let run_result = brainfuck_compiler::run(code);
    println!("{}", &run_result); // immediately returns on empty code
    assert_eq!(&String::new(), &run_result)
}

#[test]
#[should_panic] // panics with unsigned int overflow error
fn test_wrong_code() {
    let code = String::from("++++++++[>++[++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    //                                            ^ unmatched bracket
    println!("{}", brainfuck_compiler::run(code));
}

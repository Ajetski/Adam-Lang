use s1mple::prelude::*;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("invalid path. usage: s1mple <path_to_src>");
    compile_and_execute(path.as_str());
}

fn compile_and_execute(path: &str) -> Option<i32> {
    println!("reading file...");

    let mut code = String::new();
    let code = {
        let file = File::open(path).expect("couldn't find file");
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            code = format!("{}\n{}", code, line.unwrap());
        }
        code.as_str()
    };

    println!("lexing code:");
    println!("{}", code);
    let tokens = lex(code);

    println!("parsing ast...");
    let ast = parse(tokens).unwrap();

    println!("generating executable...");
    let target_name = "out";
    let build_folder = "dist";
    generate(ast, target_name, build_folder).unwrap();

    /*println!("running program");
    let result = Command::new(format!(""))
        .output()
        .unwrap();
    println!("result was success?: {}", result.status.success());
    result.status.code()*/
    todo!("write linker script")
}

#[test]
fn test_add_constants() {
    let path = "examples/add_constants.s1mple";
    assert_eq!(compile_and_execute(path).unwrap(), 42);
}

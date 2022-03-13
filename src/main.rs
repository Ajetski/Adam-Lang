use junkyard::prelude::*;

fn main() {
    let code = r"
fn main ( ) -> i64 {
    1 + 39 + 1 + 1
}
";

    println!("lexing code:");
    println!("{}", code);
    let tokens = lex(code);

    println!("parsing ast...");
    let ast = parse(tokens).unwrap();

    println!("generating executable...");
    let target_name = "out";
    let build_folder = "dist";
    generate(ast, target_name, build_folder).unwrap();

    println!("running program");
    let result = Command::new(format!("./{}/{}.exe", build_folder, target_name))
        .arg(format!("{}/{}.o", build_folder, target_name))
        .arg("-o")
        .arg(format!("{}/{}.exe", build_folder, target_name))
        .output()
        .unwrap();
    println!("result was {}", result.status.code().unwrap());
}

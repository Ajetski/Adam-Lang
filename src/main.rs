use junkyard::prelude::*;

fn main() {
    println!("reading file...");
    
    let mut code = String::new();
    let code = {
        let path = std::env::args().nth(1).expect("invalid path. usage: junkyard <path_to_src>");
        let file = File::open(path).unwrap();
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

    println!("running program");
    let result = Command::new(format!("./{}/{}.exe", build_folder, target_name))
        .arg(format!("{}/{}.o", build_folder, target_name))
        .arg("-o")
        .arg(format!("{}/{}.exe", build_folder, target_name))
        .output()
        .unwrap();
    println!("result was {}", result.status.code().unwrap());
}

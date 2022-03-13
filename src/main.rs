use junkyard::prelude::*;

fn main() {
    let code = r"
        fn main ( ) -> i32 {
            2 + 2
        }
    ";
    dbg!(code.split('\n').collect::<Vec<&str>>());
    let tokens = lex(code);
    dbg!(&tokens);
    let ast = parse(tokens).unwrap();
    dbg!(&ast);
    generate(ast).unwrap();
}

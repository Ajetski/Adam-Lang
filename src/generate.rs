use crate::prelude::*;

pub fn generate(
    ast_function: AstFunction,
    target_name: &str,
    build_folder: &str,
) -> Result<(), Box<dyn Error>> {
    // setup code generator
    std::fs::create_dir_all(format!("./{}", build_folder)).unwrap();

    // generate code
    let llvm = ast_function.generate_llvm();

    let mut file = File::create(format!("./{}/{}.ll", build_folder, target_name)).unwrap();
    for line in llvm {
        file.write_all((line + "\n").as_bytes()).unwrap();
    }

    // link results
    Command::new("llc")
        .arg("-filetype=obj")
        .arg(format!("./{}/{}.ll", build_folder, target_name))
        .arg("-o")
        .arg(format!("./{}/{}.o", build_folder, target_name))
        .output()
        .unwrap();

    Command::new("clang")
        .arg(format!("./{}/{}.o", build_folder, target_name))
        .arg("-o")
        .arg(format!("./{}/{}", build_folder, target_name))
        .output()
        .unwrap();
    Ok(())
}

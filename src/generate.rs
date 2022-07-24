extern crate cranelift;
extern crate cranelift_native;

use crate::prelude::*;

pub fn generate(
    ast_function: AstFunction,
    target_name: &str,
    build_folder: &str,
) -> Result<(), Box<dyn Error>> {
    // setup code generator
    let isa_builder = cranelift_native::builder_with_options(true).unwrap();
    let flags = settings::Flags::new(settings::builder());
    let isa = isa_builder.finish(flags.clone()).unwrap();
    let object_builder =
        cranelift_object::ObjectBuilder::new(isa, target_name, default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(object_builder);
    std::fs::create_dir_all(format!("./{}", build_folder)).unwrap();

    // generate code
    ast_function
        .codegen(&mut module, &flags, build_folder)
        .unwrap();

    let mut file = File::create(format!("{}/{}.o", build_folder, target_name)).unwrap();
    file.write_all(&module.finish().emit()?)?;

    // link results
    Command::new("ld")
        .arg(format!("{}/{}.o", build_folder, target_name))
        .arg("-o")
        .arg(format!("{}/{}.exe", build_folder, target_name))
        .output()?;
    Ok(())
}

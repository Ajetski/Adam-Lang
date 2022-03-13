extern crate cranelift;
extern crate cranelift_native;

use crate::prelude::*;

pub fn generate(ast_function: AstFunction) -> Result<(), Box<dyn Error>> {
    // setup code generator
    let isa_builder = cranelift_native::builder()?;
    let flags = settings::Flags::new(settings::builder());
    let isa = isa_builder.finish(flags.clone())?;
    let object_name = "main";
    let build_folder = "dist";
    let object_builder = cranelift_object::ObjectBuilder::new(isa, object_name, default_libcall_names())?;
    let mut module = ObjectModule::new(object_builder);

    // generate code
    ast_function.codegen(&mut module, &flags)?;

    // print results
    let res = module.finish();
    dbg!(&res.object);
    let mut file = File::create(format!("{}/{}.o", build_folder, object_name))?;
    file.write_all(&res.emit()?)?;

    // link results
    let output = Command::new("ld")
        .arg(format!("{}/{}.o", build_folder, object_name))
        .arg("-o")
        .arg(format!("{}/{}.exe", build_folder, object_name))
        .output()?;
    dbg!(&output.stdout);
    Ok(())
}

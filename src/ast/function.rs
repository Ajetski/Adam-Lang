use crate::prelude::*;
#[derive(Debug)]
#[allow(dead_code)]
pub struct AstFunctionBody {
    pub(crate) expression: AstExpression,
}
impl AstFunctionBody {
    fn codegen(&self, builder: &mut FunctionBuilder) -> entities::Value {
        self.expression.codegen(builder)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstFunction {
    pub(crate) name: Option<String>,
    pub(crate) function_return: Option<AstIdent>,
    pub(crate) function_body: AstFunctionBody,
}
impl AstFunction {
    pub(crate) fn codegen(
        &self,
        module: &mut ObjectModule,
        flags: &Flags,
        build_folder: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut main_func_sig = Signature::new(CallConv::SystemV);
        if let Some(ast_ident) = &self.function_return {
            if ast_ident.ident == "i64" {
                main_func_sig
                    .returns
                    .push(AbiParam::new(codegen::ir::types::I64));
            }
        }
        let mut fn_builder_ctx = FunctionBuilderContext::new();
        let mut main_func =
            Function::with_name_signature(ExternalName::user(0, 0), main_func_sig.clone());
        {
            let mut builder = FunctionBuilder::new(&mut main_func, &mut fn_builder_ctx);

            let block = builder.create_block();
            builder.switch_to_block(block);
            builder.seal_block(block);

            let arg0 = self.function_body.codegen(&mut builder);
            builder.ins().return_(&[arg0]);

            builder.finalize();
        }

        verify_function(&main_func, flags)?;
        let mut context = Context::for_function(main_func.clone());
        let name = match self.name.clone() {
            Some(name) => name,
            None => String::from("temp"),
        };
        let main_func_id =
            module.declare_function(name.as_str(), Linkage::Local, &main_func_sig)?;
        module.define_function(main_func_id, &mut context)?;

        let mut file = File::create(format!("{}/{}.clif", build_folder, &name))?;
        file.write_all(main_func.display().to_string().as_bytes())?;

        Ok(())
    }
}

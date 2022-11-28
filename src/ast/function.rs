use crate::prelude::*;
#[derive(Debug)]
#[allow(dead_code)]
pub struct AstFunctionBody {
    pub(crate) expression: AstExpression,
}
impl AstNode for AstFunctionBody {
    fn generate_llvm(&self) -> Vec<String> {
        self.expression.generate_llvm()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstFunction {
    pub(crate) name: Option<String>,
    pub(crate) function_return: Option<AstIdent>,
    pub(crate) function_body: AstFunctionBody,
}
impl AstNode for AstFunction {
    fn generate_llvm(&self) -> Vec<String> {
        //TODO: replace with real function implementation
        let mut output = vec!["define i32 @main() {".to_owned()];
        output.push("\tret i32 0".to_owned());
        output.push("}".to_owned());
        output
    }
}

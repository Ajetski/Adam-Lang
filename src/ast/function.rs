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
        self.function_body.generate_llvm()
    }
}

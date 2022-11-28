use crate::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstIdent {
    pub(crate) ident: String,
}
impl AstIdent {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Value {
    Ident(AstIdent),
    Literal(i64),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstValue {
    pub(crate) value: Value,
}
impl AstNode for AstValue {
    fn generate_llvm(&self) -> Vec<String> {
        match self.value {
            Value::Literal(val) => todo!(),
            _ => todo!(),
        }
    }
}

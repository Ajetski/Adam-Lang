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
impl AstValue {
    pub fn codegen(&self, builder: &mut FunctionBuilder) -> entities::Value {
        match self.value {
            Value::Literal(val) => builder.ins().iconst(types::I64, Imm64::from(val)),
            _ => todo!(),
        }
    }
}

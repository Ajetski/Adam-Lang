#[derive(Debug)]
#[allow(dead_code)]
pub struct AstIdent {
    pub(crate) ident: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Value {
    Ident(AstIdent),
    Literal(i32),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstValue {
    pub(crate) value: Value,
}

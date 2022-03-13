use crate::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstModule {
    name: Option<String>,
}

impl AstNode for AstModule {}

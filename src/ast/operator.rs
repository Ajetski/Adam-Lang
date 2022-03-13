use crate::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstOperator {
    pub(crate) operator: Operator,
}

impl AstNode for AstOperator {}

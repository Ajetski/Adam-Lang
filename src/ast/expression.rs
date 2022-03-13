use crate::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstExpression {
    pub(crate) value: AstValue,
    pub(crate) operator: Option<AstOperator>,
    pub(crate) right_expression: Option<Box<AstExpression>>,
}

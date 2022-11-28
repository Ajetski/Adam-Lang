use super::AstNode;
use crate::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstExpression {
    pub(crate) value: AstValue,
    pub(crate) operator: Option<AstOperator>,
    pub(crate) right_expression: Option<Box<AstExpression>>,
}
impl AstNode for AstExpression {
    fn generate_llvm(&self) -> Vec<String> {
        match &self.operator {
            Some(operator) => {
                let expression = self.right_expression.as_ref().unwrap();
                match operator.operator {
                    Operator::Add => todo!(),
                }
            }
            None => self.value.generate_llvm(),
        }
    }
}

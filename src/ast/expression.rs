use crate::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AstExpression {
    pub(crate) value: AstValue,
    pub(crate) operator: Option<AstOperator>,
    pub(crate) right_expression: Option<Box<AstExpression>>,
}
impl AstExpression {
    pub fn codegen(&self, builder: &mut FunctionBuilder) -> entities::Value {
        let value = self.value.codegen(builder);
        match &self.operator {
            Some(operator) => {
                let expression = self.right_expression.as_ref().unwrap();
                let rhs = expression.codegen(builder);
                match operator.operator {
                    Operator::Add => builder.ins().iadd(value, rhs),
                }
            }
            None => value,
        }
    }
}

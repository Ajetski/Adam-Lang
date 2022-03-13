use crate::prelude::*;


struct Parser {
    tokens: Vec<Token>,
    position: usize,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn parse_function(&mut self) -> Result<AstFunction, ()> {
        let init_position = self.position;
        match self.tokens[self.position] {
            Token::Keyword(Keyword::Fn) => {
                self.position += 1;
            }
            _ => return Err(()),
        }
        let name = match &self.tokens[self.position] {
            Ident(name) => {
                self.position += 1;
                Some(name.clone())
            },
            _ => None,
        };
        match self.tokens[self.position] {
            Punctuation(Punctuation::LeftParen) => {
                self.position += 1;
            }
            _ => return Err(()),
        }
        match self.tokens[self.position] {
            Punctuation(Punctuation::RightParen) => {
                self.position += 1;
            }
            _ => {
                self.position = init_position;
                return Err(());
            }
        }
        let function_return = match self.tokens[self.position] {
            Punctuation(Punctuation::ReturnArrow) => {
                self.position += 1;
                match &self.tokens[self.position] {
                    Ident(ident) => {
                        self.position += 1;
                        Some(AstIdent { ident: ident.clone() })
                    },
                    _ => None,
                }
            } 
            _ => None,
        };
        let function_body = match self.tokens[self.position] {
            Punctuation(Punctuation::LeftBrace) => {
                self.position += 1;
                let expression = self.parse_expression()?;
                match self.tokens[self.position] {
                    Punctuation(Punctuation::RightBrace) => {
                        self.position += 1;
                        AstFunctionBody {
                            expression,
                        }
                    }
                    _ => {
                        self.position = init_position;
                        return Err(());
                    }
                }
            }
            _ => {
                self.position = init_position;
                return Err(());
            }
        };

        Ok(AstFunction {
            name,
            function_return,
            function_body,
        })
    }

    fn parse_value(&mut self) -> Result<AstValue, ()> {
        match &self.tokens[self.position] {
            Literal(I32(val)) => {
                self.position += 1;
                Ok(AstValue {
                    value: Value::Literal(val.clone()),
                })
            }
            Ident(ident) => {
                self.position += 1;
                Ok(AstValue {
                    value: Value::Ident(AstIdent {
                        ident: ident.clone(),
                    }),
                })
            }
            _ => Err(()),
        }
    }

    fn parse_expression(&mut self) -> Result<AstExpression, ()> {
        let value = self.parse_value()?;
        match self.parse_operator() {
            Ok(operator) => {
                let right_expression = self.parse_expression()?;

                Ok(AstExpression {
                    value,
                    operator: Some(operator),
                    right_expression: Some(Box::new(right_expression)),
                })
            }
            _ => Ok(AstExpression {
                value,
                operator: None,
                right_expression: None,
            }),
        }
    }

    fn parse_operator(&mut self) -> Result<AstOperator, ()> {
        match self.tokens[self.position] {
            Operator(operator) => {
                self.position += 1;
                Ok(AstOperator { operator })
            }
            _ => Err(()),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Box<dyn AstNode>, ()> {
    let mut parser = Parser::new(tokens);
    return Ok(Box::new(parser.parse_function()?));
}

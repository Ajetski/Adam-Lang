pub mod ast;
pub mod generate;
pub mod lexer;
pub mod parser;

pub mod prelude {
    pub use crate::{
        ast::{
            expression::*, function::*, module::*, operator::*, value::AstIdent, value::AstValue,
            value::Value, AstNode,
        },
        generate::*,
        lexer::{Token::*, Value::I64, *},
        parser::*,
    };
    pub use std::error::Error;
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use std::process::Command;
}

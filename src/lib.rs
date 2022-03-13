pub mod lexer;
pub mod generate;
pub mod parser;
pub mod ast;

pub mod prelude {
	pub use crate::{
		lexer::{*, Token::*, Value::*},
		generate::*,
		ast::{
			node::*,
			module::*,
			operator::*,
			value::Value,
			value::AstValue,
			value::AstIdent,
			function::*,
			expression::*,
		},
	};
	pub use std::error::Error;
	pub use crate::parser::*;
}

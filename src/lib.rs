pub mod lexer;
pub mod generate;
pub mod parser;
pub mod ast;

pub mod prelude {
	pub use crate::{
		lexer::{*, Token::*, Value::I32},
		generate::*,
		ast::{
			module::*,
			operator::*,
			value::Value,
			value::AstValue,
			value::AstIdent,
			function::*,
			expression::*,
		},
		parser::*,
	};
	pub use std::error::Error;
	pub use codegen::{ir::Function, verify_function};
	pub use cranelift::codegen::Context;
	pub use cranelift::prelude::{isa::*, types::*, *};
	pub use cranelift_module::{default_libcall_names, Module, Linkage};
	pub use cranelift_object::ObjectModule;
	pub use std::fs::File;
	pub use std::io::prelude::*;
	pub use std::process::Command;
}

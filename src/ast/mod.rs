pub mod expression;
pub mod function;
pub mod module;
pub mod operator;
pub mod value;

pub trait AstNode {
    fn generate_llvm(&self) -> Vec<String>;
}

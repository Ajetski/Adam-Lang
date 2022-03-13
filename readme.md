## Junkyard

A toy language/compiler for learning compiler design.

### Grammar

| Symbol         |                        Symbol Defisnition                         |
| -------------- | :---------------------------------------------------------------: |
| Function       | fn \<Ident>? \<FunctionParams> \<FunctionReturn>? \<FunctionBody> |
| FunctionParams |                                ( )                                |
| FunctionReturn |                              -> i64                               |
| FunctionBody   |                        { \<Expression\> }                         |
| Expression     |               \<Value> \<Operator>? \<Expression>?                |
| Value          |                      \<Ident> \| \<Literal>                       |
| Operator       |                                 +                                 |
| Ident          |                             [A-Za-z]*                             |
| Literal        |                              [0-9]*                               |

### Todo 

* Expand grammar to add modules that can contain mulitple functions
* Implement symbol table
* Expand grammar to include capturing arguments for a function
* Add more operators and operator precedence

### Tech Stack

* [Rust](https://www.rust-lang.org/)
* [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)

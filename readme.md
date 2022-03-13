## s1mple

A toy language/compiler for learning compiler design.

[![CI](https://github.com/Ajetski/s1mple/actions/workflows/ci.yml/badge.svg)](https://github.com/Ajetski/s1mple/actions/workflows/ci.yml)

### Screenshot

![image](https://user-images.githubusercontent.com/45019515/158074087-26832364-49ab-4844-bebd-edc8bc7d4240.png)

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

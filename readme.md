## Adam Lang

A toy language/compiler for learning compiler design.

[![CI](https://github.com/Ajetski/s1mple/actions/workflows/ci.yml/badge.svg)](https://github.com/Ajetski/s1mple/actions/workflows/ci.yml)

### Screenshot

![image](https://user-images.githubusercontent.com/45019515/158074087-26832364-49ab-4844-bebd-edc8bc7d4240.png)

### Grammar

| Symbol         |                        Symbol Definition                         |
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

* Implement symbol table
* Expand grammar to include capturing arguments for a function
* Add more operators and operator precedence
* Expand grammar to add module system

### Technologies Used

* [Rust](https://www.rust-lang.org/)
* [LLVM](https://www.rust-lang.org/)

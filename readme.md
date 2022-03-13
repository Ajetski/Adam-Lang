## Junkyard

A toy language/compiler for learning compiler design.

### Screenshot

![image](https://user-images.githubusercontent.com/45019515/158071356-45229864-c7b8-4869-a322-e7f62e4d9959.png)

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

* Rust
* Cranelift

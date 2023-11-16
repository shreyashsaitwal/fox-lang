## 🦊 Fox lang

Just me tryna make a programming language.

### 🚧 Grammer

```
expression  -> literal
            | unary
            | binary
            | grouping ;
literal     -> NUMBER | STRING | "true" | "false" | "nil" ;
grouping    -> "(" expression ")" ;
unary       -> ( "-" | "!" ) expression ;
binary      -> expression operator expression ;
operator    -> "==" | "!=" | "<" | "<=" | ">" | ">="
            | "+"  | "-"  | "*" | "/" ;
```

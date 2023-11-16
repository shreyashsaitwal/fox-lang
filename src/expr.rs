use std::fmt::Display;

use crate::lexer::Token;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(Literal),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub operator: Token,
    pub rhs: Box<Expr>,
}

pub struct GroupingExpr {
    pub expr: Box<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub rhs: Box<Expr>,
}

pub enum Literal {
    String(Option<String>),
    Number(Option<f64>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        match self {
            Expr::Binary(expr) => {
                string.push('(');
                string.push_str(&expr.operator.lexeme());
                string.push(' ');
                string.push_str(&expr.lhs.to_string());
                string.push(' ');
                string.push_str(&expr.rhs.to_string());
                string.push(')');
            }
            Expr::Grouping(expr) => {
                string.push('(');
                string.push_str("group ");
                string.push_str(&expr.expr.to_string());
                string.push(')');
            }
            Expr::Literal(expr) => {
                let str = match expr {
                    Literal::String(val) if let Some(val) = val => val.to_string(),
                    Literal::Number(val) if let Some(val) = val => val.to_string(),
                    _ => "nil".to_string(),
                };
                string.push_str(&str);
            }
            Expr::Unary(expr) => {
                string.push('(');
                string.push_str(&expr.operator.lexeme());
                string.push(' ');
                string.push_str(&expr.rhs.to_string());
                string.push(')');
            }
        }
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod test {
    use crate::{
        expr::{GroupingExpr, Literal, UnaryExpr},
        lexer::{Position, Token, TokenType},
    };

    use super::{BinaryExpr, Expr};

    #[test]
    fn check_printing() {
        let simple = Expr::Binary(BinaryExpr {
            lhs: Box::new(Expr::Literal(Literal::Number(Some(1.0)))),
            operator: Token {
                ty: TokenType::Plus,
                position: Position {
                    start: 0,
                    end: 0,
                    line: 0,
                },
            },
            rhs: Box::new(Expr::Literal(Literal::Number(Some(2.0)))),
        });
        let complex = Expr::Binary(BinaryExpr {
            lhs: Box::new(Expr::Literal(Literal::Number(Some(1.0)))),
            operator: Token {
                ty: TokenType::Plus,
                position: Position {
                    start: 0,
                    end: 0,
                    line: 0,
                },
            },
            rhs: Box::new(Expr::Binary(BinaryExpr {
                lhs: Box::new(Expr::Literal(Literal::Number(Some(2.0)))),
                operator: Token {
                    ty: TokenType::Plus,
                    position: Position {
                        start: 0,
                        end: 0,
                        line: 0,
                    },
                },
                rhs: Box::new(Expr::Literal(Literal::Number(Some(3.0)))),
            })),
        });
        let complex_with_every_type_of_expr = Expr::Binary(BinaryExpr {
            lhs: Box::new(Expr::Literal(Literal::Number(Some(1.0)))),
            operator: Token {
                ty: TokenType::Plus,
                position: Position {
                    start: 0,
                    end: 0,
                    line: 0,
                },
            },
            rhs: Box::new(Expr::Binary(BinaryExpr {
                lhs: Box::new(Expr::Literal(Literal::Number(Some(2.0)))),
                operator: Token {
                    ty: TokenType::Plus,
                    position: Position {
                        start: 0,
                        end: 0,
                        line: 0,
                    },
                },
                rhs: Box::new(Expr::Grouping(GroupingExpr {
                    expr: Box::new(Expr::Unary(UnaryExpr {
                        operator: Token {
                            ty: TokenType::Minus,
                            position: Position {
                                start: 0,
                                end: 0,
                                line: 0,
                            },
                        },
                        rhs: Box::new(Expr::Literal(Literal::Number(Some(3.0)))),
                    })),
                })),
            })),
        });
        assert_eq!(simple.to_string(), "(+ 1 2)");
        assert_eq!(complex.to_string(), "(+ 1 (+ 2 3))");
        assert_eq!(
            complex_with_every_type_of_expr.to_string(),
            "(+ 1 (+ 2 (group (- 3))))"
        );
    }
}

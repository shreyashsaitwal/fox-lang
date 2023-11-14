use itertools::{Itertools, MultiPeek};
use miette::NamedSource;
use std::str::{Chars, FromStr};

use crate::errors::SyntaxError;

#[derive(Debug)]
pub struct Token<'a> {
    pub ty: TokenType<'a>,
    pub position: Position,
}

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum TokenType<'a> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,

    Bang,
    BangEq,
    Equal,
    EqualEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    Identifier(&'a str),
    String(&'a str),
    Number(f64),

    Keyword(Keyword),
    Comment,
    Eof,
}

#[derive(Debug)]
pub enum Keyword {
    Let,
    Fn,
    Return,
    Class,
    Super,
    This,
    And,
    Or,
    If,
    Else,
    True,
    False,
    For,
    While,
    Nil,
    Print,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "let" => Ok(Keyword::Let),
            "fn" => Ok(Keyword::Fn),
            "return" => Ok(Keyword::Return),
            "class" => Ok(Keyword::Class),
            "super" => Ok(Keyword::Super),
            "this" => Ok(Keyword::This),
            "and" => Ok(Keyword::And),
            "or" => Ok(Keyword::Or),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "for" => Ok(Keyword::For),
            "while" => Ok(Keyword::While),
            "nil" => Ok(Keyword::Nil),
            "print" => Ok(Keyword::Print),
            _ => Err(()),
        }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    iter: MultiPeek<Chars<'a>>,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            iter: source.chars().multipeek(),
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Option<Result<Token<'a>, SyntaxError>> {
        self.advance_while(|c| c.is_whitespace());
        let start = self.current;
        let ch = self.advance();
        ch.map(|ch| {
            let ty = match ch {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ',' => TokenType::Comma,
                ';' => TokenType::Semicolon,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                '*' => TokenType::Star,
                '/' => {
                    if let Some('/') = self.iter.peek() {
                        self.advance_while(|c| c != &'\n');
                        if self.iter.peek().is_some() {
                            self.advance();
                        }
                        TokenType::Comment
                    } else {
                        TokenType::Slash
                    }
                }
                '!' => {
                    if let Some('=') = self.iter.peek() {
                        self.advance();
                        TokenType::BangEq
                    } else {
                        TokenType::Bang
                    }
                }
                '=' => {
                    if let Some('=') = self.iter.peek() {
                        self.advance();
                        TokenType::EqualEq
                    } else {
                        TokenType::Equal
                    }
                }
                '>' => {
                    if let Some('=') = self.iter.peek() {
                        self.advance();
                        TokenType::GreaterEq
                    } else {
                        TokenType::Greater
                    }
                }
                '<' => {
                    if let Some('=') = self.iter.peek() {
                        self.advance();
                        TokenType::LessEq
                    } else {
                        TokenType::Less
                    }
                }
                '"' => match self.string(start) {
                    Ok(ty) => ty,
                    Err(err) => return Err(err),
                },
                ch if ch.is_numeric() => self.number(start),
                ch if ch.is_alphabetic() => self.identifier(start),
                ch => {
                    return Err(SyntaxError::UnexpectedCharacter {
                        src: NamedSource::new("", self.source.to_string()),
                        span: (start, 1).into(),
                        char: ch,
                    })
                }
            };

            self.iter.reset_peek();
            let position = Position {
                start,
                end: self.current,
                line: self.line,
            };
            Ok(Token { ty, position })
        })
    }

    fn advance(&mut self) -> Option<char> {
        self.iter.next().map(|ch| {
            self.current += 1;
            if '\n' == ch {
                self.line += 1;
            }
            ch
        })
    }

    fn advance_while<F>(&mut self, predicate: F) -> usize
    where
        F: Fn(&char) -> bool,
    {
        let mut count = 0usize;
        while let Some(ch) = self.iter.peek() {
            if !predicate(ch) {
                break;
            }
            count += 1;
            self.advance();
        }
        self.iter.reset_peek();
        count
    }

    fn string(&mut self, start: usize) -> Result<TokenType<'a>, SyntaxError> {
        let len = self.advance_while(|c| c != &'"');
        if self.advance().is_none() {
            return Err(SyntaxError::UnterminatedString {
                src: NamedSource::new("", self.source.to_string()),
                quote: (start, 1).into(),
            });
        }
        let start = start + 1;
        let end = start + len;
        Ok(TokenType::String(&self.source[start..end]))
    }

    fn number(&mut self, start: usize) -> TokenType<'a> {
        let mut len = self.advance_while(|ch| ch.is_numeric());
        if let Some(&'.') = self.iter.peek() {
            let is_frac = self.iter.peek().map_or(false, |ch| ch.is_numeric());
            if is_frac {
                self.advance();
                len += 1;
                len += self.advance_while(|c| c.is_numeric());
            }
        }
        self.iter.reset_peek();
        let end = start + len;
        let literal = &self.source[start..=end];
        TokenType::Number(literal.parse::<f64>().unwrap())
    }

    fn identifier(&mut self, start: usize) -> TokenType<'a> {
        let len = self.advance_while(|c| c.is_alphanumeric() || c == &'_');
        let end = start + len;
        let literal = &self.source[start..=end];
        if let Ok(kw) = Keyword::from_str(literal) {
            TokenType::Keyword(kw)
        } else {
            TokenType::Identifier(literal)
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, SyntaxError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.scan_token() {
            match item {
                Ok(t) => Some(Ok(t)),
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }
}

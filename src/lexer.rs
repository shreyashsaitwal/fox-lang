use itertools::{Itertools, MultiPeek};
use miette::NamedSource;
use std::{
    fmt,
    str::{Chars, FromStr},
};

use crate::errors::SyntaxError;

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub position: Position,
}

impl Token {
    pub fn lexeme(&self) -> String {
        match &self.ty {
            TokenType::LeftParen => "(".to_string(),
            TokenType::RightParen => ")".to_string(),
            TokenType::LeftBrace => "{".to_string(),
            TokenType::RightBrace => "}".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Slash => "/".to_string(),
            TokenType::Star => "*".to_string(),
            TokenType::Bang => "!".to_string(),
            TokenType::BangEq => "!=".to_string(),
            TokenType::Equal => "=".to_string(),
            TokenType::EqualEq => "==".to_string(),
            TokenType::Greater => ">".to_string(),
            TokenType::GreaterEq => ">=".to_string(),
            TokenType::Less => "<".to_string(),
            TokenType::LessEq => "<=".to_string(),
            TokenType::Identifier(ident) => ident.to_string(),
            TokenType::String(lit) => lit.to_string(),
            TokenType::Number(num) => num.to_string(),
            TokenType::Keyword(kw) => kw.lexeme().to_owned(),
            TokenType::Comment => "<comment>".to_string(),
            TokenType::Eof => "<eof>".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
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

    Identifier(String),
    String(String),
    Number(f64),

    Keyword(Keyword),
    Comment,
    Eof,
}

#[derive(Debug, PartialEq)]
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

impl Keyword {
    fn lexeme(&self) -> &str {
        match self {
            Keyword::Let => "let",
            Keyword::Fn => "fn",
            Keyword::Return => "return",
            Keyword::Class => "class",
            Keyword::Super => "super",
            Keyword::This => "this",
            Keyword::And => "and",
            Keyword::Or => "or",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::For => "for",
            Keyword::While => "while",
            Keyword::Nil => "nil",
            Keyword::Print => "print",
        }
    }
}

impl FromStr for Keyword {
    type Err = fmt::Error;

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
            _ => Err(fmt::Error),
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    iter: MultiPeek<Chars<'a>>,
    current: usize,
    line: usize,
    at_eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            iter: source.chars().multipeek(),
            current: 0,
            line: 1,
            at_eof: false,
        }
    }

    pub fn scan_token(&mut self) -> Option<Result<Token, SyntaxError>> {
        self.advance_while(|ch| ch.is_whitespace());
        let start = self.current;
        let ch = self.advance();
        let token = ch.map(|ch| {
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
                    let next = self.iter.peek();
                    if let Some('/') = next {
                        self.advance_while(|ch| ch != &'\n');
                        if self.iter.peek().is_some() {
                            self.advance();
                        }
                        TokenType::Comment
                    } else if let Some('*') = next {
                        self.advance();
                        match self.block_comment(start) {
                            Ok(ty) => ty,
                            Err(err) => return Err(err),
                        }
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
        });

        match token {
            Some(t) => Some(t),
            None if !self.at_eof => {
                self.at_eof = true;
                Some(Ok(Token {
                    ty: TokenType::Eof,
                    position: Position {
                        line: self.line,
                        start: self.current,
                        end: self.current,
                    },
                }))
            }
            None => None,
        }
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

    fn string(&mut self, start: usize) -> Result<TokenType, SyntaxError> {
        let len = self.advance_while(|ch| ch != &'"');
        if self.advance().is_none() {
            return Err(SyntaxError::UnterminatedString {
                src: NamedSource::new("", self.source.to_string()),
                leading_quote: (start, 1).into(),
            });
        }
        let start = start + 1;
        let end = start + len;
        Ok(TokenType::String(self.source[start..end].to_string()))
    }

    fn number(&mut self, start: usize) -> TokenType {
        let mut len = self.advance_while(|ch| ch.is_numeric());
        if let Some(&'.') = self.iter.peek() {
            let is_frac = self.iter.peek().map_or(false, |ch| ch.is_numeric());
            if is_frac {
                self.advance();
                len += 1;
                len += self.advance_while(|ch| ch.is_numeric());
            }
        }
        self.iter.reset_peek();
        let end = start + len;
        let literal = &self.source[start..=end];
        TokenType::Number(literal.parse::<f64>().unwrap())
    }

    fn identifier(&mut self, start: usize) -> TokenType {
        let len = self.advance_while(|ch| ch.is_alphanumeric() || ch == &'_');
        let end = start + len;
        let literal = &self.source[start..=end];
        if let Ok(kw) = Keyword::from_str(literal) {
            TokenType::Keyword(kw)
        } else {
            TokenType::Identifier(literal.to_string())
        }
    }

    fn block_comment(&mut self, start: usize) -> Result<TokenType, SyntaxError> {
        let mut count = 1;
        while count > 0 && self.iter.peek().is_some() {
            self.iter.reset_peek();
            let curr = self.iter.peek();
            if let Some('/') = curr {
                if let Some('*') = self.iter.peek() {
                    count += 1;
                    self.advance();
                }
            } else if let Some('*') = curr {
                if let Some('/') = self.iter.peek() {
                    count -= 1;
                    self.advance();
                }
            }
            self.advance();
        }
        if count > 0 {
            Err(SyntaxError::UnterminatedBlockComment {
                src: NamedSource::new("", self.source.to_string()),
                comment_start: (start, 2).into(),
            })
        } else {
            Ok(TokenType::Comment)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, SyntaxError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.scan_token() {
            match item {
                Ok(t) if let TokenType::Comment = t.ty => {}
                Ok(t) => return Some(Ok(t)),
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }
}

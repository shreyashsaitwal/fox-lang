use itertools::{Itertools, MultiPeek};
use std::str::{Chars, FromStr};

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

    pub fn scan_token(&mut self) -> Option<Token<'a>> {
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
                '"' => self.string(start),
                ch if ch.is_numeric() => self.number(start),
                ch if ch.is_alphabetic() => self.identifier(start),
                _ => todo!(),
            };

            self.iter.reset_peek();
            let position = Position {
                line: self.line,
                start,
                end: self.current,
            };
            Token { ty, position }
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

    fn string(&mut self, start: usize) -> TokenType<'a> {
        let literal_size = self.advance_while(|c| c != &'"');
        if self.advance().is_none() {
            eprintln!("Unterminated string")
        }
        let start = start + 1;
        let end = start + literal_size;
        TokenType::String(&self.source[start..end])
    }

    fn number(&mut self, start: usize) -> TokenType<'a> {
        let mut literal_size = self.advance_while(|ch| ch.is_numeric());
        if let Some(&'.') = self.iter.peek() {
            let is_frac = self.iter.peek().map_or(false, |ch| ch.is_numeric());
            if is_frac {
                self.advance();
                literal_size += 1;
                literal_size += self.advance_while(|c| c.is_numeric());
            }
        }
        self.iter.reset_peek();
        let end = start + literal_size;
        let literal = &self.source[start..=end];
        TokenType::Number(literal.parse::<f64>().unwrap())
    }

    fn identifier(&mut self, start: usize) -> TokenType<'a> {
        let literal_size = self.advance_while(|c| c.is_alphanumeric());
        let end = start + literal_size;
        let literal = &self.source[start..=end];
        if let Ok(kw) = Keyword::from_str(literal) {
            TokenType::Keyword(kw)
        } else {
            TokenType::Identifier(literal)
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

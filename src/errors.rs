use std::fmt::Debug;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum SyntaxError {
    #[error("Syntax error: Unexpected character `{char}` found")]
    #[diagnostic()]
    UnexpectedCharacter {
        #[source_code]
        src: NamedSource,
        #[label(primary, "this one right here")]
        span: SourceSpan,
        char: char,
    },

    #[error("Syntax error: Missing trailing `\"` to terminate the string")]
    #[diagnostic(help("consider adding a `\"` after the string literal"))]
    UnterminatedString {
        #[source_code]
        src: NamedSource,
        #[label(primary, "opening `\"` found here")]
        leading_quote: SourceSpan,
    },

    #[error("Unterminated block comment: Missing trailing `*/` to terminate the block comment")]
    #[diagnostic(help("consider adding `*/` at the end of the block comment"))]
    UnterminatedBlockComment {
        #[source_code]
        src: NamedSource,
        #[label(primary, "start of the block comment")]
        comment_start: SourceSpan
    }
}

use std::fmt::Debug;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum SyntaxError {
    #[error("Unexpected character: {char}")]
    #[diagnostic()]
    UnexpectedCharacter {
        #[source_code]
        src: NamedSource,
        #[label(primary, "this one right here")]
        span: SourceSpan,
        char: char,
    },

    #[error("Unterminated string: closing \" not found")]
    #[diagnostic(help("consider adding a closing \" after the string literal"))]
    UnterminatedString {
        #[source_code]
        src: NamedSource,
        #[label(primary, "opening \" found here")]
        quote: SourceSpan,
    },
}

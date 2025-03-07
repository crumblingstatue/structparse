//! A library for parsing (a simplified form of) structs
#![warn(missing_docs, clippy::pedantic, clippy::unwrap_used)]

use {
    std::num::ParseIntError,
    thiserror::Error,
    tokenize::{TokenKind, TokenizeErrorKind, tokenize},
};

mod parse;
mod tokenize;

/// A parsed struct
#[derive(Debug, PartialEq)]
pub struct Struct<'s> {
    /// The name of the struct
    pub name: &'s str,
    /// The fields of the struct
    pub fields: Vec<Field<'s>>,
}

impl<'s> Struct<'s> {
    /// Parse a struct definition from a string
    ///
    /// # Errors
    ///
    /// Returns an error if the text failed to parse as a struct.
    pub fn parse(input: &'s str) -> Result<Self, StructParseError> {
        match tokenize(input) {
            Ok(tokens) => parse::parse_struct(input, &tokens),
            Err(e) => Err(StructParseError {
                span: e.span,
                kind: StructParseErrorKind::Tokenize(e.kind),
            }),
        }
    }
}

/// Error that can happen while parsing a struct
#[derive(Debug, Error)]
#[error("Parse error at {span:?}: {kind}")]
pub struct StructParseError {
    span: std::ops::Range<usize>,
    kind: StructParseErrorKind,
}
impl StructParseError {
    fn unexpected(tok: tokenize::Token) -> Self {
        Self {
            span: tok.span,
            kind: StructParseErrorKind::UnexpectedTok(tok.kind),
        }
    }
}

/// Kind of error that can happen while parsing a struct
#[derive(Debug, Error)]
pub enum StructParseErrorKind {
    /// Tokenize error
    #[error("Tokenize error: {0:?}")]
    Tokenize(TokenizeErrorKind),
    /// Unexpected end of token stream
    #[error("Unexpected end")]
    UnexpectedEnd,
    /// Unexpected token
    #[error("Unexpected token: {0:?}")]
    UnexpectedTok(TokenKind),
    /// Num parse error
    #[error("Num parse error: {0}")]
    NumParse(#[from] ParseIntError),
}

/// A struct field
#[derive(Debug, PartialEq)]
pub struct Field<'s> {
    /// Name of the struct field
    pub name: &'s str,
    /// Type of the struct field
    pub ty: Ty<'s>,
}

/// A type
#[derive(Debug, PartialEq)]
pub enum Ty<'s> {
    /// A type marked by an identifier
    Ident(&'s str),
    /// An array type
    Array(Array<'s>),
}

/// An array
#[derive(Debug, PartialEq)]
pub struct Array<'s> {
    /// The type of the elements
    pub ty: Box<Ty<'s>>,
    /// The length of the array
    pub len: u64,
}

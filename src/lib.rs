//! A library for parsing (a simplified form of) structs
#![warn(missing_docs, clippy::pedantic)]

mod parse;

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
    pub fn parse(mut input: &'s str) -> Result<Self, StructParseError> {
        parse::parse_struct(&mut input).map_err(StructParseError)
    }
}

/// Error that can happen while parsing a struct
pub struct StructParseError(winnow::error::ContextError);

impl std::fmt::Display for StructParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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

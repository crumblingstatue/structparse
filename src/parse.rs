use {
    crate::{
        Array, Field, Struct, StructParseError, StructParseErrorKind, Ty,
        tokenize::{Token, TokenKind},
    },
    std::num::ParseIntError,
};

#[cfg(test)]
mod tests;

trait TokIterExt {
    fn expect_tok(&mut self, tok_kind: TokenKind) -> Result<Token, StructParseError>;
    fn next_tok(&mut self) -> Result<Token, StructParseError>;
}

impl<'a, T: Iterator<Item = &'a Token>> TokIterExt for T {
    fn expect_tok(&mut self, tok_kind: TokenKind) -> Result<Token, StructParseError> {
        match self.next() {
            Some(tok) => {
                if tok_kind == tok.kind {
                    Ok(tok.clone())
                } else {
                    Err(StructParseError {
                        span: tok.span.clone(),
                        kind: StructParseErrorKind::UnexpectedTok(tok.kind),
                    })
                }
            }
            None => Err(StructParseError {
                span: 0..0,
                kind: StructParseErrorKind::UnexpectedEnd,
            }),
        }
    }

    fn next_tok(&mut self) -> Result<Token, StructParseError> {
        match self.next() {
            Some(tok) => Ok(tok.clone()),
            None => Err(StructParseError {
                span: 0..0,
                kind: StructParseErrorKind::UnexpectedEnd,
            }),
        }
    }
}

pub fn parse_struct<'a>(src: &'a str, tokens: &[Token]) -> Result<Struct<'a>, StructParseError> {
    let mut toks = tokens.iter();
    toks.expect_tok(TokenKind::KwStruct)?;
    let name_tok = toks.expect_tok(TokenKind::Ident)?;
    toks.expect_tok(TokenKind::LBrace)?;
    let mut struct_ = Struct {
        name: &src[name_tok.span.clone()],
        fields: Vec::new(),
    };
    while let Some(field) = parse_field(src, &mut toks)? {
        struct_.fields.push(field);
    }
    Ok(struct_)
}

fn parse_field<'a, 'tok>(
    src: &'a str,
    tokens: &mut impl Iterator<Item = &'tok Token>,
) -> Result<Option<Field<'a>>, StructParseError> {
    let tok = tokens.next_tok()?;
    match tok.kind {
        TokenKind::Ident => {
            let name = &src[tok.span.clone()];
            tokens.expect_tok(TokenKind::Colon)?;
            let ty = parse_ty(src, tokens)?;
            Ok(Some(Field { name, ty }))
        }
        TokenKind::RBrace => Ok(None),
        // Comma consumed, try parsing field again
        TokenKind::Comma => parse_field(src, tokens),
        _ => Err(StructParseError::unexpected(tok)),
    }
}

fn parse_ty<'a, 'tok>(
    src: &'a str,
    tokens: &mut impl Iterator<Item = &'tok Token>,
) -> Result<Ty<'a>, StructParseError> {
    let tok = tokens.next_tok()?;
    match tok.kind {
        TokenKind::Ident => Ok(Ty::Ident(&src[tok.span.clone()])),
        TokenKind::LSqBracket => Ok(Ty::Array(parse_array(src, tokens)?)),
        _ => Err(StructParseError::unexpected(tok)),
    }
}

fn parse_array<'a, 'tok>(
    src: &'a str,
    tokens: &mut impl Iterator<Item = &'tok Token>,
) -> Result<Array<'a>, StructParseError> {
    let ty = parse_ty(src, tokens)?;
    tokens.expect_tok(TokenKind::Semi)?;
    let len_tok = tokens.expect_tok(TokenKind::NumLit)?;
    let len: u64 =
        src[len_tok.span.clone()].parse().map_err(|e: ParseIntError| StructParseError {
            span: len_tok.span.clone(),
            kind: e.into(),
        })?;
    tokens.expect_tok(TokenKind::RSqBracket)?;
    Ok(Array {
        ty: Box::new(ty),
        len,
    })
}

use winnow::{
    ascii::{digit1, multispace0},
    combinator::{alt, opt, separated, seq},
    error::{ContextError, ErrMode, ErrorKind, ParserError, StrContext, StrContextValue},
    token::take_while,
    PResult, Parser,
};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Struct<'s> {
    name: &'s str,
    fields: Vec<Field<'s>>,
}

#[derive(Debug, PartialEq)]
struct Field<'s> {
    name: &'s str,
    ty: Ty<'s>,
}

#[derive(Debug, PartialEq)]
enum Ty<'s> {
    Ident(&'s str),
    Array(Array<'s>),
}
fn alpha_or_underscore<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., |ch: char| ch.is_alphanumeric() || ch == '_').parse_next(input)
}

fn identifier<'s>(input: &mut &'s str) -> PResult<&'s str> {
    multispace0.parse_next(input)?;
    let ident = alpha_or_underscore.parse_next(input)?;
    multispace0.parse_next(input)?;
    Ok(ident)
}

#[derive(Debug, PartialEq)]
struct Array<'s> {
    ty: Box<Ty<'s>>,
    len: u64,
}

fn int(input: &mut &str) -> PResult<u64> {
    multispace0.parse_next(input)?;
    let num_str = digit1.parse_next(input)?;
    let num: u64 = num_str
        .parse()
        .map_err(|_e| ErrMode::from_error_kind(input, ErrorKind::Fail))?;
    Ok(num)
}

fn array<'s>(input: &mut &'s str) -> PResult<Array<'s>> {
    seq! {Array {
        _: multispace0,
        _: '['.context(StrContext::Expected(StrContextValue::CharLiteral('['))),
        _: multispace0,
        ty: ty.context(StrContext::Expected(StrContextValue::Description("type"))).map(Box::new),
        _: multispace0,
        _: ';'.context(StrContext::Expected(StrContextValue::CharLiteral(';'))),
        _: multispace0,
        len: int,
        _: multispace0,
        _: ']'.context(StrContext::Expected(StrContextValue::CharLiteral(']'))),,
    }}
    .parse_next(input)
}

fn ty<'s>(input: &mut &'s str) -> PResult<Ty<'s>> {
    alt((identifier.map(Ty::Ident), array.map(Ty::Array))).parse_next(input)
}

fn field<'s>(input: &mut &'s str) -> PResult<Field<'s>> {
    let name = identifier
        .context(StrContext::Expected(StrContextValue::Description(
            "field name",
        )))
        .parse_next(input)?;
    ":".parse_next(input)?;
    let ty = ty
        .context(StrContext::Expected(StrContextValue::Description("type")))
        .parse_next(input)?;
    Ok(Field { name, ty })
}

fn field_sep(input: &mut &str) -> PResult<()> {
    multispace0.parse_next(input)?;
    ','.parse_next(input)?;
    multispace0.parse_next(input)?;
    Ok(())
}

fn fields<'s>(input: &mut &'s str) -> PResult<Vec<Field<'s>>> {
    let fields = separated(0.., field, field_sep).parse_next(input)?;
    opt(field_sep).parse_next(input)?;
    Ok(fields)
}

pub fn parse_struct<'s>(input: &mut &'s str) -> Result<Struct<'s>, ContextError> {
    seq! {Struct {
        _: multispace0,
        _: "struct".context(StrContext::Expected(StrContextValue::StringLiteral("struct"))),
        name: identifier.context(StrContext::Expected(StrContextValue::Description("identifier"))),
        _: '{'.context(StrContext::Expected(StrContextValue::CharLiteral('{'))),
        _: multispace0,
        fields: fields,
        _: multispace0,
        _: '}'.context(StrContext::Expected(StrContextValue::CharLiteral('}'))),
    }}
    .parse_next(input)
    .map_err(|e| e.into_inner().unwrap())
}

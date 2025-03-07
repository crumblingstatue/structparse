#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    span: std::ops::Range<usize>,
    kind: TokenKind,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum TokenKind {
    KwStruct,
    Ident,
    LBrace,
    RBrace,
    Colon,
    Comma,
}

enum Status {
    Init,
    InToken { start: usize, kind: TokenKind },
    FwSlash,
    InComment,
}

#[derive(Debug)]
struct TokenizeError {
    span: std::ops::Range<usize>,
    kind: TokenizeErrorKind,
}

#[derive(Debug)]
enum TokenizeErrorKind {
    UnexpectedByte,
}

#[expect(clippy::range_plus_one, reason = "API dictates (exclusive) Range")]
pub(crate) fn tokenize(src: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut status = Status::Init;
    let mut tokens = Vec::new();
    for (i, b) in src.bytes().enumerate() {
        loop {
            match status {
                Status::Init => {
                    match b {
                        b'{' => tokens.push(Token {
                            span: i..i + 1,
                            kind: TokenKind::LBrace,
                        }),
                        b'}' => tokens.push(Token {
                            span: i..i + 1,
                            kind: TokenKind::RBrace,
                        }),
                        b':' => tokens.push(Token {
                            span: i..i + 1,
                            kind: TokenKind::Colon,
                        }),
                        b',' => tokens.push(Token {
                            span: i..i + 1,
                            kind: TokenKind::Comma,
                        }),
                        b'/' => {
                            status = Status::FwSlash;
                        }
                        b'A'..=b'Z' | b'a'..=b'z' | b'_' => {
                            status = Status::InToken {
                                start: i,
                                kind: TokenKind::Ident,
                            }
                        }
                        _ => {}
                    }
                    break;
                }
                Status::InToken { start, kind } => match b {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {
                        break;
                    }
                    _ => {
                        let word = &src[start..i];
                        dbg!(word);
                        let kind = match word {
                            "struct" => TokenKind::KwStruct,
                            _ => TokenKind::Ident,
                        };
                        tokens.push(Token {
                            span: start..i,
                            kind,
                        });
                        status = Status::Init;
                    }
                },
                Status::FwSlash => match b {
                    b'/' => status = Status::InComment,
                    _ => {
                        return Err(TokenizeError {
                            span: i..i + 1,
                            kind: TokenizeErrorKind::UnexpectedByte,
                        });
                    }
                },
                Status::InComment => {
                    match b {
                        b'\n' => status = Status::Init,
                        _ => {}
                    }
                    break;
                }
            }
        }
    }
    Ok(tokens)
}

#[test]
fn test_tokenize_simple_empty() {
    assert_eq!(
        tokenize("struct Foo {}").unwrap(),
        &[
            Token {
                span: 0..6,
                kind: TokenKind::KwStruct
            },
            Token {
                span: 7..10,
                kind: TokenKind::Ident
            },
            Token {
                span: 11..12,
                kind: TokenKind::LBrace
            },
            Token {
                span: 12..13,
                kind: TokenKind::RBrace
            }
        ],
    );
}

#[cfg(test)]
mod tests {
    use {super::*, pretty_assertions::assert_eq};
    #[test]
    fn test_tokenize_empty_multiline() {
        assert_eq!(
            tokenize(
                "struct Foo {

        }"
            )
            .unwrap(),
            &[
                Token {
                    span: 0..6,
                    kind: TokenKind::KwStruct
                },
                Token {
                    span: 7..10,
                    kind: TokenKind::Ident
                },
                Token {
                    span: 11..12,
                    kind: TokenKind::LBrace
                },
                Token {
                    span: 22..23,
                    kind: TokenKind::RBrace
                }
            ],
        );
    }

    #[test]
    fn test_tokenize_single_field_multiline() {
        assert_eq!(
            tokenize(
                "struct Foo {
            field: u32,
        }"
            )
            .unwrap(),
            &[
                Token {
                    span: 0..6,
                    kind: TokenKind::KwStruct
                },
                Token {
                    span: 7..10,
                    kind: TokenKind::Ident
                },
                Token {
                    span: 11..12,
                    kind: TokenKind::LBrace
                },
                Token {
                    span: 25..30,
                    kind: TokenKind::Ident,
                },
                Token {
                    span: 30..31,
                    kind: TokenKind::Colon,
                },
                Token {
                    span: 32..35,
                    kind: TokenKind::Ident,
                },
                Token {
                    span: 35..36,
                    kind: TokenKind::Comma,
                },
                Token {
                    span: 45..46,
                    kind: TokenKind::RBrace
                }
            ],
        );
    }
    #[test]
    fn test_tokenize_single_field_multiline_comment() {
        assert_eq!(
            tokenize(
                "struct Foo {
            // This is a cool field... I guess.
            field: u32,
        }"
            )
            .unwrap(),
            &[
                Token {
                    span: 0..6,
                    kind: TokenKind::KwStruct
                },
                Token {
                    span: 7..10,
                    kind: TokenKind::Ident
                },
                Token {
                    span: 11..12,
                    kind: TokenKind::LBrace
                },
                Token {
                    span: 73..78,
                    kind: TokenKind::Ident,
                },
                Token {
                    span: 78..79,
                    kind: TokenKind::Colon,
                },
                Token {
                    span: 80..83,
                    kind: TokenKind::Ident,
                },
                Token {
                    span: 83..84,
                    kind: TokenKind::Comma,
                },
                Token {
                    span: 93..94,
                    kind: TokenKind::RBrace
                }
            ],
        );
    }
}

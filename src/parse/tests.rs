use {
    super::{array, int},
    crate::{Array, Field, Struct, Ty},
    pretty_assertions::assert_eq,
};

#[test]
fn parse_struct_empty() {
    assert_eq!(
        super::parse_struct(&mut "struct Empty { }").unwrap(),
        Struct {
            fields: vec![],
            name: "Empty"
        }
    );
}

#[test]
fn parse_struct_empty_multiline() {
    assert_eq!(
        super::parse_struct(
            &mut "struct Foo {
        }"
        )
        .unwrap(),
        Struct {
            fields: vec![],
            name: "Foo"
        }
    );
}

#[test]
fn parse_struct_with_single_field() {
    assert_eq!(
        super::parse_struct(&mut "struct Single { field: u32 }").unwrap(),
        Struct {
            fields: vec![Field {
                name: "field",
                ty: Ty::Ident("u32")
            }],
            name: "Single"
        }
    );
}

#[test]
fn parse_struct_with_single_array_field() {
    assert_eq!(
        super::parse_struct(&mut "struct HasArray { field: [u32; 10] }").unwrap(),
        Struct {
            fields: vec![Field {
                name: "field",
                ty: Ty::Array(Array {
                    ty: Box::new(Ty::Ident("u32")),
                    len: 10
                }),
            }],
            name: "HasArray"
        }
    );
}

#[test]
fn field_with_trailing_comma() {
    let mut input = "struct Foo {
    field: [u8; 10],
    }";
    assert_eq!(
        super::parse_struct(&mut input).unwrap(),
        Struct {
            name: "Foo",
            fields: vec![Field {
                name: "field",
                ty: Ty::Array(Array {
                    ty: Box::new(Ty::Ident("u8")),
                    len: 10
                })
            }]
        }
    );
}

#[test]
fn parse_struct_with_multi_array_field_singleline() {
    assert_eq!(
        super::parse_struct(&mut "struct IHaveArrayFields { field: [u32; 10], field2: [u64; 32] }")
            .unwrap(),
        Struct {
            fields: vec![
                Field {
                    name: "field",
                    ty: Ty::Array(Array {
                        ty: Box::new(Ty::Ident("u32")),
                        len: 10
                    }),
                },
                Field {
                    name: "field2",
                    ty: Ty::Array(Array {
                        ty: Box::new(Ty::Ident("u64")),
                        len: 32
                    }),
                }
            ],
            name: "IHaveArrayFields"
        }
    );
}

#[test]
fn parse_struct_with_multi_array_field_multiline() {
    assert_eq!(
        super::parse_struct(
            &mut "
        struct IHaveArrayFields {
            field: [u32; 10],
            field2: [u64; 32]
        }"
        )
        .unwrap(),
        Struct {
            fields: vec![
                Field {
                    name: "field",
                    ty: Ty::Array(Array {
                        ty: Box::new(Ty::Ident("u32")),
                        len: 10
                    }),
                },
                Field {
                    name: "field2",
                    ty: Ty::Array(Array {
                        ty: Box::new(Ty::Ident("u64")),
                        len: 32
                    }),
                }
            ],
            name: "IHaveArrayFields"
        }
    );
}

#[test]
fn parse_struct_with_multi_fields_singleline() {
    assert_eq!(
        super::parse_struct(&mut "struct MultiSl { field: u32 , field2: u32 }").unwrap(),
        Struct {
            fields: vec![
                Field {
                    name: "field",
                    ty: Ty::Ident("u32")
                },
                Field {
                    name: "field2",
                    ty: Ty::Ident("u32")
                }
            ],
            name: "MultiSl"
        }
    );
}

#[test]
fn parse_struct_with_multi_fields_multiline() {
    assert_eq!(
        super::parse_struct(
            &mut "struct Foo {
            field: u32,
            field2: u32
        }"
        )
        .unwrap(),
        Struct {
            fields: vec![
                Field {
                    name: "field",
                    ty: Ty::Ident("u32")
                },
                Field {
                    name: "field2",
                    ty: Ty::Ident("u32")
                }
            ],
            name: "Foo"
        }
    );
}

#[test]
fn identifier() {
    assert_eq!(super::identifier(&mut "ident").unwrap(), "ident");
    assert_eq!(
        super::identifier(&mut "     spaced_out       ").unwrap(),
        "spaced_out"
    );
    assert_eq!(
        super::identifier(&mut "_under_score_").unwrap(),
        "_under_score_"
    );
    assert_eq!(super::identifier(&mut "_").unwrap(), "_");
}

#[test]
fn test_fields_single() {
    assert_eq!(
        super::fields(&mut "foo: i32").unwrap(),
        vec![Field {
            name: "foo",
            ty: Ty::Ident("i32")
        }]
    );
}

#[test]
fn test_fields_multi_singleline() {
    assert_eq!(
        super::fields(&mut "foo: i32, bar: i32, baz: u64").unwrap(),
        vec![
            Field {
                name: "foo",
                ty: Ty::Ident("i32")
            },
            Field {
                name: "bar",
                ty: Ty::Ident("i32")
            },
            Field {
                name: "baz",
                ty: Ty::Ident("u64")
            }
        ]
    );
}

#[test]
fn test_fields_multi_multiline() {
    assert_eq!(
        super::fields(
            &mut "
        foo: i32,
        bar: i32,
        baz: u64"
        )
        .unwrap(),
        vec![
            Field {
                name: "foo",
                ty: Ty::Ident("i32")
            },
            Field {
                name: "bar",
                ty: Ty::Ident("i32")
            },
            Field {
                name: "baz",
                ty: Ty::Ident("u64")
            }
        ]
    );
}

#[test]
fn test_field_single() {
    assert_eq!(
        super::field(&mut "foo: i32").unwrap(),
        Field {
            name: "foo",
            ty: Ty::Ident("i32")
        },
    );
}

#[test]
fn test_parse_int() {
    assert_eq!(int(&mut "64"), Ok(64));
    assert_eq!(int(&mut " 2348  "), Ok(2348));
}

#[test]
fn test_parse_array() {
    assert_eq!(
        array(&mut "[u32; 10]").unwrap(),
        Array {
            ty: Box::new(Ty::Ident("u32")),
            len: 10
        }
    );
}

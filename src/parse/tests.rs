#![expect(clippy::unwrap_used)]

use {
    crate::{Array, Field, Struct, Ty},
    pretty_assertions::assert_eq,
};

#[test]
fn parse_struct_empty() {
    assert_eq!(
        Struct::parse("struct Empty { }").unwrap(),
        Struct {
            fields: vec![],
            name: "Empty"
        }
    );
}

#[test]
fn parse_struct_empty_multiline() {
    assert_eq!(
        Struct::parse(
            "struct Foo {
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
        Struct::parse("struct Single { field: u32 }").unwrap(),
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
        Struct::parse("struct HasArray { field: [u32; 10] }").unwrap(),
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
    let input = "struct Foo {
    field: [u8; 10],
    }";
    assert_eq!(
        Struct::parse(input).unwrap(),
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
        Struct::parse("struct IHaveArrayFields { field: [u32; 10], field2: [u64; 32] }").unwrap(),
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
        Struct::parse(
            "
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
        Struct::parse("struct MultiSl { field: u32 , field2: u32 }").unwrap(),
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
        Struct::parse(
            "struct Foo {
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

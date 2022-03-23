use super::*;
use expect_test::{expect, Expect};

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual)
}

#[test]
fn namespace_test() {
    check_lexing(
        "namespace com.example.foo",
        expect![[r#"
        Token { kind: Ident, len: 9 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 3 }
        Token { kind: Dot, len: 1 }
        Token { kind: Ident, len: 7 }
        Token { kind: Dot, len: 1 }
        Token { kind: Ident, len: 3 }
        "#]],
    )
}

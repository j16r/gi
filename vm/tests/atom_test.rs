mod support;
use support::verify_output;

#[test]
fn atom_test() {
    verify_output(r#"(dump (atom 1))"#, r#"true"#);
    verify_output(r#"(dump (atom "string"))"#, r#"true"#);
    verify_output(r#"(dump (atom "string"))"#, r#"true"#);

    verify_output(r#"(dump (atom (quote "string")))"#, r#"false"#);
    verify_output(r#"(dump (atom (1 2 3)))"#, r#"false"#);
}

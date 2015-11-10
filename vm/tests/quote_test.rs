mod support;
use support::verify_output;

#[test]
fn quote_test() {
    verify_output(
        r#"(dump (quote (add 1 2)))"#,
        r#"(:add, (1_i32, (2_i32, Nil)))"#);

    verify_output(
        r#"(dump (equal (quote (1 2)) (1 2)))"#,
        r#"true"#);
}

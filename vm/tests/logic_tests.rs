mod support;
use support::verify_output;

#[test]
fn test_equal_bool() {
    verify_output(r#"(dump (equal true true))"#, "true");
    verify_output(r#"(dump (equal true false))"#, "false");
}

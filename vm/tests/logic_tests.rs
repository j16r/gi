mod support;
use support::verify_output;

#[test]
fn test_equal_bool() {
    verify_output(r#"(dump (equal true true))"#, "true");
    verify_output(r#"(dump (equal true false))"#, "false");
    verify_output(r#"(dump (equal 0 0))"#, "true");
    verify_output(r#"(dump (equal 0 1))"#, "false");
    verify_output(r#"(dump (equal "string" ""))"#, "false");
    verify_output(r#"(dump (equal "true" "true"))"#, "true");
}

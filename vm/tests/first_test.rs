mod support;
use support::verify_output;

#[test]
fn first() {
    verify_output(r#"(dump (first "first" "second" "third"))"#, r#""first""#);
}

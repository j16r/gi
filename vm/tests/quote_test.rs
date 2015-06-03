mod support;
use support::verify_output;

#[test]
fn quote_test() {
    verify_output(
        r#"(dump (quote (add 1 2)))"#,
        r#"Cons(:add, Cons(1_i32, Cons(2_i32, Nil)))"#);
}

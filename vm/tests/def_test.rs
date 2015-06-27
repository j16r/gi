mod support;
use support::verify_output;

#[test]
fn def_test() {
    verify_output(r#"(def pi 3) (dump pi)"#, r#"Cons(3_i32, Nil)"#);
}

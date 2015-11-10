mod support;
use support::verify_output;

#[test]
fn def_test() {
    verify_output(r#"(def pi 3) (dump pi)"#, "3_i32");
}

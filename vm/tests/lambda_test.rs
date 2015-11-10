mod support;
use support::verify_output;

#[test]
fn lambda_test() {
    verify_output(
        r#"(def identity (lambda (x) x)) (dump (identity 9))"#,
        "9_i32");
    verify_output(
        r#"(def square (lambda (x) (mul x x))) (dump (square 9))"#,
        "81_i32");
}

mod support;
use support::verify_output;

#[test]
fn lambda_test() {
    verify_output(
        r#"(def identity (lambda (x) x)) (dump (identity 9))"#,
        r#"Cons(9_i32, Nil)"#);
    verify_output(
        r#"(def square (lambda (x) (mul x x))) (dump (square 9))"#,
        r#"Cons(81_i32, Nil)"#);
}

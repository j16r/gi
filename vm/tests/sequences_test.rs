mod support;
use support::verify_output;

#[test]
fn cons_test() {
    verify_output(
        r#"(dump (cons (quote 1) (quote 2)))"#,
        r#"(1_i32, (2_i32, Nil))"#);
}

#[test]
fn cons_nested_test() {
    verify_output(
        r#"(dump (cons (quote 1) (quote (2 3))))"#,
        r#"(1_i32, ((2_i32, (3_i32, Nil)), Nil))"#);
}

#[test]
fn cons_nested_multiple_in_rhs_cons_cell_test() {
    verify_output(
        r#"(dump (cons (quote 1) (quote (2 3)) (quote 4)))"#,
        r#"(1_i32, ((2_i32, (3_i32, Nil)), (4_i32, Nil)))"#);
}

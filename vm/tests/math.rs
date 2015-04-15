mod support;
use support::verify_output;

#[test]
fn add() {
    verify_output("(dump (add 1 2))", "3_i32");
}

#[test]
fn mul() {
    verify_output("(dump (mul 7 6))", "42_i32");
}

#[test]
fn div() {
    verify_output("(dump (div 30 3))", "10_i32");
}

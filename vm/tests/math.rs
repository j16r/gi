mod support;
use support::verify_output;

#[test]
fn add() {
    verify_output("tests/add_two_small_ints.gi", "3_i32");
}

#[test]
fn mul() {
    verify_output("tests/mul_two_small_ints.gi", "42_i32");
}

#[test]
fn div() {
    verify_output("tests/div_two_small_ints.gi", "10_i32");
}

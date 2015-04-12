mod support;
use support::verify_output;

#[test]
fn cond_true() {
    verify_output("tests/cond_true.gi", "\"statement\"");
}

#[test]
fn cond_false() {
    verify_output("tests/cond_false.gi", "Nil");
}

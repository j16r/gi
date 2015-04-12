mod support;
use support::verify_output;

#[test]
fn first() {
    verify_output("tests/first.gi", "\"first\"");
}

mod support;
use support::verify_output;

#[test]
fn cond_true() {
    verify_output(r#"dump(cond(true "statement"))"#, r#""statement""#);
}

#[test]
fn cond_false() {
    verify_output(r#"dump(cond(false "statement"))"#, "");
}

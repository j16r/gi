mod support;
use support::verify_output;

#[test]
fn atom_test_integer() {
    verify_output(r#"dump(atom(1))"#, r#"true"#);
}

#[test]
fn atom_test_string() {
    verify_output(r#"dump(atom("string"))"#, r#"true"#);
}

//#[test]
//fn atom_test_list() {
    // FIXME: What had I intended here, an array literal?
    // to do atom of 3 integers?
    // there isn't yet an array / list literal, but this parses...
    //verify_output(r#"dump(atom((1 2 3)))"#, r#"false"#);
//}

#[test]
fn atom_test_quoted_string() {
    verify_output(r#"dump(atom(quote("string")))"#, r#"false"#);
}

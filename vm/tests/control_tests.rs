use std::process::Command;

#[test]
fn cond_true() {
    let stdout = Command::new("target/debug/gi_vm")
        .args(&["run", "tests/cond_true.gi"])
        .output()
        .unwrap()
        .stdout;

    assert_eq!(String::from_utf8_lossy(&stdout), "\"statement\"");
}

#[test]
fn cond_false() {
    let stdout = Command::new("target/debug/gi_vm")
        .args(&["run", "tests/cond_false.gi"])
        .output()
        .unwrap()
        .stdout;

    assert_eq!(String::from_utf8_lossy(&stdout), "Nil");
}

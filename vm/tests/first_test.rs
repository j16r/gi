use std::process::Command;

#[test]
fn first() {
    let stdout = Command::new("target/debug/gi_vm")
        .args(&["run", "tests/first.gi"])
        .output()
        .unwrap()
        .stdout;

    assert_eq!(String::from_utf8_lossy(&stdout), "\"first\"");
}

use std::process::Command;

#[test]
fn add() {
    let stdout = Command::new("target/gi_vm")
        .args(&["run", "tests/add_test.gi"])
        .output()
        .unwrap()
        .stdout;

    assert_eq!(String::from_utf8_lossy(&stdout), "3\n");
}

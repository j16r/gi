use std::process::Command;

#[test]
fn add() {
    let stdout = Command::new("target/gi_vm")
        .args(&["run", "tests/add_two_small_ints.gi"])
        .output()
        .unwrap()
        .stdout;

    assert_eq!(String::from_utf8_lossy(&stdout), "3");
}

#[test]
fn mul() {
    let stdout = Command::new("target/gi_vm")
        .args(&["run", "tests/mul_two_small_ints.gi"])
        .output()
        .unwrap()
        .stdout;

    assert_eq!(String::from_utf8_lossy(&stdout), "42");
}

#[test]
fn div() {
    let stdout = Command::new("target/gi_vm")
        .args(&["run", "tests/div_two_small_ints.gi"])
        .output()
        .unwrap()
        .stdout;

    assert_eq!(String::from_utf8_lossy(&stdout), "10");
}

use std::io::Write;
use std::process::{Command, Output, Stdio};

pub fn verify_output(program : &str, expected : &str) {
    let mut cmd = Command::new("target/debug/gi_vm");
    let program_output = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(mut process) => {
            process.stdin.as_mut().unwrap_or_else(|| {
                panic!("Couldn't get stdin mut handle")
            }).write_all(program.as_bytes()).unwrap_or_else(|e| {
                panic!("Couldn't write to stdin, error: {}", e)
            });

            match process.wait_with_output() {
                Ok(Output { stdout, .. }) => {
                    match String::from_utf8(stdout) {
                        Ok(output) => output,
                        _ => panic!("Couldn't convert output")
                    }
                },
                _ => panic!("Execution of gi resulted in error")
            }
        },
        _ => panic!("Error executing gi")
    };
    
    assert_eq!(&program_output, expected);
}

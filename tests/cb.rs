use assert_cmd::Command;

#[test]
fn cb_version() {
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let output = cmd
        .arg("--version")
        .output()
        .expect("failed to execute cb --version");

    let output_string = String::from_utf8(output.stdout).unwrap();
    assert!(output_string.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cb_copy_and_paste() {
    // Input from argument
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let argument_data = "I'm from argument.";

    cmd.args(["copy", argument_data])
        .output()
        .expect("failed to execute cb copy from argument");

    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("paste").assert().success().stdout(argument_data);

    // Input from stdin
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let stdin_data = "I'm from stdin.";

    cmd.arg("copy")
        .write_stdin(stdin_data)
        .output()
        .expect("failed to execute cb copy from stdin");

    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("paste").assert().success().stdout(stdin_data);
}
